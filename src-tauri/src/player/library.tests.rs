use super::*;
use std::path::PathBuf;

#[test]
fn test_new_library_initialization() {
    let path = PathBuf::from("/some/path/to/library");
    let name = "My Library".to_string();
    let library = Library::new(path.clone(), name.clone());
    assert_eq!(library.path, path);
    assert_eq!(library.name, name);
    // tracks should be initialized (empty if no files)
    assert!(library.tracks.is_empty());
}

#[test]
fn test_update_library() {
    let mut library = Library::new(
        PathBuf::from("/some/path/to/library"),
        "My Library".to_string(),
    );
    library.update(
        Some(PathBuf::from("/some/path/to/another/library")),
        Some("New Library".to_string()),
    );
    assert_eq!(library.path, PathBuf::from("/some/path/to/another/library"));
    assert_eq!(library.name, "New Library");
}

#[test]
fn test_update_only_path() {
    let mut library = Library::new(PathBuf::from("/some/path/to/library"), "Lib".to_string());
    library.update(Some(PathBuf::from("/other_music")), None);
    assert_eq!(library.path, PathBuf::from("/other_music"));
    assert_eq!(library.name, "Lib");
}

#[test]
fn test_update_only_name() {
    let mut library = Library::new(PathBuf::from("/some/path/to/library"), "Lib".to_string());
    library.update(None, Some("Renamed".to_string()));
    assert_eq!(library.path, PathBuf::from("/some/path/to/library"));
    assert_eq!(library.name, "Renamed");
}
