//! Card representation in Anki
//!
//! A card represents a single flashcard generated from a note.

use crate::core::config::db;
use serde::{Deserialize, Serialize};

/// A flashcard generated from a note
///
/// Cards are created automatically when you add a note to a deck.
/// The number of cards depends on the model type and templates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    /// Ordinal/index of the card
    pub ord: i64,
    /// Whether the card is suspended
    pub suspend: bool,
}

impl Card {
    /// Creates a new card
    ///
    /// # Arguments
    ///
    /// * `ord` - The ordinal/index of this card
    /// * `suspend` - Whether the card should be suspended
    pub fn new(ord: i64, suspend: bool) -> Self {
        Self { ord, suspend }
    }

    /// Get the card's ordinal
    pub fn ord(&self) -> i64 {
        self.ord
    }

    /// Check if the card is suspended
    pub fn is_suspended(&self) -> bool {
        self.suspend
    }

    /// Set the suspended state
    pub fn with_suspended(mut self, suspend: bool) -> Self {
        self.suspend = suspend;
        self
    }

    /// Get the queue value for this card
    pub fn queue_value(&self) -> i64 {
        if self.suspend {
            db::queue::SUSPENDED
        } else {
            db::queue::NEW
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_new() {
        let card = Card::new(0, false);
        assert_eq!(card.ord(), 0);
        assert!(!card.is_suspended());
    }

    #[test]
    fn test_card_suspended() {
        let card = Card::new(0, true);
        assert!(card.is_suspended());
        assert_eq!(card.queue_value(), -1);
    }

    #[test]
    fn test_card_with_suspended() {
        let card = Card::new(0, false).with_suspended(true);
        assert!(card.is_suspended());
    }
}
