//! Core data structures and types for genanki-rs
//!
//! This crate provides the fundamental types used throughout the genanki ecosystem,
//! including models, notes, cards, and deck representations.

pub mod card;
pub mod config;
pub mod deck;
pub mod error;
pub mod guid;
pub mod model;
pub mod note;

// Re-exports for convenience
pub use card::Card;
pub use config::{AnkiConfig, DeckConfig, FieldDefaults, ModelConfig, ModelIds};
pub use deck::Deck;
pub use error::{Error, Result};
pub use guid::guid_for;
pub use model::{Field, Model, ModelType, Template};
pub use note::Note;
