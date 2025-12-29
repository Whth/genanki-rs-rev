//! Package creation and export

use crate::core::Deck;
use crate::storage::{CollectionManager, cards, decks, models, notes};
use crate::{Error, ModelDbEntry, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::ops::RangeFrom;
use std::path::Path;
use std::time::SystemTime;
use tempfile::NamedTempFile;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Errors that can occur during package writing

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
            return Err(Error::NoDecks);
        }
        Ok(Self { decks, media_files })
    }

    /// Write to a file
    pub fn write_to_file<P: AsRef<Path>>(self, path: P) -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;

        let mut collection = CollectionManager::open(&temp_file)?;
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

        let package_file = File::create(path)?;

        let opt = SimpleFileOptions::default();
        let mut zip = ZipWriter::new(package_file);
        let mut buf = vec![];
        temp_file.rewind()?;
        temp_file.read_to_end(&mut buf)?;
        zip.start_file(crate::constants::DATABASE_FILENAME, opt)?;
        zip.write_all(&buf)?;

        let media_files_mapping_string =
            serde_json::to_string(&self.prepare_media_files_mapping())?;

        zip.start_file(crate::constants::MEDIA_MAPPING_FILENAME, opt)?;
        zip.write_all(media_files_mapping_string.as_bytes())?;

        self.media_files.iter().try_for_each(|(name, data)| {
            zip.start_file(format!("{}/{name}", crate::constants::MEDIA_DIRNAME), opt)?;
            zip.write_all(data)?;
            Ok::<(), Error>(())
        })?;

        Ok(())
    }

    fn prepare_media_files_mapping(&self) -> HashMap<String, String> {
        self.media_files
            .keys()
            .map(|name| {
                (
                    name.clone(),
                    format!("{}/{}", crate::constants::MEDIA_DIRNAME, name),
                )
            })
            .collect()
    }

    fn write_deck_to_db(
        &self,
        deck: &Deck,
        conn: &mut rusqlite::Connection,
        timestamp: f64,
        id_gen: &mut RangeFrom<usize>,
    ) -> Result<()> {
        let transaction = conn.transaction()?;

        // 1. Write deck
        decks::write_deck_to_db(deck, &transaction)?;

        // 2. Write models
        {
            // a. Read existing models from DB
            let models_json_str: String =
                transaction.query_row("SELECT models FROM col", [], |row| row.get(0))?;

            let mut models: HashMap<i64, ModelDbEntry> = serde_json::from_str(&models_json_str)?;

            // b. Convert each model to DB entry and insert into map
            for model in deck.models() {
                let mut model_clone = model.clone(); // or avoid clone if possible
                let db_entry = models::model_to_db_entry(&mut model_clone, timestamp, deck.id);
                models.insert(model.id, db_entry);
            }

            // c. Write back updated models JSON
            let models_json = serde_json::to_string(&models)?;
            transaction.execute("UPDATE col SET models = ?", [models_json])?;
        }

        // 3. Write notes and cards
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
