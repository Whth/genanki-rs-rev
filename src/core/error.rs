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
    ///
    /// Currently wraps `rusqlite::Error` but is type-erased to allow
    /// changing the database library in the future.
    #[error("Database error: {0}")]
    Database(Box<dyn std::error::Error + Send + Sync>),

    /// JSON parsing errors
    ///
    /// Currently wraps `serde_json::Error` but is type-erased.
    #[error("JSON error: {0}")]
    Json(Box<dyn std::error::Error + Send + Sync>),

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
    ///
    /// Currently wraps `ramhorns::Error` but is type-erased.
    #[error("Template rendering error: {0}")]
    TemplateRendering(Box<dyn std::error::Error + Send + Sync>),

    /// System time errors
    #[error("System time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),

    /// Zip file errors
    ///
    /// Currently wraps `zip::result::ZipError` but is type-erased.
    #[error("Zip error: {0}")]
    Zip(Box<dyn std::error::Error + Send + Sync>),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),
}

impl Error {
    /// Create a database error from any error type
    pub fn database<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Error::Database(Box::new(err))
    }

    /// Create a JSON error from any error type
    pub fn json<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Error::Json(Box::new(err))
    }

    /// Create a template rendering error from any error type
    pub fn template<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Error::TemplateRendering(Box::new(err))
    }

    /// Create a zip error from any error type
    pub fn zip<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Error::Zip(Box::new(err))
    }
}

// From implementations for common error types
impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::Database(Box::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(Box::new(err))
    }
}

impl From<ramhorns::Error> for Error {
    fn from(err: ramhorns::Error) -> Self {
        Error::TemplateRendering(Box::new(err))
    }
}

#[cfg(feature = "export")]
impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Zip(Box::new(err))
    }
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
