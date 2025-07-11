use super::*;
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn test_new_track() {
    let path = PathBuf::from("/music/song.mp3");
    let track = Track::new(&path);
    assert_eq!(track.path, path);
    assert!(track.title.is_none());
    assert!(track.duration.is_none());
}

#[test]
fn test_set_title() {
    let track = Track::new("/music/song.mp3").set_title("Title");
    assert_eq!(track.title, Some("Title".to_string()));
}

#[test]
fn test_set_duration() {
    let dur = Duration::from_secs(120);
    let track = Track::new("/music/song.mp3").set_duration(dur);
    assert_eq!(track.duration, Some(dur));
}
