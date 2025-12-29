//! Package integration tests

use genanki_rs_rev::{Deck, MediaFiles, Package};

#[test]
fn test_package_new_with_deck() {
    let deck = Deck::new(1234, "Test Deck", "Description");
    let result = Package::new(vec![deck], std::collections::HashMap::new());
    assert!(result.is_ok());
}

#[test]
fn test_package_new_empty_decks_error() {
    let result = Package::new(vec![], std::collections::HashMap::new());
    assert!(result.is_err());
}

#[test]
fn test_package_with_media_files() {
    let deck = Deck::new(1234, "Test Deck", "Description");
    let mut media = std::collections::HashMap::new();
    media.insert("test.mp3".to_string(), vec![1, 2, 3, 4]);
    let result = Package::new(vec![deck], media);
    assert!(result.is_ok());
}

#[test]
fn test_media_files_new() {
    let media = MediaFiles::new();
    assert!(media.is_empty());
}

#[test]
fn test_media_files_add() {
    let mut media = MediaFiles::new();
    media.add("file.mp3".to_string(), vec![1, 2, 3]);
    assert_eq!(media.len(), 1);
}

#[test]
fn test_media_files_get() {
    let mut media = MediaFiles::new();
    media.add("test.mp3".to_string(), vec![1, 2, 3, 4]);
    let data = media.get("test.mp3");
    assert!(data.is_some());
    assert_eq!(data.unwrap(), &[1, 2, 3, 4]);
}

#[test]
fn test_media_files_get_missing() {
    let media = MediaFiles::new();
    let data = media.get("missing.mp3");
    assert!(data.is_none());
}

#[test]
fn test_media_files_len() {
    let mut media = MediaFiles::new();
    assert_eq!(media.len(), 0);
    media.add("f1.mp3".to_string(), vec![]);
    assert_eq!(media.len(), 1);
    media.add("f2.mp3".to_string(), vec![]);
    assert_eq!(media.len(), 2);
}

#[test]
fn test_media_files_is_empty() {
    let media = MediaFiles::new();
    assert!(media.is_empty());
    let mut media2 = media;
    media2.add("file.mp3".to_string(), vec![]);
    assert!(!media2.is_empty());
}

#[test]
fn test_media_files_clone() {
    let mut media1 = MediaFiles::new();
    media1.add("test.mp3".to_string(), vec![1, 2, 3]);

    let media2 = media1.clone();
    assert_eq!(media2.len(), 1);
    let data = media2.get("test.mp3").unwrap();
    assert_eq!(data, &[1, 2, 3]);
}

#[test]
fn test_media_files_files() {
    let mut media = MediaFiles::new();
    media.add("test.mp3".to_string(), vec![1, 2, 3]);
    let files = media.files();
    assert!(files.contains_key("test.mp3"));
}
