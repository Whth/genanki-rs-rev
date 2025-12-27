//! Collection management

use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

/// Anki collection manager
pub struct CollectionManager {
    conn: Connection,
}

impl CollectionManager {
    /// Open a collection from a file
    pub fn open<P: AsRef<Path>>(path: P) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    /// Create an in-memory collection
    pub fn memory() -> SqlResult<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    /// Initialize with Anki schema
    pub fn init_schema(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        crate::schema::AnkiSchema::init_db(&mut self.conn)?;
        Ok(())
    }

    /// Get the underlying connection
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Get the underlying connection (mutable)
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

/// Collection wrapper for type safety
pub struct Collection(pub CollectionManager);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_memory() {
        let col = CollectionManager::memory().unwrap();
        assert!(col.connection().is_open());
    }
}
