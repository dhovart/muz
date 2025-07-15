use super::*;
use std::path::PathBuf;

#[test]
fn test_new_track() {
    let path = PathBuf::from("/music/song.mp3");
    let track = Track::new(&path);
    assert_eq!(track.path, path);
    assert!(track.title.is_none());
    assert!(track.total_frames.is_none());
}
