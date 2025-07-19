#[cfg(feature = "awedio-driver")]
pub mod awedio_impl {
    use crate::player::{driver::PlaybackDriver, playback::PlaybackEvent, track::Track};
    use anyhow::{anyhow, Error, Result};
    use awedio::{
        manager::Manager,
        sounds::wrappers::{
            AdjustableVolume, CompletionNotifier, Controllable, Controller, Pausable,
        },
        Sound,
    };
    use sound::WithProgressAndSpectrum;

    use std::thread;
    use std::{
        sync::mpsc::{self, Sender},
        time::Duration,
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

    pub struct AwedioPlaybackDriver {
        command_sender: Sender<AudioCommand>,
    }

    type UnControlledAppSound =
        WithProgressAndSpectrum<CompletionNotifier<Pausable<AdjustableVolume<Box<dyn Sound>>>>>;
    type SoundController = Controller<UnControlledAppSound>;
    pub type AppSound = Controllable<UnControlledAppSound>;

    impl AwedioPlaybackDriver {
        pub fn new(volume: f32) -> Result<Self> {
            let (command_sender, command_receiver) = mpsc::channel();

            // NOTE: CpalBackend is not Send, using a dedicated thread as a workaround
            thread::spawn(move || {
                let (mut manager, backend) =
                    awedio::start().expect("Failed to start audio manager");
                let mut controller: Option<SoundController> = None;
                let mut volume = volume.clamp(0.0, 1.0);
                let mut current_track: Option<Track> = None;
                let mut current_playback_sender: Option<Sender<PlaybackEvent>> = None;
                let mut current_position_ms: u64 = 0;
                let mut _current_sound: Option<Box<dyn Sound>> = None;

                let start_playback = |track: &Track,
                                      playback_sender: &Sender<PlaybackEvent>,
                                      seek_duration: Option<Duration>,
                                      manager: &mut Manager,
                                      controller: &mut Option<SoundController>,
                                      volume: f32|
                 -> Result<(), Error> {
                    let progress_sender = playback_sender.clone();

                    let sound =
                        sound::create_seeked_sound(track.path.to_str().unwrap(), seek_duration)?;

                    let sound = sound.with_adjustable_volume_of(volume).pausable();
                    let (sound, notifier) = sound.with_completion_notifier();

                    let samples_played = if let Some(seek) = seek_duration {
                        let sample_rate = sound.sample_rate() as u64;
                        let channels = sound.channel_count() as u64;
                        let seek_samples =
                            seek.as_secs_f64() * sample_rate as f64 * channels as f64;
                        seek_samples.round() as u64
                    } else {
                        0
                    };

                    let sound = WithProgressAndSpectrum::new(
                        sound,
                        track.total_frames,
                        samples_played,
                        Box::new(move |progress, frames_played, spectrum_data| {
                            progress_sender
                                .send(PlaybackEvent::Progress(
                                    progress,
                                    frames_played,
                                    spectrum_data,
                                ))
                                .unwrap();
                        }),
                    );

                    let (sound, ctrl) = sound.controllable();

                    *controller = Some(ctrl);

                    let completion_sender = playback_sender.clone();
                    thread::spawn(move || {
                        if notifier.recv().is_ok() {
                            let _ = completion_sender.send(PlaybackEvent::TrackCompleted);
                        }
                    });

                    manager.play(Box::new(sound));
                    Ok(())
                };

                while let Ok(cmd) = command_receiver.recv() {
                    match cmd {
                        AudioCommand::Play(track, playback_sender) => {
                            current_track = Some(track.clone());
                            current_playback_sender = Some(playback_sender.clone());

                            // Reset position
                            current_position_ms = 0;

                            start_playback(
                                &track,
                                &playback_sender,
                                None,
                                &mut manager,
                                &mut controller,
                                volume,
                            )
                            .ok();
                        }
                        AudioCommand::Pause => {
                            if let Some(ctrl) = controller.as_mut() {
                                ctrl.set_paused(true);
                            }
                        }
                        AudioCommand::Resume => {
                            if let Some(ctrl) = controller.as_mut() {
                                ctrl.set_paused(false);
                            }
                        }
                        AudioCommand::SetVolume(vol) => {
                            volume = vol.clamp(0.0, 1.0);
                            if let Some(ctrl) = controller.as_mut() {
                                ctrl.set_volume(volume);
                            }
                        }
                        AudioCommand::Seek(duration) => {
                            if let (Some(track), Some(sender)) =
                                (current_track.clone(), current_playback_sender.clone())
                            {
                                let target_position_ms = duration.as_millis() as u64;
                                let _current_position = Duration::from_millis(current_position_ms);

                                println!(
                                "Seeking to {target_position_ms}ms with optimized chunked skipping (restart)",
                            );

                                // Clear current playback and restart with seek
                                controller = None;
                                _current_sound = None;
                                manager.clear();

                                // Update position tracker
                                current_position_ms = target_position_ms;

                                // Restart playback with optimized seek
                                start_playback(
                                    &track,
                                    &sender,
                                    Some(duration),
                                    &mut manager,
                                    &mut controller,
                                    volume,
                                )
                                .ok();
                            }
                        }
                        AudioCommand::Clear => {
                            controller = None;
                            _current_sound = None;
                            manager.clear();
                        }
                        AudioCommand::Exit => break,
                    }
                }
                drop(backend);
            });

            Ok(Self { command_sender })
        }
    }

    impl PlaybackDriver for AwedioPlaybackDriver {
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

    impl Drop for AwedioPlaybackDriver {
        fn drop(&mut self) {
            let _ = self.command_sender.send(AudioCommand::Exit);
        }
    }

    mod sound {
        use crate::player::spectrum::SpectrumAnalyzer;
        use anyhow::Result;
        use awedio::{sounds::wrappers::Wrapper, NextSample, Sound};
        use std::{
            sync::{Arc, Mutex},
            time::Duration,
        };

        const REFRESH_RATE: u128 = 100;

        pub struct WithProgressAndSpectrum<S: Sound> {
            inner: S,
            total_frames: u64,
            samples_played: u64,
            on_update: Box<dyn Fn(f64, u64, Vec<f32>) + Send>,
            spectrum_analyzer: Arc<Mutex<SpectrumAnalyzer>>,
            sample_buffer: Vec<f32>,
            batch_size: usize,
            cached_spectrum: Vec<f32>,
            last_update_time: std::time::Instant,
        }

        impl<S: Sound> WithProgressAndSpectrum<S> {
            pub fn new(
                inner: S,
                total_frames: u64,
                samples_offset: u64,
                on_update: Box<dyn Fn(f64, u64, Vec<f32>) + Send>,
            ) -> Self {
                let sample_rate = inner.sample_rate() as f32;
                let spectrum_analyzer =
                    Arc::new(Mutex::new(SpectrumAnalyzer::new(4096, sample_rate)));

                WithProgressAndSpectrum {
                    inner,
                    total_frames,
                    samples_played: samples_offset,
                    on_update,
                    spectrum_analyzer,
                    sample_buffer: Vec::new(),
                    batch_size: 512,
                    cached_spectrum: Vec::new(),
                    last_update_time: std::time::Instant::now(),
                }
            }
        }

        impl<S> Wrapper for WithProgressAndSpectrum<S>
        where
            S: Sound,
        {
            type Inner = S;

            fn inner(&self) -> &S {
                &self.inner
            }

            fn inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.inner
            }

            fn into_inner(self) -> S {
                self.inner
            }
        }

        impl<S: Sound> Sound for WithProgressAndSpectrum<S> {
            fn next_sample(&mut self) -> Result<NextSample, awedio::Error> {
                match self.inner.next_sample() {
                    Ok(sample) => {
                        self.samples_played += 1;

                        let sample_f32 = match sample {
                            NextSample::Sample(s) => s as f32 / i16::MAX as f32,
                            NextSample::Paused => 0.0,
                            NextSample::Finished => 0.0,
                            NextSample::MetadataChanged => 0.0,
                        };

                        self.sample_buffer.push(sample_f32);

                        let total_samples = self.total_frames * self.inner.channel_count() as u64;
                        let frames_played = self.samples_played / self.inner.channel_count() as u64;
                        let percent_completed = if total_samples > 0 {
                            self.samples_played as f64 / total_samples as f64
                        } else {
                            0.0
                        };
                        let percent_completed = (percent_completed * 1000.0).round() / 1000.0;

                        if self.sample_buffer.len() >= self.batch_size {
                            if let Ok(mut analyzer) = self.spectrum_analyzer.lock() {
                                analyzer.add_samples(&self.sample_buffer);
                                self.cached_spectrum = analyzer.get_spectrum();
                            }
                            self.sample_buffer.clear();
                        }

                        let now = std::time::Instant::now();
                        let should_update =
                            now.duration_since(self.last_update_time).as_millis() >= REFRESH_RATE;

                        if should_update {
                            self.last_update_time = now;
                            (self.on_update)(
                                percent_completed,
                                frames_played,
                                self.cached_spectrum.clone(),
                            );
                        }

                        Ok(sample)
                    }
                    Err(err) => Err(err),
                }
            }

            fn sample_rate(&self) -> u32 {
                self.inner.sample_rate()
            }

            fn channel_count(&self) -> u16 {
                self.inner.channel_count()
            }

            fn on_start_of_batch(&mut self) {
                self.inner.on_start_of_batch();
            }
        }

        pub fn create_seeked_sound(
            path: &str,
            seek_duration: Option<std::time::Duration>,
        ) -> Result<Box<dyn Sound>> {
            let mut sound = awedio::sounds::open_file(path)?;
            if let Some(seek) = seek_duration {
                sound.skip(seek)?;
            }
            Ok(Box::new(sound))
        }
    }
}

#[cfg(feature = "awedio-driver")]
pub use awedio_impl::*;
