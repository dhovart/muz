use crate::player::{library::Library, track::Track};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct LibraryService {
    library: Arc<Mutex<Library>>,
}

impl LibraryService {
    pub fn new(library: Arc<Mutex<Library>>) -> Self {
        Self { library }
    }

    pub fn get_library_path(&self) -> Result<String> {
        let library = self.library.lock().unwrap();
        Ok(library.path.to_string_lossy().to_string())
    }

    pub fn set_library_path(&self, path: PathBuf) -> Result<()> {
        let mut library = self.library.lock().unwrap();
        library.update(Some(path), None);
        library.rescan();
        Ok(())
    }

    pub fn rescan_library(&self) -> Result<()> {
        let mut library = self.library.lock().unwrap();
        library.rescan();
        Ok(())
    }

    pub fn get_library_tracks(&self) -> Result<HashMap<String, Vec<Track>>> {
        let library = self.library.lock().unwrap();
        let tracks = library.get_tracks();

        let mut grouped: HashMap<String, Vec<Track>> = HashMap::new();
        for track in tracks {
            let album = track
                .metadata
                .as_ref()
                .and_then(|m| m.album.clone())
                .unwrap_or_else(|| "Unknown Album".to_string());
            grouped.entry(album).or_insert_with(Vec::new).push(track);
        }

        Ok(grouped)
    }

    pub fn get_albums_by_artist(&self) -> Result<HashMap<String, HashMap<String, Vec<Track>>>> {
        let library = self.library.lock().unwrap();
        let tracks = library.get_tracks();

        let mut grouped: HashMap<String, HashMap<String, Vec<Track>>> = HashMap::new();

        for track in tracks {
            let artist = track
                .metadata
                .as_ref()
                .and_then(|m| m.album_artist.as_ref().or(m.artist.as_ref()))
                .cloned()
                .unwrap_or_else(|| "Unknown Artist".to_string());

            let album = track
                .metadata
                .as_ref()
                .and_then(|m| m.album.clone())
                .unwrap_or_else(|| "Unknown Album".to_string());

            grouped
                .entry(artist)
                .or_insert_with(HashMap::new)
                .entry(album)
                .or_insert_with(Vec::new)
                .push(track);
        }

        for artist_albums in grouped.values_mut() {
            for album_tracks in artist_albums.values_mut() {
                album_tracks.sort_by(|a, b| {
                    let a_track_num = a
                        .metadata
                        .as_ref()
                        .and_then(|m| m.track_number)
                        .unwrap_or(0);
                    let b_track_num = b
                        .metadata
                        .as_ref()
                        .and_then(|m| m.track_number)
                        .unwrap_or(0);
                    a_track_num.cmp(&b_track_num)
                });
            }
        }

        Ok(grouped)
    }

    pub fn get_track_by_id(&self, track_id: &str) -> Result<Track> {
        let library = self.library.lock().unwrap();
        library
            .get_track_by_id(track_id)
            .ok_or_else(|| anyhow::anyhow!("Track not found"))
    }

    pub fn get_tracks_by_album(&self, album_name: &str, artist_name: &str) -> Result<Vec<Track>> {
        let library = self.library.lock().unwrap();
        let mut tracks: Vec<Track> = library
            .get_tracks()
            .into_iter()
            .filter(|track| {
                let unknown_album = "Unknown Album".to_string();
                let unknown_artist = "Unknown Artist".to_string();

                let track_album = track
                    .metadata
                    .as_ref()
                    .and_then(|m| m.album.as_ref())
                    .unwrap_or(&unknown_album);
                let track_artist = track
                    .metadata
                    .as_ref()
                    .and_then(|m| m.album_artist.as_ref().or(m.artist.as_ref()))
                    .unwrap_or(&unknown_artist);

                track_album == album_name && track_artist == artist_name
            })
            .collect();

        if tracks.is_empty() {
            return Err(anyhow::anyhow!("Album not found"));
        }

        tracks.sort_by(|a, b| {
            let a_track_num = a
                .metadata
                .as_ref()
                .and_then(|m| m.track_number)
                .unwrap_or(0);
            let b_track_num = b
                .metadata
                .as_ref()
                .and_then(|m| m.track_number)
                .unwrap_or(0);
            a_track_num.cmp(&b_track_num)
        });

        Ok(tracks)
    }
}
