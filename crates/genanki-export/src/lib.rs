//! APKG export functionality for genanki-rs
//!
//! This crate handles writing decks to .apkg files.

pub mod media;
pub mod package;

// Re-exports
pub use media::MediaFiles;
pub use package::{Package, PackageWriter};
