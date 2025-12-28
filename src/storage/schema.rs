//! Database schema for Anki collections
//!
//! This module contains the SQL DDL for creating Anki database structures
//! and the default collection data.

use serde::{Deserialize, Serialize};

/// Anki database schema
pub static SCHEMA_SQL: &str = r#"CREATE TABLE col (
    id              integer primary key,
    crt             integer not null,
    mod             integer not null,
    scm             integer not null,
    ver             integer not null,
    dty             integer not null,
    usn             integer not null,
    ls              integer not null,
    conf            text not null,
    models          text not null,
    decks           text not null,
    dconf           text not null,
    tags            text not null
);
CREATE TABLE notes (
    id              integer primary key,   /* 0 */
    guid            text not null,         /* 1 */
    mid             integer not null,      /* 2 */
    mod             integer not null,      /* 3 */
    usn             integer not null,      /* 4 */
    tags            text not null,         /* 5 */
    flds            text not null,         /* 6 */
    sfld            integer not null,      /* 7 */
    csum            integer not null,      /* 8 */
    flags           integer not null,      /* 9 */
    data            text not null          /* 10 */
);
CREATE TABLE cards (
    id              integer primary key,   /* 0 */
    nid             integer not null,      /* 1 */
    did             integer not null,      /* 2 */
    ord             integer not null,      /* 3 */
    mod             integer not null,      /* 4 */
    usn             integer not null,      /* 5 */
    type            integer not null,      /* 6 */
    queue           integer not null,      /* 7 */
    due             integer not null,      /* 8 */
    ivl             integer not null,      /* 9 */
    factor          integer not null,      /* 10 */
    reps            integer not null,      /* 11 */
    lapses          integer not null,      /* 12 */
    left            integer not null,      /* 13 */
    odue            integer not null,      /* 14 */
    odid            integer not null,      /* 15 */
    flags           integer not null,      /* 16 */
    data            text not null          /* 17 */
);
CREATE TABLE revlog (
    id              integer primary key,
    cid             integer not null,
    usn             integer not null,
    ease            integer not null,
    ivl             integer not null,
    lastIvl         integer not null,
    factor          integer not null,
    time            integer not null,
    type            integer not null
);
CREATE TABLE graves (
    usn             integer not null,
    oid             integer not null,
    type            integer not null
);
CREATE INDEX ix_notes_usn on notes (usn);
CREATE INDEX ix_cards_usn on cards (usn);
CREATE INDEX ix_revlog_usn on revlog (usn);
CREATE INDEX ix_cards_nid on cards (nid);
CREATE INDEX ix_cards_sched on cards (did, queue, due);
CREATE INDEX ix_revlog_cid on revlog (cid);
CREATE INDEX ix_notes_csum on notes (csum);
"#;

/// Default collection data SQL
pub static COL_SQL: &str = r#"INSERT INTO col VALUES(
    null,
    1411124400,
    1425279151694,
    1425279151690,
    11,
    0,
    0,
    0,
    '{
        "activeDecks": [
            1
        ],
        "addToCur": true,
        "collapseTime": 1200,
        "curDeck": 1,
        "curModel": "1425279151691",
        "dueCounts": true,
        "estTimes": true,
        "newBury": true,
        "newSpread": 0,
        "nextPos": 1,
        "sortBackwards": false,
        "sortType": "noteFld",
        "timeLim": 0
    }',
    '{}',
    '{
        "1": {
            "collapsed": false,
            "conf": 1,
            "desc": "",
            "dyn": 0,
            "extendNew": 10,
            "extendRev": 50,
            "id": 1,
            "lrnToday": [
                0,
                0
            ],
            "mod": 1425279151,
            "name": "Default",
            "newToday": [
                0,
                0
            ],
            "revToday": [
                0,
                0
            ],
            "timeToday": [
                0,
                0
            ],
            "usn": 0
        }
    }',
    '{
        "1": {
            "autoplay": true,
            "id": 1,
            "lapse": {
                "delays": [
                    10
                ],
                "leechAction": 0,
                "leechFails": 8,
                "minInt": 1,
                "mult": 0
            },
            "maxTaken": 60,
            "mod": 0,
            "name": "Default",
            "new": {
                "bury": true,
                "delays": [
                    1,
                    10
                ],
                "initialFactor": 2500,
                "ints": [
                    1,
                    4,
                    7
                ],
                "order": 1,
                "perDay": 20,
                "separate": true
            },
            "replayq": true,
            "rev": {
                "bury": true,
                "ease4": 1.3,
                "fuzz": 0.05,
                "ivlFct": 1,
                "maxIvl": 36500,
                "minSpace": 1,
                "perDay": 100
            },
            "timer": 0,
            "usn": 0
        }
    }',
    '{}'
);
"#;

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
