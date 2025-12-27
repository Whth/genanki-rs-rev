//! Storage layer for genanki-rs
//!
//! This crate handles all database operations and serialization for Anki packages.

pub mod schema;
pub mod models;
pub mod notes;
pub mod cards;
pub mod decks;
pub mod collection;

// Re-exports from schema
pub use schema::{
    AnkiSchema, SCHEMA_SQL, COL_SQL,
    DeckDbEntry, ModelDbEntry, FieldDbEntry, TemplateDbEntry
};

// Re-exports from modules
pub use cards::CardDbEntry;
pub use collection::{Collection, CollectionManager};
