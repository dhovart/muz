use anyhow::Result;
use awedio::{sounds::wrappers::Wrapper, NextSample, Sound};

pub struct CompletionTracking<S: Sound> {
    inner: S,
    total_frames: u64,
    samples_played: u64,
    percent_completed: f64,
}

impl<S: Sound> CompletionTracking<S> {
    pub fn new(inner: S, total_frames: u64) -> Self {
        CompletionTracking {
            inner,
            total_frames,
            samples_played: 0,
            percent_completed: 0.0,
        }
    }

    pub fn played_frames(&self) -> u64 {
        self.samples_played / self.inner.channel_count() as u64
    }
}

impl<S> Wrapper for CompletionTracking<S>
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

impl<S: Sound> Sound for CompletionTracking<S> {
    fn next_sample(&mut self) -> Result<NextSample, awedio::Error> {
        match self.inner.next_sample() {
            Ok(sample) => {
                self.samples_played += 1;

                self.percent_completed = if self.total_frames > 0 {
                    (self.played_frames() as f64 / self.total_frames as f64) * 100.0
                } else {
                    0.0
                };

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
