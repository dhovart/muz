use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey};
use symphonia::core::probe::{Hint, ProbeResult};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub album_artist: Option<String>,
    pub track_number: Option<i32>,
    pub disc_number: Option<i32>,
    pub genre: Option<String>,
    pub year: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    pub path: PathBuf,
    pub total_frames: u64,
    pub duration_ms: u64,
    pub metadata: Option<TrackMetadata>,
}

pub static SUPPORTED_EXTENSIONS: &[&str] = &[
    "aac", "adpcm", "aif", "aifc", "aiff", "alac", "caf", "flac", "m4a", "m4b", "m4p", "m4r",
    "m4v", "mka", "mkv", "mp1", "mp2", "mp3", "mp4", "oga", "ogg", "ogv", "ogx", "wav", "wave",
    "weba", "webm",
];

impl Track {
    pub fn new<P: Into<PathBuf> + AsRef<Path>>(path: P) -> Self {
        let (total_frames, duration_ms, mut metadata) = Self::get_metadata(path.as_ref());
        if total_frames.is_none() {
            eprintln!("Failed to get total frames for track: {:?}", path.as_ref());
        }
        if let Some(metadata) = metadata.as_mut() {
            if metadata.title.is_none() {
                metadata.title = Some(Self::default_title(path.as_ref()));
            }
        }
        Self {
            id: Uuid::new_v4().to_string(),
            path: path.into(),
            total_frames: total_frames.unwrap_or(0),
            duration_ms: duration_ms.unwrap_or(0),
            metadata,
        }
    }

    pub fn default_title(path: &Path) -> String {
        path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }

    fn get_metadata_from_probe(mut probed: ProbeResult) -> TrackMetadata {
        let mut meta = TrackMetadata {
            title: None,
            album: None,
            artist: None,
            album_artist: None,
            track_number: None,
            disc_number: None,
            genre: None,
            year: None,
        };

        let mut fill_from_tags = |tags: &[symphonia::core::meta::Tag]| {
            for tag in tags {
                match tag.std_key {
                    Some(StandardTagKey::TrackTitle) => meta.title = Some(tag.value.to_string()),
                    Some(StandardTagKey::Album) => meta.album = Some(tag.value.to_string()),
                    Some(StandardTagKey::Artist) => meta.artist = Some(tag.value.to_string()),
                    Some(StandardTagKey::AlbumArtist) => {
                        meta.album_artist = Some(tag.value.to_string())
                    }
                    Some(StandardTagKey::TrackNumber) => {
                        let s = tag.value.to_string();
                        meta.track_number = s.parse::<i32>().ok();
                    }
                    Some(StandardTagKey::DiscNumber) => {
                        let s = tag.value.to_string();
                        meta.disc_number = s.parse::<i32>().ok();
                    }
                    Some(StandardTagKey::Genre) => meta.genre = Some(tag.value.to_string()),
                    Some(StandardTagKey::Date) => meta.year = Some(tag.value.to_string()),
                    _ => {}
                }
            }
        };

        if let Some(metadata_rev) = probed.format.metadata().skip_to_latest() {
            fill_from_tags(metadata_rev.tags());
        }
        let mut metadata = probed.metadata;
        if let Some(mut m) = metadata.get() {
            if let Some(metadata_rev) = m.skip_to_latest() {
                fill_from_tags(metadata_rev.tags());
            }
        }
        meta
    }

    fn get_audio_track_from_probe(
        probed: &ProbeResult,
    ) -> Result<&symphonia::core::formats::Track> {
        let format = &probed.format;
        format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No audio track found"))
    }

    fn get_total_frames_from_probe(probed: &ProbeResult) -> Result<u64> {
        let track = Self::get_audio_track_from_probe(probed)?;
        track
            .codec_params
            .n_frames
            .ok_or_else(|| anyhow::anyhow!("Unable to determine total frames"))
    }

    fn get_duration_from_probe(probed: &ProbeResult) -> Result<u64> {
        let track = Self::get_audio_track_from_probe(probed)?;
        if let (Some(n_frames), Some(sample_rate)) =
            (track.codec_params.n_frames, track.codec_params.sample_rate)
        {
            let duration_seconds = n_frames as f64 / sample_rate as f64;
            Ok((duration_seconds * 1000.0) as u64)
        } else {
            Err(anyhow::anyhow!("Unable to determine duration"))
        }
    }

    pub fn get_metadata(path: &Path) -> (Option<u64>, Option<u64>, Option<TrackMetadata>) {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to open file: {:?}", path);
                return (None, None, None);
            }
        };
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                hint.with_extension(ext_str);
            }
        }

        let probed = match symphonia::default::get_probe().format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        ) {
            Ok(p) => p,
            Err(_) => {
                eprintln!("Failed to probe file: {:?}", path);
                return (None, None, None);
            }
        };

        let total_frames = Self::get_total_frames_from_probe(&probed).ok();
        let duration_ms = Self::get_duration_from_probe(&probed).ok();
        let metadata = Some(Self::get_metadata_from_probe(probed));
        (total_frames, duration_ms, metadata)
    }
}

#[cfg(test)]
#[path = "./track.tests.rs"]
mod tests;
