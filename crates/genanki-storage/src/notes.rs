//! Note database operations

use genanki_core::Note;
use rusqlite::{Transaction, params};
use std::ops::RangeFrom;

/// Write a note to the database
pub fn write_note_to_db(
    note: &Note,
    transaction: &Transaction,
    timestamp: f64,
    _deck_id: i64,
    id_gen: &mut RangeFrom<usize>,
) -> Result<i64, Box<dyn std::error::Error>> {
    note.check_invalid_html();

    let note_id = id_gen.next().expect("Range overflowed!") as i64;

    transaction.execute(
        "INSERT INTO notes VALUES(?,?,?,?,?,?,?,?,?,?,?);",
        params![
            note_id,                          // id
            note.guid(),                      // guid
            note.model().id,                  // mid
            timestamp as i64,                 // mod
            -1_i64,                           // usn
            note.format_tags(),               // tags
            note.format_fields(),             // flds
            false,                            // sfld
            0_i64,                            // csum
            0_i64,                            // flags
            "",                               // data
        ],
    )?;

    Ok(note_id)
}
