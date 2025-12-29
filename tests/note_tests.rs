//! Note integration tests

use genanki_rs_rev::{Error, Field, Model, Note, Template, basic_model, cloze_model};

#[test]
fn test_note_creation_with_basic_model() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::new(
        model,
        vec!["What is Rust?", "A systems programming language"],
    )?;
    assert_eq!(note.fields().len(), 2);
    Ok(())
}

#[test]
fn test_note_creation_with_cloze_model() -> Result<(), Error> {
    let model = cloze_model();
    let note = Note::new(model, vec!["The capital of France is {{c1::Paris}}."])?;
    assert_eq!(note.fields().len(), 1);
    Ok(())
}

#[test]
fn test_note_with_multiple_fields() -> Result<(), Error> {
    let model = Model::new(
        1234567890,
        "Multi-field Model",
        vec![
            Field::new("Term"),
            Field::new("Definition"),
            Field::new("Example"),
            Field::new("Notes"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Term}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Definition}}"#),
        ],
    );

    let note = Note::new(
        model,
        vec![
            "Rust",
            "A systems programming language",
            "cargo new my_project",
            "Memory safe",
        ],
    )?;
    assert_eq!(note.fields().len(), 4);
    Ok(())
}

#[test]
fn test_note_with_html_content() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::new(model, vec!["<b>Bold Question</b>", "<i>Italic Answer</i>"])?;
    assert!(note.fields()[0].contains("<b>"));
    assert!(note.fields()[1].contains("<i>"));
    Ok(())
}

#[test]
fn test_note_duplicate_fields_count() -> Result<(), Error> {
    let model = basic_model();
    let result = Note::new(model, vec!["Single field"]);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_note_tags() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::with_options(
        model,
        vec!["Question", "Answer"],
        None,
        Some(vec!["programming", "rust", "testing"]),
        None,
    )?;
    assert_eq!(note.tags().len(), 3);
    Ok(())
}

#[test]
fn test_note_empty_tags() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::with_options(model, vec!["Question", "Answer"], None, Some(vec![]), None)?;
    assert!(note.tags().is_empty());
    Ok(())
}

#[test]
fn test_note_with_special_characters() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::new(model, vec!["What is 1 + 1?", "2 < 3 and 4 > 2"])?;
    assert!(note.fields()[1].contains("<"));
    assert!(note.fields()[1].contains(">"));
    Ok(())
}

#[test]
fn test_note_with_unicode() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::new(model, vec!["What is 你好?", "Hello in Chinese is 你好"])?;
    assert!(note.fields()[0].contains("你好"));
    assert!(note.fields()[1].contains("你好"));
    Ok(())
}

#[test]
fn test_note_guid_is_deterministic() -> Result<(), Error> {
    let model = basic_model();
    let note1 = Note::new(model.clone(), vec!["Q", "A"])?;
    let note2 = Note::new(model, vec!["Q", "A"])?;
    // Same fields should produce same GUID (deterministic)
    assert_eq!(note1.guid(), note2.guid());
    Ok(())
}

#[test]
fn test_note_custom_guid() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::with_options(
        model,
        vec!["Q", "A"],
        None,
        None,
        Some("my_custom_guid_123"),
    )?;
    assert_eq!(note.guid(), "my_custom_guid_123");
    Ok(())
}

#[test]
fn test_note_cards_generated() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::new(model, vec!["Question", "Answer"])?;
    // Basic model should generate at least one card
    assert!(!note.cards().is_empty());
    Ok(())
}

#[test]
fn test_cloze_note_cards() -> Result<(), Error> {
    let model = cloze_model();
    let note = Note::new(model, vec!["Text with {{c1::cloze}} deletion."])?;
    // Cloze model should generate cards
    assert!(!note.cards().is_empty());
    Ok(())
}

#[test]
fn test_note_with_tag_whitespace_error() {
    let model = basic_model();
    let result = Note::with_options(
        model,
        vec!["Q", "A"],
        None,
        Some(vec!["invalid tag with space"]),
        None,
    );
    assert!(result.is_err());
}

#[test]
fn test_note_format_fields() -> Result<(), Error> {
    let model = Model::new(
        123,
        "Test",
        vec![Field::new("F1"), Field::new("F2")],
        vec![Template::new("Card 1")],
    );
    let note = Note::new(model, vec!["Value1", "Value2"])?;
    let formatted = note.format_fields();
    // Fields should be separated by field separator
    assert!(formatted.contains("Value1"));
    assert!(formatted.contains("Value2"));
    Ok(())
}

#[test]
fn test_note_format_tags() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::with_options(
        model,
        vec!["Q", "A"],
        None,
        Some(vec!["tag1", "tag2"]),
        None,
    )?;
    let formatted = note.format_tags();
    assert!(formatted.contains("tag1"));
    assert!(formatted.contains("tag2"));
    Ok(())
}
