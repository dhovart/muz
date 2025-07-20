use crate::player::driver::PlaybackDriver;
use anyhow::Result;

pub trait PlaybackDriverFactory {
    fn create_driver(volume: f32) -> Result<Box<dyn PlaybackDriver>>;
}

pub struct DefaultDriverFactory;

impl PlaybackDriverFactory for DefaultDriverFactory {
    fn create_driver(volume: f32) -> Result<Box<dyn PlaybackDriver>> {
        use crate::player::driver::rodio::RodioPlaybackDriver;
        Ok(Box::new(RodioPlaybackDriver::new(volume)?))
    }
}

pub fn get_active_backend() -> &'static str {
    "rodio"
}
