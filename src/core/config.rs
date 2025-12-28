//! Configuration constants for Anki
//!
//! This module centralizes all hard-coded values previously scattered throughout the codebase.

use serde::{Deserialize, Serialize};

/// Default field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefaults {
    /// Default font family
    pub font: &'static str,
    /// Default font size
    pub size: i64,
    /// Right-to-left text direction
    pub rtl: bool,
    /// Sticky field behavior
    pub sticky: bool,
}

impl Default for FieldDefaults {
    fn default() -> Self {
        Self {
            font: "Liberation Sans",
            size: 20,
            rtl: false,
            sticky: false,
        }
    }
}

/// Model configuration defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Default LaTeX preamble
    pub latex_pre: &'static str,
    /// Default LaTeX postscript
    pub latex_post: &'static str,
    /// Default CSS
    pub css: String,
    /// Default sort field index
    pub sort_field_index: i64,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            latex_pre: r#"
\documentclass[12pt]{article}
\special{papersize:3in,5in}
\usepackage[utf8]{inputenc}
\usepackage{amssymb,amsmath}
\pagestyle{empty}
\setlength{\parindent:0in}
\begin{document}

"#,
            latex_post: r"\end{document}",
            css: String::new(),
            sort_field_index: 0,
        }
    }
}

/// Deck configuration defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckConfig {
    /// Learning today stats
    pub lrn_today: Vec<i64>,
    /// New cards today stats
    pub new_today: Vec<i64>,
    /// Review cards today stats
    pub rev_today: Vec<i64>,
    /// Time spent today stats
    pub time_today: Vec<i64>,
    /// Deck modification time
    pub modification_time: i64,
    /// Default deck configuration ID
    pub conf: i64,
    /// Extend new cards
    pub extend_new: i64,
    /// Extend review cards
    pub extend_rev: i64,
    /// Dynamic deck type
    pub is_dynamic: i64,
}

impl Default for DeckConfig {
    fn default() -> Self {
        Self {
            lrn_today: vec![163, 2],
            new_today: vec![163, 2],
            rev_today: vec![163, 0],
            time_today: vec![163, 23598],
            modification_time: 1425278051,
            conf: 1,
            extend_new: 0,
            extend_rev: 50,
            is_dynamic: 0,
        }
    }
}

/// Collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionConfig {
    /// Creation timestamp
    pub crt: i64,
    /// Modification timestamp
    pub modification: i64,
    /// Schema modification timestamp
    pub schema_mod: i64,
    /// Version
    pub ver: i64,
    /// Dirty flag
    pub dty: i64,
    /// Update sequence number
    pub usn: i64,
    /// Last sync time
    pub ls: i64,
    /// Model IDs for built-in models
    pub model_ids: ModelIds,
}

/// Built-in model IDs
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ModelIds {
    pub basic_model: i64,
    pub basic_and_reversed_card_model: i64,
    pub basic_optional_reversed_card_model: i64,
    pub basic_type_in_the_answer_model: i64,
    pub cloze_model: i64,
}

impl Default for CollectionConfig {
    fn default() -> Self {
        Self {
            crt: 1411124400,
            modification: 1425279151694,
            schema_mod: 1425279151690,
            ver: 11,
            dty: 0,
            usn: 0,
            ls: 0,
            model_ids: ModelIds::default(),
        }
    }
}

impl Default for ModelIds {
    fn default() -> Self {
        Self {
            basic_model: 1559383000,
            basic_and_reversed_card_model: 1485830179,
            basic_optional_reversed_card_model: 1382232460,
            basic_type_in_the_answer_model: 1305534440,
            cloze_model: 1122529321,
        }
    }
}

/// Anki configuration aggregate
#[derive(Debug, Clone, Default)]
pub struct AnkiConfig {
    pub field_defaults: FieldDefaults,
    pub model: ModelConfig,
    pub deck: DeckConfig,
    pub collection: CollectionConfig,
}

impl AnkiConfig {
    /// Create a new AnkiConfig with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom model configuration
    pub fn with_model(mut self, model: ModelConfig) -> Self {
        self.model = model;
        self
    }

    /// Create with custom deck configuration
    pub fn with_deck(mut self, deck: DeckConfig) -> Self {
        self.deck = deck;
        self
    }

    /// Get the CSS for a specific built-in model
    pub fn get_model_css(model_type: ModelType) -> String {
        match model_type {
            ModelType::Basic => {
                ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n".to_string()
            }
            ModelType::Cloze => {
                ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n\n \
                 .cloze {\n font-weight: bold;\n color: blue;\n}\n.nightMode .cloze {\n color: lightblue;\n}".to_string()
            }
        }
    }
}

/// Model type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Basic,
    Cloze,
}

/// Constants for field separators
pub const FIELD_SEPARATOR: char = '\x1f';
pub const FIELD_SEPARATOR_STR: &str = "\x1f";

/// Database constants
pub mod db {
    /// Unknown sync number
    pub const USN_UNKNOWN: i64 = -1;

    /// Queue types
    pub mod queue {
        /// New card
        pub const NEW: i64 = 0;
        /// Suspended card
        pub const SUSPENDED: i64 = -1;
    }

    /// Card types
    pub mod card_type {
        /// Learning card
        pub const LEARNING: i64 = 0;
        /// Review card
        pub const REVIEW: i64 = 1;
        /// Relearning card
        pub const RELEARNING: i64 = 2;
    }

    /// Default values for card fields
    pub const DEFAULT_DUE: i64 = 0;
    pub const DEFAULT_IVL: i64 = 0;
    pub const DEFAULT_FACTOR: i64 = 0;
    pub const DEFAULT_REPS: i64 = 0;
    pub const DEFAULT_LAPSES: i64 = 0;
    pub const DEFAULT_LEFT: i64 = 0;
    pub const DEFAULT_ODUE: i64 = 0;
    pub const DEFAULT_ODID: i64 = 0;
    pub const DEFAULT_FLAGS: i64 = 0;
    pub const DEFAULT_DATA: &str = "";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AnkiConfig::new();
        assert_eq!(config.field_defaults.font, "Liberation Sans");
        assert_eq!(config.field_defaults.size, 20);
    }

    #[test]
    fn test_model_ids() {
        let ids = ModelIds::default();
        assert_eq!(ids.basic_model, 1559383000);
        assert_eq!(ids.cloze_model, 1122529321);
    }
}
