use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub path: PathBuf,
    pub title: Option<String>,
    pub duration: Option<Duration>,
}

pub static SUPPORTED_EXTENSIONS: &[&str] = &[
    "aac", "adpcm", "aif", "aifc", "aiff", "alac", "caf", "flac", "m4a", "m4b", "m4p", "m4r",
    "m4v", "mka", "mkv", "mp1", "mp2", "mp3", "mp4", "oga", "ogg", "ogv", "ogx", "wav", "wave",
    "weba", "webm",
];

impl Track {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            title: None,
            duration: None,
        }
    }

    pub fn set_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn set_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

#[cfg(test)]
#[path = "./track.tests.rs"]
mod tests;
