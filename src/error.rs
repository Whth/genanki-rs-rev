use std::time::SystemTimeError;

use crate::db_entries::Tmpl;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Indicates an error happened with the database layer
    ///
    /// Currently the argument is a `rusqlite::Error`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    Database(#[from] rusqlite::Error),
    /// Indicates an error happened with the JSON parser
    ///
    /// Currently the argument is a `serde_json::Error`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    JsonParser(#[from] serde_json::Error),
    #[error(
        "Could not compute required fields for this template; please check the formatting of \"qfmt\": {0:?}"
    )]
    TemplateFormat(Box<Tmpl>),
    #[error("number of model field ({0}) does not match number of fields ({1})")]
    ModelFieldCountMismatch(usize, usize),
    #[error("One of the tags contains whitespace, this is not allowed!")]
    TagContainsWhitespace,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Indicates an error with the underlying template system
    ///
    /// Currently the argument is a `ramhorns::Error`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    Template(#[from] ramhorns::Error),
    #[error(transparent)]
    SystemTime(#[from] SystemTimeError),
    /// Indicates an error with zip file handling
    ///
    /// Currently the argument is a `zip::result::ZipError`, but it is
    /// cast to a Box<dyn std::error::Error> so that we can change
    /// the underlying library in the future if needed without breaking
    /// client code.
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}

pub type Result<T> = std::result::Result<T, Error>;
