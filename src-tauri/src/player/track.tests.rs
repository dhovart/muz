use super::*;
use std::path::PathBuf;

#[test]
fn test_track_creation() {
    let track = Track::new("test.mp3");
    assert!(!track.id.is_empty());
    assert_eq!(track.path.to_str().unwrap(), "test.mp3");
    assert_eq!(track.total_frames, 0); // Will be 0 for non-existent file
    assert_eq!(track.duration_ms, 0); // Will be 0 for non-existent file
}

#[test]
fn test_track_uuid_uniqueness() {
    let track1 = Track::new("test1.mp3");
    let track2 = Track::new("test2.mp3");
    assert_ne!(track1.id, track2.id, "Each track should have a unique ID");
}

#[test]
fn test_default_title() {
    let path = PathBuf::from("/path/to/my_song.mp3");
    let title = Track::default_title(&path);
    assert_eq!(title, "my_song.mp3");
}

#[test]
fn test_default_title_no_extension() {
    let path = PathBuf::from("/path/to/song");
    let title = Track::default_title(&path);
    assert_eq!(title, "song");
}

#[test]
fn test_default_title_empty_path() {
    let path = PathBuf::from("");
    let title = Track::default_title(&path);
    assert_eq!(title, "Unknown");
}

#[test]
fn test_path_handling() {
    // Test various path formats
    let paths = vec![
        "/home/user/music/song.mp3",
        "C:\\Users\\User\\Music\\song.mp3",
        "./relative/path/song.flac",
        "../parent/song.wav",
    ];

    for path_str in paths {
        let track = Track::new(path_str);
        assert_eq!(track.path.to_string_lossy(), path_str);
        assert!(!track.id.is_empty());
    }
}
