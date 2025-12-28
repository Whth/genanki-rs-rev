//! Integration tests for APKG export functionality
//!
//! These tests verify that packages can be created and exported correctly.

use genanki_rs_rev::{basic_model, cloze_model, Deck, Model, Note, Field, Template};
use std::fs::File;
use std::io::Read;
use tempfile::TempDir;

fn create_package_result(deck: Deck) -> Result<genanki_rs_rev::Package, genanki_rs_rev::export::package::PackageError> {
    genanki_rs_rev::Package::new(vec![deck], std::collections::HashMap::new())
}

#[test]
fn test_package_write_to_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.apkg");

    let deck = Deck::new(1234, "Test Deck", "A test deck");
    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
    // APKG files are ZIP archives, should be at least a few bytes
    let metadata = std::fs::metadata(&output_path).unwrap();
    assert!(metadata.len() > 100);
}

#[test]
fn test_package_write_with_notes() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("with_notes.apkg");

    let mut deck = Deck::new(1234, "Notes Deck", "Deck with notes");
    let model = basic_model();

    for i in 0..5 {
        let note = Note::new(model.clone(), vec![&format!("Question {}", i), &format!("Answer {}", i)]).unwrap();
        deck.add_note(note);
    }

    assert_eq!(deck.num_notes(), 5);

    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_write_with_media() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("with_media.apkg");

    let deck = Deck::new(1234, "Media Deck", "Deck with media files");
    let mut media = std::collections::HashMap::new();

    // Add some fake media data
    media.insert("sound.mp3".to_string(), vec![0x52, 0x49, 0x46, 0x46]); // "RIFF" header
    media.insert("image.png".to_string(), vec![0x89, 0x50, 0x4E, 0x47]); // PNG header

    let package = genanki_rs_rev::Package::new(vec![deck], media).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_write_is_valid_zip() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("valid_zip.apkg");

    let deck = Deck::new(1234, "Zip Test", "Test ZIP validity");
    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    // Verify it's a valid ZIP file by opening it with zip crate
    let file = File::open(&output_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    // Should contain the database file
    assert!(archive.by_name("collection.anki2").is_ok());
    // Should contain media mapping
    assert!(archive.by_name("collection.media").is_ok());
}

#[test]
fn test_package_multiple_decks() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("multi_deck.apkg");

    let deck1 = Deck::new(1000, "Deck 1", "First deck");
    let deck2 = Deck::new(2000, "Deck 2", "Second deck");

    let package = genanki_rs_rev::Package::new(vec![deck1, deck2], std::collections::HashMap::new()).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_with_custom_model() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("custom_model.apkg");

    let model = Model::new(
        9999999999,
        "Custom Model",
        vec![
            Field::new("Term"),
            Field::new("Definition"),
            Field::new("Example"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Term}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Definition}}"#),
            Template::new("Card 2")
                .qfmt("{{Definition}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Term}}"#),
        ],
    );

    let mut deck = Deck::new(5555, "Custom", "Custom model deck");
    let note = Note::new(model, vec!["Rust", "A systems language", "cargo new"]).unwrap();
    deck.add_note(note);

    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_with_tags() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("tags.apkg");

    let model = basic_model();
    let note = Note::with_options(
        model,
        vec!["Q", "A"],
        None,
        Some(vec!["programming", "rust", "testing"]),
        None,
    ).unwrap();

    let mut deck = Deck::new(7777, "Tags Test", "Test tags");
    deck.add_note(note);

    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_rejects_empty_decks() {
    let result = genanki_rs_rev::Package::new(vec![], std::collections::HashMap::new());
    assert!(result.is_err());
}

#[test]
fn test_package_with_large_media() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("large_media.apkg");

    let deck = Deck::new(8888, "Large Media", "Test large media files");
    let mut media = std::collections::HashMap::new();

    // Add a larger "file" (1KB of data)
    let large_data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    media.insert("large_file.dat".to_string(), large_data);

    let package = genanki_rs_rev::Package::new(vec![deck], media).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_filename_special_chars() {
    let temp_dir = TempDir::new().unwrap();
    // Create path with special characters (spaces, etc.)
    let output_path = temp_dir.path().join("test deck (special).apkg");

    let deck = Deck::new(9999, "Special", "Test special filename");
    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}

#[test]
fn test_package_contains_database() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("db_contents.apkg");

    let mut deck = Deck::new(4444, "DB Test", "Test database contents");
    let model = basic_model();
    deck.add_note(Note::new(model, vec!["Q", "A"]).unwrap());

    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    // Open and verify database contents
    let file = File::open(&output_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    // Check database file exists and has content (SQLite binary header)
    let mut db_file = archive.by_name("collection.anki2").unwrap();
    let mut header = vec![0u8; 16];
    db_file.read_exact(&mut header).unwrap();

    // SQLite database starts with "SQLite format 3"
    assert_eq!(&header[0..6], b"SQLite");
}

#[test]
fn test_package_with_cloze_notes() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("cloze.apkg");

    let cloze = cloze_model();
    let note = Note::new(
        cloze,
        vec!["The capital of {{c1::France}} is {{c2::Paris}}."],
    ).unwrap();

    let mut deck = Deck::new(3333, "Cloze Test", "Test cloze notes");
    deck.add_note(note);

    let package = create_package_result(deck).unwrap();
    package.write_to_file(&output_path).unwrap();

    assert!(output_path.exists());
}
