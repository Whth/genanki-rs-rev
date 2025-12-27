//! Genanki-rs - Create Anki flashcard decks
//!
//! This is the main library crate that re-exports all functionality.

// Re-export core types and functions
pub use genanki_core::{
    // Configuration
    AnkiConfig,
    Card,
    Deck,
    DeckConfig,
    // Error types
    Error,
    Field,
    FieldDefaults,
    // Core types
    Model,
    ModelConfig,
    ModelIds,

    // ModelType from both model and config (they're the same)
    ModelType,
    Note,
    Result,

    Template,

    // Utilities
    guid_for,
};

// Re-export storage types
pub use genanki_storage::{
    AnkiSchema, COL_SQL, Collection, CollectionManager, DeckDbEntry, ModelDbEntry, SCHEMA_SQL,
};

// Re-export builder types
pub use genanki_builder::{
    BasicModels, DeckBuilder, FieldBuilder, ModelBuilder, NoteBuilder, TemplateBuilder,
};

// Re-export export types
pub use genanki_export::{MediaFiles, Package, PackageWriter};

// ===== BACKWARD COMPATIBILITY =====
// Re-export old API for compatibility

pub mod builders {
    pub use genanki_builder::{FieldBuilder, TemplateBuilder};
    pub use genanki_core::{Field, Template};
}

pub mod constants {
    pub use genanki_storage::{COL_SQL as APKG_COL, SCHEMA_SQL as APKG_SCHEMA};
}

pub use genanki_builder as builder;
pub use genanki_core as core;
pub use genanki_export as export;
pub use genanki_storage as storage;

/// Basic model (backward compatible)
pub fn basic_model() -> Model {
    BasicModels::basic()
}

/// Basic and reversed card model (backward compatible)
pub fn basic_and_reversed_card_model() -> Model {
    Model::with_options(
        1485830179,
        "Basic (and reversed card) (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
            Template::new("Card 2")
                .qfmt("{{Back}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Basic optional reversed card model (backward compatible)
pub fn basic_optional_reversed_card_model() -> Model {
    Model::with_options(
        1382232460,
        "Basic (optional reversed card) (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
            Field::new("AddReverse").font("Arial"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
            Template::new("Card 2")
                .qfmt("{{#AddReverse}}{{Back}}{{/AddReverse}}")
                .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Front}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Basic type in the answer model (backward compatible)
pub fn basic_type_in_the_answer_model() -> Model {
    Model::with_options(
        1305534440,
        "Basic (type in the answer) (genanki)",
        vec![
            Field::new("Front").font("Arial"),
            Field::new("Back").font("Arial"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}\n\n{{type:Back}}")
                .afmt("{{Front}}\n\n<hr id=answer>\n\n{{type:Back}}"),
        ],
        Some(
            ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
        ),
        None,
        None,
        None,
        None,
    )
}

/// Cloze model (backward compatible)
pub fn cloze_model() -> Model {
    BasicModels::cloze()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_model() {
        let model = basic_model();
        assert_eq!(model.id, 1559383000);
        assert_eq!(model.num_fields(), 2);
    }

    #[test]
    fn test_cloze_model() {
        let model = cloze_model();
        assert_eq!(model.id, 1122529321);
        assert_eq!(model.num_fields(), 1);
    }

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new(1234, "Test", "Description");
        assert_eq!(deck.id, 1234);
        assert!(deck.is_empty());
    }

    #[test]
    fn test_note_creation() {
        let model = basic_model();
        let note = Note::new(model, vec!["Question", "Answer"]).unwrap();
        assert_eq!(note.fields().len(), 2);
    }
}
