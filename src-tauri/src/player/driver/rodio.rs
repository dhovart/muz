pub mod rodio_impl {
    use anyhow::{anyhow, Result};
    use rodio::{Decoder, OutputStream, Sink, Source};
    use std::io::BufReader;
    use std::sync::mpsc::Sender;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};

    use crate::player::{
        driver::PlaybackDriver, playback::PlaybackEvent, spectrum::SpectrumAnalyzer, track::Track,
    };

    enum AudioCommand {
        Play(Track, Sender<PlaybackEvent>),
        Pause,
        Resume,
        Clear,
        SetVolume(f32),
        Seek(Duration),
        Exit,
    }

    pub struct RodioPlaybackDriver {
        command_sender: Sender<AudioCommand>,
    }

    impl RodioPlaybackDriver {
        pub fn new(volume: f32) -> Result<Self> {
            use std::sync::mpsc;
            let (command_sender, command_receiver) = mpsc::channel();

            // NOTE: Cpal Backend is not Send, using a dedicated thread as a workaround
            thread::spawn(move || {
                let (_stream, stream_handle) =
                    OutputStream::try_default().expect("Failed to create audio output stream");
                let mut sink: Option<Sink> = None;
                let mut volume = volume.clamp(0.0, 1.0);

                while let Ok(cmd) = command_receiver.recv() {
                    match cmd {
                        AudioCommand::Play(track, progress_sender) => {
                            if let Some(old_sink) = sink.take() {
                                old_sink.stop();
                            }
                            let sink_new =
                                Sink::try_new(&stream_handle).expect("Failed to create sink");
                            sink_new.set_volume(volume);
                            let file = std::fs::File::open(&track.path)
                                .expect("Failed to open audio file");
                            let source = Decoder::new(BufReader::new(file))
                                .expect("Failed to decode audio file");
                            let progress_source = ProgressAndSpectrumSource::new(
                                source,
                                track.total_frames,
                                0,
                                progress_sender.clone(),
                            );
                            sink_new.append(progress_source);
                            sink = Some(sink_new);
                            let completion_sender = progress_sender;
                            let sink_len = sink.as_ref().unwrap().len();
                            thread::spawn(move || loop {
                                if sink_len == 0 {
                                    let _ = completion_sender.send(PlaybackEvent::TrackCompleted);
                                    break;
                                }
                                thread::sleep(Duration::from_millis(100));
                            });
                        }
                        AudioCommand::Pause => {
                            if let Some(ref s) = sink {
                                s.pause();
                            }
                        }
                        AudioCommand::Resume => {
                            if let Some(ref s) = sink {
                                s.play();
                            }
                        }
                        AudioCommand::Clear => {
                            if let Some(old_sink) = sink.take() {
                                old_sink.stop();
                            }
                        }
                        AudioCommand::SetVolume(vol) => {
                            volume = vol.clamp(0.0, 1.0);
                            if let Some(ref s) = sink {
                                s.set_volume(volume);
                            }
                        }
                        AudioCommand::Seek(position) => {
                            if let Some(ref s) = sink {
                                match s.try_seek(position) {
                                    Ok(_) => {
                                        println!("Successfully seeked to {:?}", position);
                                    }
                                    Err(e) => {
                                        println!("Failed to seek: {:?}", e);
                                    }
                                }
                            }
                        }
                        AudioCommand::Exit => break,
                    }
                }
            });
            Ok(Self { command_sender })
        }
    }

    impl PlaybackDriver for RodioPlaybackDriver {
        fn play(&mut self, track: Track, progress_sender: Sender<PlaybackEvent>) -> Result<()> {
            self.command_sender
                .send(AudioCommand::Play(track, progress_sender))
                .map_err(|e| anyhow!("Failed to send play command: {}", e))
        }

        fn pause(&mut self) -> Result<()> {
            self.command_sender
                .send(AudioCommand::Pause)
                .map_err(|e| anyhow!("Failed to send pause command: {}", e))
        }

        fn resume(&mut self) -> Result<()> {
            self.command_sender
                .send(AudioCommand::Resume)
                .map_err(|e| anyhow!("Failed to send resume command: {}", e))
        }

        fn clear(&mut self) -> Result<()> {
            self.command_sender
                .send(AudioCommand::Clear)
                .map_err(|e| anyhow!("Failed to send clear command: {}", e))
        }

        fn set_volume(&mut self, volume: f32) -> Result<()> {
            self.command_sender
                .send(AudioCommand::SetVolume(volume))
                .map_err(|e| anyhow!("Failed to send volume command: {}", e))
        }

        fn seek(&mut self, position: Duration) -> Result<()> {
            self.command_sender
                .send(AudioCommand::Seek(position))
                .map_err(|e| anyhow!("Failed to send seek command: {}", e))
        }
    }
    impl Drop for RodioPlaybackDriver {
        fn drop(&mut self) {
            let _ = self.command_sender.send(AudioCommand::Exit);
        }
    }

    struct ProgressAndSpectrumSource<S: Source<Item = i16>> {
        inner: S,
        total_frames: u64,
        samples_played: u64,
        playback_sender: Sender<PlaybackEvent>,
        spectrum_analyzer: Arc<Mutex<SpectrumAnalyzer>>,
        sample_buffer: Vec<f32>,
        batch_size: usize,
        cached_spectrum: Vec<f32>,
        last_update_time: Instant,
        sample_rate: u32,
        channels: u16,
    }

    impl<S: Source<Item = i16>> ProgressAndSpectrumSource<S> {
        fn new(
            inner: S,
            total_frames: u64,
            samples_offset: u64,
            playback_sender: Sender<PlaybackEvent>,
        ) -> Self {
            let sample_rate = inner.sample_rate();
            let channels = inner.channels();
            let spectrum_analyzer =
                Arc::new(Mutex::new(SpectrumAnalyzer::new(4096, sample_rate as f32)));

            Self {
                inner,
                total_frames,
                samples_played: samples_offset,
                playback_sender,
                spectrum_analyzer,
                sample_buffer: Vec::new(),
                batch_size: 512,
                cached_spectrum: Vec::new(),
                last_update_time: Instant::now(),
                sample_rate,
                channels,
            }
        }
    }

    impl<S: Source<Item = i16>> Iterator for ProgressAndSpectrumSource<S> {
        type Item = i16;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(sample) = self.inner.next() {
                self.samples_played += 1;

                let total_samples = self.total_frames * self.channels as u64;
                let frames_played = self.samples_played / self.channels as u64;
                let percent_completed = if total_samples > 0 {
                    self.samples_played as f64 / total_samples as f64
                } else {
                    0.0
                };
                let percent_completed = (percent_completed * 1000.0).round() / 1000.0;

                // Convert i16 to f32 for spectrum analysis
                let sample_f32 = sample as f32 / i16::MAX as f32;
                self.sample_buffer.push(sample_f32);

                if self.sample_buffer.len() >= self.batch_size {
                    if let Ok(mut analyzer) = self.spectrum_analyzer.lock() {
                        analyzer.add_samples(&self.sample_buffer);
                        self.cached_spectrum = analyzer.get_spectrum();
                    }
                    self.sample_buffer.clear();
                }

                let now = Instant::now();
                let should_update = now.duration_since(self.last_update_time).as_millis() >= 100; // 10 FPS

                if should_update {
                    self.last_update_time = now;
                    let _ = self
                        .playback_sender
                        .send(PlaybackEvent::Progress(percent_completed, frames_played));
                    let spectrum_data = self.cached_spectrum.clone();
                    let _ = self
                        .playback_sender
                        .send(PlaybackEvent::Spectrum(spectrum_data));
                }

                Some(sample)
            } else {
                None
            }
        }
    }

    impl<S: Source<Item = i16>> Source for ProgressAndSpectrumSource<S> {
        fn current_frame_len(&self) -> Option<usize> {
            self.inner.current_frame_len()
        }

        fn channels(&self) -> u16 {
            self.channels
        }

        fn sample_rate(&self) -> u32 {
            self.sample_rate
        }

        fn total_duration(&self) -> Option<Duration> {
            self.inner.total_duration()
        }

        fn try_seek(&mut self, pos: Duration) -> std::result::Result<(), rodio::source::SeekError> {
            let result = self.inner.try_seek(pos);
            if result.is_ok() {
                let sample_rate = self.sample_rate as u64;
                let channels = self.channels as u64;
                self.samples_played =
                    (pos.as_secs_f64() * sample_rate as f64 * channels as f64) as u64;
            }
            result
        }
    }
}

pub use rodio_impl::*;
