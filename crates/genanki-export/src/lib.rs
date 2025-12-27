//! APKG export functionality for genanki-rs
//!
//! This crate handles writing decks to .apkg files.

pub mod package;
pub mod media;

// Re-exports
pub use package::{Package, PackageWriter};
pub use media::MediaFiles;
