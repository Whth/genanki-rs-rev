//! Decks in Anki
//!
//! A deck is a collection of notes.

use crate::core::model::Model;
use crate::core::note::Note;
use std::collections::HashMap;

/// A flashcard deck which can be written to an .apkg file
///
/// Decks contain notes and track which models are used.
#[derive(Clone)]
pub struct Deck {
    pub id: i64,
    pub name: String,
    pub description: String,
    notes: Vec<Note>,
    models: HashMap<i64, Model>,
}

impl Deck {
    /// Create a new deck
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for this deck
    /// * `name` - The name of the deck
    /// * `description` - A description of the deck
    ///
    /// # Example
    ///
    /// ```
    /// use genanki_rs_rev::core::Deck;
    ///
    /// let deck = Deck::new(1234, "My Deck", "This is my deck");
    /// ```
    pub fn new(id: i64, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            notes: Vec::new(),
            models: HashMap::new(),
        }
    }

    /// Add a note to the deck
    ///
    /// # Example
    ///
    /// ```no_run
    /// use genanki_rs_rev::core::{Deck, Note, Model, Field, Template};
    ///
    /// let mut deck = Deck::new(1234, "My Deck", "");
    /// let model = Model::new(
    ///     123,
    ///     "Basic",
    ///     vec![Field::new("Front"), Field::new("Back")],
    ///     vec![Template::new("Card 1").qfmt("{{Front}}").afmt("{{Back}}")],
    /// );
    /// let note = Note::new(model, vec!["Question", "Answer"]).unwrap();
    /// deck.add_note(note);
    /// ```
    pub fn add_note(&mut self, note: Note) {
        // Track the model
        let model_id = note.model().id;
        self.models.insert(model_id, note.model().clone());
        self.notes.push(note);
    }

    /// Add multiple notes
    pub fn add_notes(&mut self, notes: Vec<Note>) {
        for note in notes {
            self.add_note(note);
        }
    }

    /// Get all notes
    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    /// Get all notes (mutable)
    pub fn notes_mut(&mut self) -> &mut [Note] {
        &mut self.notes
    }

    /// Get all models
    pub fn models_items(&self) -> &HashMap<i64, Model> {
        &self.models
    }

    pub fn models(&self) -> Vec<&Model> {
        self.models.values().collect()
    }

    /// Get number of notes
    pub fn num_notes(&self) -> usize {
        self.notes.len()
    }

    /// Get number of models
    pub fn num_models(&self) -> usize {
        self.models.len()
    }

    /// Check if deck is empty
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Set name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Field, Template};

    #[test]
    fn test_deck_new() {
        let deck = Deck::new(1234, "Test Deck", "Test Description");
        assert_eq!(deck.id, 1234);
        assert_eq!(deck.name, "Test Deck");
        assert_eq!(deck.description, "Test Description");
        assert!(deck.is_empty());
    }

    #[test]
    fn test_deck_add_note() {
        let mut deck = Deck::new(1234, "Test", "");
        let model = Model::new(
            123,
            "Basic",
            vec![Field::new("F"), Field::new("B")],
            vec![Template::new("C1").qfmt("{{F}}").afmt("{{B}}")],
        );

        let note = Note::new(model, vec!["Q", "A"]).unwrap();
        deck.add_note(note);

        assert_eq!(deck.num_notes(), 1);
        assert_eq!(deck.num_models(), 1);
    }

    #[test]
    fn test_deck_with_modifiers() {
        let deck = Deck::new(1234, "Old Name", "Old Desc")
            .with_name("New Name")
            .with_description("New Desc");

        assert_eq!(deck.name, "New Name");
        assert_eq!(deck.description, "New Desc");
    }
}
