//! Error types for genanki-rs

use std::io::Error as IoError;
use thiserror::Error;

/// Result type alias for genanki operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for genanki-rs
///
/// This error type uses transparent wrapping to allow changing underlying
/// implementations without breaking API compatibility.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// JSON parsing errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Template formatting errors
    #[error(
        "Could not compute required fields for this template; please check the formatting: {0:?}"
    )]
    TemplateFormat(String),

    /// Model field count mismatch
    #[error("Number of model fields ({0}) does not match number of provided fields ({1})")]
    ModelFieldCountMismatch(usize, usize),

    /// Invalid tag whitespace
    #[error("One of the tags contains whitespace, which is not allowed")]
    TagContainsWhitespace,

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    /// Template rendering errors
    #[error("Template rendering error: {0}")]
    TemplateRendering(#[from] ramhorns::Error),

    /// System time errors
    #[error("System time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    /// Zip file errors
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("No decks provided")]
    NoDecks,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::ModelFieldCountMismatch(2, 3);
        assert!(err.to_string().contains("2"));
        assert!(err.to_string().contains("3"));
    }

    #[test]
    fn test_error_conversions() {
        let io_err = IoError::new(std::io::ErrorKind::NotFound, "test");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
    }
}
