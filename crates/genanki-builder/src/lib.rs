//! Builder patterns for genanki-rs
//!
//! This crate provides enhanced builder patterns and DSL for creating Anki components.

pub mod field;
pub mod template;
pub mod model;
pub mod note;
pub mod deck;

// Re-exports
pub use field::{FieldBuilder, FieldDefaultsConstants};
pub use template::{TemplateBuilder, TemplateDefaults};
pub use model::{ModelBuilder, BasicModels};
pub use note::NoteBuilder;
pub use deck::DeckBuilder;
