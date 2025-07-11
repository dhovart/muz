use super::*;
use std::path::PathBuf;

#[test]
fn test_update_library() {
    let mut library = Library::new(PathBuf::from("/music"), "My Library".to_string());
    library.update(
        Some(PathBuf::from("/new_music")),
        Some("New Library".to_string()),
    );
    assert_eq!(library.path, PathBuf::from("/new_music"));
    assert_eq!(library.name, "New Library");
}
