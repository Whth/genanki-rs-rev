//! Core data structures and types for genanki-rs
//!
//! This crate provides the fundamental types used throughout the genanki ecosystem,
//! including models, notes, cards, and deck representations.

pub mod config;
pub mod error;
pub mod model;
pub mod note;
pub mod card;
pub mod deck;
pub mod guid;

// Re-exports for convenience
pub use config::{AnkiConfig, ModelConfig, DeckConfig, FieldDefaults, ModelIds};
pub use error::{Error, Result};
pub use model::{Model, ModelType, Field, Template};
pub use note::Note;
pub use card::Card;
pub use deck::Deck;
pub use guid::guid_for;
