//! Deck database operations

use crate::schema::DeckDbEntry;
use genanki_core::Deck;
use rusqlite::{Transaction, params};
use serde_json;

/// Convert a core Deck to a database entry
pub fn deck_to_db_entry(deck: &Deck) -> DeckDbEntry {
    DeckDbEntry {
        id: deck.id,
        name: deck.name.clone(),
        desc: deck.description.clone(),
        ..Default::default()
    }
}

/// Write deck to database
pub fn write_deck_to_db(
    deck: &Deck,
    transaction: &Transaction,
) -> Result<(), Box<dyn std::error::Error>> {
    let decks_json: String =
        transaction.query_row("SELECT decks FROM col", [], |row| row.get(0))?;
    let mut decks: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&decks_json)?;

    let deck_entry = deck_to_db_entry(deck);
    decks.insert(deck.id.to_string(), serde_json::to_value(&deck_entry)?);

    transaction.execute(
        "UPDATE col SET decks = ?",
        params![serde_json::to_string(&decks)?],
    )?;

    Ok(())
}
