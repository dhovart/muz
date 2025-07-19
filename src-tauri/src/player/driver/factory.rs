use crate::player::driver::PlaybackDriver;
use anyhow::Result;

pub trait PlaybackDriverFactory {
    fn create_driver(volume: f32) -> Result<Box<dyn PlaybackDriver>>;
}

pub struct DefaultDriverFactory;

impl PlaybackDriverFactory for DefaultDriverFactory {
    fn create_driver(volume: f32) -> Result<Box<dyn PlaybackDriver>> {
        #[cfg(feature = "rodio-driver")]
        {
            use crate::player::driver::rodio::RodioPlaybackDriver;
            Ok(Box::new(RodioPlaybackDriver::new(volume)?))
        }

        #[cfg(all(feature = "awedio-driver", not(feature = "rodio-driver")))]
        {
            use crate::player::driver::awedio::AwedioPlaybackDriver;
            Ok(Box::new(AwedioPlaybackDriver::new(volume)?))
        }

        #[cfg(not(any(feature = "rodio-driver", feature = "awedio-driver")))]
        {
            compile_error!("Must enable either 'rodio-driver' or 'awedio-driver' feature");
        }
    }
}

pub fn get_active_backend() -> &'static str {
    #[cfg(feature = "rodio-driver")]
    return "rodio";

    #[cfg(all(feature = "awedio-driver", not(feature = "rodio-driver")))]
    return "awedio";

    #[cfg(not(any(feature = "rodio-driver", feature = "awedio-driver")))]
    return "none";
}
