use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey};
use symphonia::core::probe::{Hint, ProbeResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub path: PathBuf,
    pub title: Option<String>,
    pub total_frames: u64,
}

pub static SUPPORTED_EXTENSIONS: &[&str] = &[
    "aac", "adpcm", "aif", "aifc", "aiff", "alac", "caf", "flac", "m4a", "m4b", "m4p", "m4r",
    "m4v", "mka", "mkv", "mp1", "mp2", "mp3", "mp4", "oga", "ogg", "ogv", "ogx", "wav", "wave",
    "weba", "webm",
];

impl Track {
    pub fn new<P: Into<PathBuf> + AsRef<Path>>(path: P) -> Self {
        let (total_frames, title) = Self::get_metadata(path.as_ref());
        if total_frames.is_none() {
            eprintln!("Failed to get total frames for track: {:?}", path.as_ref());
        }
        Self {
            path: path.into(),
            title,
            total_frames: total_frames.unwrap_or(0),
        }
    }

    fn get_title_from_probe(probed: &mut ProbeResult) -> Result<String> {
        let metadata = &mut probed.metadata;
        if let Some(metadata) = metadata.get() {
            if let Some(revision) = metadata.current() {
                for tag in revision.tags() {
                    if let Some(StandardTagKey::TrackTitle) = tag.std_key {
                        return Ok(tag.value.to_string());
                    }
                }
            }
        }
        Err(anyhow::anyhow!("No title found"))
    }

    fn get_total_frames_from_probe(probed: &ProbeResult) -> Result<u64> {
        let format = &probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No audio track found"))?;

        if let Some(n_frames) = track.codec_params.n_frames {
            Ok(n_frames)
        } else {
            Err(anyhow::anyhow!("Unable to determine total frames"))
        }
    }

    pub fn get_metadata(path: &Path) -> (Option<u64>, Option<String>) {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return (None, None),
        };
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                hint.with_extension(ext_str);
            }
        }

        let mut probed = match symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        ) {
            Ok(p) => p,
            Err(_) => return (None, None),
        };

        let total_frames = Self::get_total_frames_from_probe(&probed).ok();
        let title = Self::get_title_from_probe(&mut probed).ok();

        (total_frames, title)
    }
}

#[cfg(test)]
#[path = "./track.tests.rs"]
mod tests;
