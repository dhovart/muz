use super::spectrum::SpectrumAnalyzer;
use anyhow::Result;
use awedio::{sounds::wrappers::Wrapper, NextSample, Sound};
use std::sync::{Arc, Mutex};

pub struct ProgressUpdate<S: Sound> {
    inner: S,
    total_frames: u64,
    samples_played: u64,
    percent_completed: f64,
    on_update: Box<dyn Fn(f64, u64, Vec<f32>) + Send>,
    spectrum_analyzer: Arc<Mutex<SpectrumAnalyzer>>,
    sample_buffer: Vec<f32>,
    batch_size: usize,
    cached_spectrum: Vec<f32>,
}

impl<S: Sound> ProgressUpdate<S> {
    pub fn new(
        inner: S,
        total_frames: u64,
        on_update: Box<dyn Fn(f64, u64, Vec<f32>) + Send>,
    ) -> Self {
        let sample_rate = inner.sample_rate() as f32;
        let spectrum_analyzer = Arc::new(Mutex::new(SpectrumAnalyzer::new(2048, sample_rate)));

        ProgressUpdate {
            inner,
            total_frames,
            samples_played: 0,
            percent_completed: 0.0,
            on_update,
            spectrum_analyzer,
            sample_buffer: Vec::new(),
            batch_size: 512,
            cached_spectrum: Vec::new(),
        }
    }
}

impl<S> Wrapper for ProgressUpdate<S>
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

impl<S: Sound> Sound for ProgressUpdate<S> {
    fn next_sample(&mut self) -> Result<NextSample, awedio::Error> {
        match self.inner.next_sample() {
            Ok(sample) => {
                self.samples_played += 1;

                // Convert sample to f32 and add to buffer
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
                    (self.samples_played as f64 / total_samples as f64) * 100.0
                } else {
                    0.0
                };
                let percent_completed = (percent_completed * 10.0).round() / 10.0;

                if self.percent_completed != percent_completed {
                    if self.sample_buffer.len() >= self.batch_size {
                        if let Ok(mut analyzer) = self.spectrum_analyzer.lock() {
                            analyzer.add_samples(&self.sample_buffer);
                            self.cached_spectrum = analyzer.get_spectrum();
                        }
                        self.sample_buffer.clear();
                    }

                    self.percent_completed = percent_completed;
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
