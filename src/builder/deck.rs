//! Deck builder

use crate::core::{Deck, Note};

/// Builder for decks
pub struct DeckBuilder {
    id: i64,
    name: String,
    description: String,
    notes: Vec<Note>,
}

impl DeckBuilder {
    pub fn new(id: i64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            notes: Vec::new(),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    pub fn note(mut self, note: Note) -> Self {
        self.notes.push(note);
        self
    }

    pub fn notes(mut self, notes: Vec<Note>) -> Self {
        self.notes = notes;
        self
    }

    pub fn build(self) -> Deck {
        let mut deck = Deck::new(self.id, &self.name, &self.description);
        for note in self.notes {
            deck.add_note(note);
        }
        deck
    }
}
