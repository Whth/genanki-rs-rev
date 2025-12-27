//! Card database operations

use genanki_core::Card;
use rusqlite::{Transaction, params};
use std::ops::RangeFrom;

/// Write a card to the database
pub fn write_card_to_db(
    card: &Card,
    transaction: &Transaction,
    timestamp: f64,
    deck_id: i64,
    note_id: i64,
    id_gen: &mut RangeFrom<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let queue = card.queue_value();
    transaction.execute(
        "INSERT INTO cards VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);",
        params![
            id_gen.next().expect("Range overflowed!") as i64, // id
            note_id,                                          // nid
            deck_id,                                          // did
            card.ord(),                                       // ord
            timestamp as i64,                                 // mod
            -1_i64,                                           // usn
            0_i64,                                            // type (=0 for non-Cloze)
            queue,                                            // queue
            0_i64,                                            // due
            0_i64,                                            // ivl
            0_i64,                                            // factor
            0_i64,                                            // reps
            0_i64,                                            // lapses
            0_i64,                                            // left
            0_i64,                                            // odue
            0_i64,                                            // odid
            0_i64,                                            // flags
            "",                                               // data
        ],
    )?;
    Ok(())
}

/// Database entry for cards (for future use)
#[derive(Debug, Clone)]
pub struct CardDbEntry {
    pub id: i64,
    pub nid: i64,
    pub did: i64,
    pub ord: i64,
    pub queue: i64,
}
