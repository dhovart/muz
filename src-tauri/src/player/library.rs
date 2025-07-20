use anyhow::Result;
use std::path::PathBuf;
use thiserror::Error;

use crate::player::track::{Track, SUPPORTED_EXTENSIONS};
use std::ffi::OsStr;
use std::fs;

pub struct Library {
    pub path: PathBuf,
    pub name: String,
    pub tracks: Vec<Track>,
}

impl Library {
    pub fn new(path: PathBuf, name: String) -> Self {
        let mut library = Self {
            path,
            name,
            tracks: Vec::new(),
        };

        library.build_tracks();

        library
    }

    fn build_tracks(&mut self) {
        self.tracks.clear();
        let path = self.path.clone();
        self.scan_directory_recursive(&path);
    }

    fn scan_directory_recursive(&mut self, dir_path: &PathBuf) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                        if SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                            let track = Track::new(path);
                            self.tracks.push(track);
                        }
                    }
                } else if path.is_dir() {
                    self.scan_directory_recursive(&path);
                }
            }
        }
    }

    pub fn create(&mut self) -> Result<()> {
        todo!("Implement library creation logic");
    }

    pub fn delete(&mut self) -> Result<()> {
        todo!("Implement library deletion logic");
    }

    pub fn tracks_cloned(&self) -> Vec<Track> {
        self.tracks.clone()
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    pub fn track_by_id(&self, track_id: &str) -> Option<Track> {
        self.tracks
            .iter()
            .find(|track| track.id == track_id)
            .cloned()
    }

    pub fn update(&mut self, path: Option<PathBuf>, name: Option<String>) {
        if let Some(p) = path {
            self.path = p;
        }
        if let Some(n) = name {
            self.name = n;
        }
    }

    pub fn rescan(&mut self) {
        self.build_tracks();
    }
}

#[cfg(test)]
#[path = "./library.tests.rs"]
mod tests;
