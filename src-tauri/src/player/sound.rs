use anyhow::Result;
use awedio::{sounds::wrappers::Wrapper, NextSample, Sound};

pub struct ProgressUpdate<S: Sound> {
    inner: S,
    total_frames: u64,
    samples_played: u64,
    on_update: Box<dyn Fn(f64) + Send>,
}

impl<S: Sound> ProgressUpdate<S> {
    pub fn new(inner: S, total_frames: u64, on_update: Box<dyn Fn(f64) + Send>) -> Self {
        ProgressUpdate {
            inner,
            total_frames,
            samples_played: 0,
            on_update,
        }
    }

    pub fn played_frames(&self) -> u64 {
        self.samples_played / self.inner.channel_count() as u64
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

                let percent_completed = if self.total_frames > 0 {
                    (self.played_frames() as f64 / self.total_frames as f64) * 100.0
                } else {
                    0.0
                };

                (self.on_update)(percent_completed);

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
