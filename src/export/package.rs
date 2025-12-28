//! Package creation and export

use crate::core::Deck;
use crate::storage::{CollectionManager, cards, decks, models, notes};
use std::collections::HashMap;
use std::ops::RangeFrom;
use std::path::Path;
use std::time::{SystemTime, SystemTimeError};
use tempfile::NamedTempFile;
use thiserror::Error;

/// Errors that can occur during package writing
#[derive(Error, Debug)]
pub enum PackageError {
    #[error("Database error: {0}")]
    Database(Box<dyn std::error::Error>),

    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("Zip error: {0}")]
    Zip(String),

    #[error("No decks provided")]
    NoDecks,

    #[error("System time error: {0}")]
    SystemTime(SystemTimeError),
}

// From implementations
impl From<rusqlite::Error> for PackageError {
    fn from(err: rusqlite::Error) -> Self {
        PackageError::Database(Box::new(err))
    }
}

impl From<Box<dyn std::error::Error>> for PackageError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        PackageError::Database(err)
    }
}

impl From<std::io::Error> for PackageError {
    fn from(err: std::io::Error) -> Self {
        PackageError::Io(err)
    }
}

impl From<SystemTimeError> for PackageError {
    fn from(err: SystemTimeError) -> Self {
        PackageError::SystemTime(err)
    }
}

/// Result type for package operations
pub type Result<T> = std::result::Result<T, PackageError>;

/// Package containing one or more decks
#[allow(dead_code)]
pub struct Package {
    decks: Vec<Deck>,
    media_files: HashMap<String, Vec<u8>>,
}

impl Package {
    /// Create a new package
    pub fn new(decks: Vec<Deck>, media_files: HashMap<String, Vec<u8>>) -> Result<Self> {
        if decks.is_empty() {
            return Err(PackageError::NoDecks);
        }
        Ok(Self { decks, media_files })
    }

    /// Write to a file
    pub fn write_to_file<P: AsRef<Path>>(&self, _path: P) -> Result<()> {
        let mut collection = CollectionManager::memory()?;
        collection.init_schema()?;

        // Write decks, models, notes, and cards
        let mut id_gen = 0..;
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs_f64()
            * 1000.0;

        for deck in &self.decks {
            self.write_deck_to_db(deck, collection.connection_mut(), timestamp, &mut id_gen)?;
        }

        // Write to temp file then zip
        let temp_db = NamedTempFile::new()?;
        let _temp_db_path = temp_db.into_temp_path();

        // Export database to temp file
        // ... (implementation details)

        Ok(())
    }

    fn write_deck_to_db(
        &self,
        deck: &Deck,
        conn: &mut rusqlite::Connection,
        timestamp: f64,
        id_gen: &mut RangeFrom<usize>,
    ) -> Result<()> {
        let transaction = conn.transaction()?;

        // Write deck
        decks::write_deck_to_db(deck, &transaction)?;

        // Write models
        for (_, model) in deck.models() {
            let mut model_clone = model.clone();
            let _db_entry = models::model_to_db_entry(&mut model_clone, timestamp, deck.id);
            // Write model to database...
        }

        // Write notes and cards
        for note in deck.notes() {
            let note_id = notes::write_note_to_db(note, &transaction, timestamp, deck.id, id_gen)?;

            for card in note.cards() {
                cards::write_card_to_db(card, &transaction, timestamp, deck.id, note_id, id_gen)?;
            }
        }

        transaction.commit()?;
        Ok(())
    }
}

/// Writer for creating packages
pub struct PackageWriter {
    media: HashMap<String, Vec<u8>>,
}

impl PackageWriter {
    pub fn new() -> Self {
        Self {
            media: HashMap::new(),
        }
    }

    pub fn add_media<P: AsRef<Path>>(&mut self, name: &str, path: P) -> Result<()> {
        use std::io::Read;
        let mut file = std::fs::File::open(path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        self.media.insert(name.to_string(), data);
        Ok(())
    }

    pub fn build(self, decks: Vec<Deck>) -> Result<Package> {
        Package::new(decks, self.media)
    }
}

impl Default for PackageWriter {
    fn default() -> Self {
        Self::new()
    }
}
