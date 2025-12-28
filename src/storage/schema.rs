//! Database schema for Anki collections
//!
//! This module contains the SQL DDL for creating Anki database structures
//! and the default collection data.

use serde::{Deserialize, Serialize};

/// Anki database schema
pub static SCHEMA_SQL: &str = include_str!("../../sql/schema.sql");

/// Default collection data SQL
pub static COL_SQL: &str = include_str!("../../sql/default_inject.sql");

/// Database schema representation
#[derive(Debug, Clone)]
pub struct AnkiSchema;

impl AnkiSchema {
    /// Get the complete SQL schema
    pub fn get_schema() -> &'static str {
        SCHEMA_SQL
    }

    /// Get the default collection data
    pub fn get_collection_data() -> &'static str {
        COL_SQL
    }

    /// Initialize a database with the Anki schema
    pub fn init_db(conn: &mut rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
        conn.execute_batch(SCHEMA_SQL)?;
        conn.execute_batch(COL_SQL)?;
        Ok(())
    }
}

/// Database entry for decks
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckDbEntry {
    pub collapsed: bool,
    pub conf: i64,
    pub desc: String,
    #[serde(rename = "dyn")]
    pub deck_db_entry_dyn: i64,
    #[serde(rename = "extendNew")]
    pub extend_new: i64,
    #[serde(rename = "extendRev")]
    pub extend_rev: i64,
    pub id: i64,
    #[serde(rename = "lrnToday")]
    pub lrn_today: Vec<i64>,
    #[serde(rename = "mod")]
    pub deck_db_entry_mod: i64,
    pub name: String,
    #[serde(rename = "newToday")]
    pub new_today: Vec<i64>,
    #[serde(rename = "revToday")]
    pub rev_today: Vec<i64>,
    #[serde(rename = "timeToday")]
    pub time_today: Vec<i64>,
    pub usn: i64,
}

impl Default for DeckDbEntry {
    fn default() -> Self {
        Self {
            collapsed: false,
            conf: 1,
            desc: String::new(),
            deck_db_entry_dyn: 0,
            extend_new: 0,
            extend_rev: 50,
            id: 0,
            lrn_today: vec![163, 2],
            deck_db_entry_mod: 1425278051,
            name: String::new(),
            new_today: vec![163, 2],
            rev_today: vec![163, 0],
            time_today: vec![163, 23598],
            usn: -1,
        }
    }
}

/// Database entry for models
#[derive(Serialize, Deserialize, Clone)]
pub struct ModelDbEntry {
    pub vers: Vec<Option<serde_json::Value>>,
    pub name: String,
    pub tags: Vec<Option<serde_json::Value>>,
    pub did: i64,
    pub usn: i64,
    pub req: Vec<(usize, String, Vec<usize>)>,
    pub flds: Vec<FieldDbEntry>,
    pub sortf: i64,
    pub tmpls: Vec<TemplateDbEntry>,
    #[serde(rename = "mod")]
    pub model_db_entry_mod: i64,
    #[serde(rename = "latexPost")]
    pub latex_post: String,
    #[serde(rename = "type")]
    pub model_db_entry_type: i64,
    pub id: String,
    pub css: String,
    #[serde(rename = "latexPre")]
    pub latex_pre: String,
}

/// Database entry for fields
#[derive(Serialize, Deserialize, Clone)]
pub struct FieldDbEntry {
    pub name: String,
    pub media: Vec<Option<serde_json::Value>>,
    pub sticky: bool,
    pub rtl: bool,
    pub ord: i64,
    pub font: String,
    pub size: i64,
}

/// Database entry for templates
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateDbEntry {
    pub name: String,
    pub qfmt: String,
    pub did: Option<usize>,
    pub bafmt: String,
    pub afmt: String,
    pub ord: i64,
    pub bqfmt: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_not_empty() {
        assert!(!SCHEMA_SQL.is_empty());
        assert!(!COL_SQL.is_empty());
    }

    #[test]
    fn test_deck_db_entry_default() {
        let entry = DeckDbEntry::default();
        assert_eq!(entry.conf, 1);
        assert_eq!(entry.usn, -1);
    }
}
