use super::*;
use std::path::PathBuf;

#[test]
fn test_new_track() {
    let path = PathBuf::from("/music/song.mp3");
    let track = Track::new(&path);
    assert_eq!(track.path, path);
    assert_eq!(track.title, Some("song.mp3".to_string()));
}
