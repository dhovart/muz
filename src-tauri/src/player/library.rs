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

        if let Ok(entries) = fs::read_dir(&self.path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                        if SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                            let track = Track::new(path);
                            self.tracks.push(track);
                        }
                    }
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

    pub fn get_tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }

    pub fn update(&mut self, path: Option<PathBuf>, name: Option<String>) {
        if let Some(p) = path {
            self.path = p;
        }
        if let Some(n) = name {
            self.name = n;
        }
    }
}

#[cfg(test)]
#[path = "./library.tests.rs"]
mod tests;
