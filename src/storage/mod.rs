//! Storage layer for genanki-rs
//!
//! This module handles all database operations and serialization for Anki packages.

pub mod cards;
pub mod collection;
pub mod decks;
pub mod models;
pub mod notes;
pub mod schema;

// Re-exports from schema
pub use schema::{
    AnkiSchema, COL_SQL, DeckDbEntry, FieldDbEntry, ModelDbEntry, SCHEMA_SQL, TemplateDbEntry,
};

// Re-exports from modules
pub use cards::CardDbEntry;
pub use collection::{Collection, CollectionManager};
