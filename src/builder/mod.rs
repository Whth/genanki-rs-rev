//! Builder patterns for genanki-rs
//!
//! This module provides enhanced builder patterns and DSL for creating Anki components.

pub mod deck;
pub mod field;
pub mod model;
pub mod note;
pub mod template;

// Re-exports
pub use deck::DeckBuilder;
pub use field::{FieldBuilder, FieldDefaultsConstants};
pub use model::{BasicModels, ModelBuilder};
pub use note::NoteBuilder;
pub use template::{TemplateBuilder, TemplateDefaults};
