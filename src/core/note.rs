//! Notes in Anki
//!
//! A note represents a flashcard with specific field values.

use crate::core::card::Card;
use crate::core::config::FIELD_SEPARATOR_STR;
use crate::core::guid::guid_for;
use crate::core::model::{Model, ModelType};
use crate::error::{Error, Result};
use fancy_regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

/// A note (flashcard) to be added to a deck
///
/// Notes are created from models and contain field values.
#[derive(Clone)]
pub struct Note {
    model: Model,
    fields: Vec<String>,
    sort_field: bool,
    tags: Vec<String>,
    guid: String,
    cards: Vec<Card>,
}

impl Note {
    /// Create a new note
    ///
    /// # Arguments
    ///
    /// * `model` - The model to use for this note
    /// * `fields` - The field values for this note
    ///
    /// # Example
    ///
    /// ```
    /// use genanki_rs_rev::core::{Note, Model, Field, Template};
    ///
    /// let model = Model::new(
    ///     123,
    ///     "Basic",
    ///     vec![Field::new("Front"), Field::new("Back")],
    ///     vec![Template::new("Card 1").qfmt("{{Front}}").afmt("{{Back}}")],
    /// );
    /// let note = Note::new(model, vec!["Capital of France", "Paris"]).unwrap();
    /// ```
    pub fn new(model: Model, fields: Vec<&str>) -> Result<Self> {
        let fields: Vec<String> = fields.iter().map(|s| s.to_string()).collect();

        // Validate field count
        if model.num_fields() != fields.len() {
            return Err(Error::ModelFieldCountMismatch(
                model.num_fields(),
                fields.len(),
            ));
        }

        let cards = match model.model_type {
            ModelType::Basic => generate_basic_cards(&model, &fields)?,
            ModelType::Cloze => generate_cloze_cards(&model, &fields),
        };

        let guid = guid_for(&fields);

        Ok(Self {
            model,
            fields,
            sort_field: false,
            tags: Vec::new(),
            guid,
            cards,
        })
    }

    /// Create a new note with options
    #[allow(clippy::too_many_arguments)]
    pub fn with_options(
        model: Model,
        fields: Vec<&str>,
        sort_field: Option<bool>,
        tags: Option<Vec<&str>>,
        guid: Option<&str>,
    ) -> Result<Self> {
        let tags: Vec<String> = tags
            .unwrap_or_default()
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Validate tags
        validate_tags(&tags)?;

        let fields: Vec<String> = fields.iter().map(|s| s.to_string()).collect();

        // Validate field count
        if model.num_fields() != fields.len() {
            return Err(Error::ModelFieldCountMismatch(
                model.num_fields(),
                fields.len(),
            ));
        }

        let cards = match model.model_type {
            ModelType::Basic => generate_basic_cards(&model, &fields)?,
            ModelType::Cloze => generate_cloze_cards(&model, &fields),
        };

        let guid = guid.unwrap_or(&guid_for(&fields)).to_string();

        Ok(Self {
            model,
            fields,
            sort_field: sort_field.unwrap_or(false),
            tags,
            guid,
            cards,
        })
    }

    /// Set sort field
    pub fn with_sort_field(mut self, sort_field: bool) -> Self {
        self.sort_field = sort_field;
        self
    }

    /// Set tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: impl ToString) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Set GUID
    pub fn with_guid(mut self, guid: impl ToString) -> Self {
        self.guid = guid.to_string();
        self
    }

    /// Get the model
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Get the model (mut)
    pub fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    /// Get the cards
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Get the GUID
    pub fn guid(&self) -> &str {
        &self.guid
    }

    /// Get the fields
    pub fn fields(&self) -> &[String] {
        &self.fields
    }

    /// Get the tags
    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    /// Format fields for database storage
    pub fn format_fields(&self) -> String {
        self.fields.join(FIELD_SEPARATOR_STR)
    }

    /// Format tags for database storage
    pub fn format_tags(&self) -> String {
        format!(" {} ", self.tags.join(" "))
    }

    /// Check for invalid HTML tags in fields
    pub fn check_invalid_html(&self) {
        for field in &self.fields {
            let invalid_tags = find_invalid_html_tags(field);
            if !invalid_tags.is_empty() {
                eprintln!(
                    "Warning: The field '{}' contains invalid HTML tags: {:?}",
                    field, invalid_tags
                );
            }
        }
    }
}

/// Generate cards for basic model type
fn generate_basic_cards(model: &Model, fields: &[String]) -> Result<Vec<Card>> {
    let mut cards = Vec::new();

    for (card_ord, any_or_all, required_field_ords) in model.req()?.iter() {
        let should_create = match any_or_all.as_str() {
            "any" => required_field_ords
                .iter()
                .any(|&ord| !fields[ord].is_empty()),
            "all" => required_field_ords
                .iter()
                .all(|&ord| !fields[ord].is_empty()),
            _ => {
                return Err(Error::Validation(format!(
                    "Invalid req type: {}",
                    any_or_all
                )));
            }
        };

        if should_create {
            cards.push(Card::new(*card_ord as i64, false));
        }
    }

    Ok(cards)
}

/// Generate cards for cloze model type
fn generate_cloze_cards(model: &Model, fields: &[String]) -> Vec<Card> {
    let mut card_ords: HashSet<i64> = HashSet::new();
    let mut cloze_replacements: HashSet<String> = HashSet::new();

    // Find cloze field names in templates
    cloze_replacements.extend(re_findall(
        r"{{[^}]*?cloze:(?:[^}]?:)*(.+?)}}",
        &model.templates[0].qfmt,
    ));
    cloze_replacements.extend(re_findall("<%cloze:(.+?)%>", &model.templates[0].qfmt));

    let empty_string = String::new();
    for field_name in cloze_replacements {
        let field_value = model
            .fields
            .iter()
            .position(|f| f.name == field_name)
            .map(|idx| &fields[idx])
            .unwrap_or(&empty_string);

        let updates_str = re_findall(r"(?s){{c(\d+)::.+?}}", field_value);
        let updates = updates_str
            .iter()
            .filter_map(|m| i64::from_str(m).ok())
            .map(|m| m - 1)
            .filter(|&m| m >= 0);

        card_ords.extend(updates);
    }

    if card_ords.is_empty() {
        card_ords.insert(0);
    }

    card_ords.iter().map(|&ord| Card::new(ord, false)).collect()
}

/// Find all regex matches in a string
fn re_findall(pattern: &str, text: &str) -> Vec<String> {
    let regex = Regex::new(pattern).expect("Invalid regex pattern");
    regex
        .captures_iter(text)
        .filter_map(|m| m.ok())
        .flat_map(|cap| {
            cap.iter()
                .skip(1)
                .flatten()
                .map(|m| m.as_str().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Validate tags don't contain whitespace
fn validate_tags(tags: &[String]) -> Result<()> {
    if tags.iter().any(|tag| tag.contains(' ')) {
        Err(Error::TagContainsWhitespace)
    } else {
        Ok(())
    }
}

/// Find invalid HTML tags in a field
fn find_invalid_html_tags(field: &str) -> Vec<String> {
    let regex = Regex::new(r"<(?!/?[a-z0-9]+(?: .*|/?)>)(?:.|\n)*?>").unwrap();
    regex
        .find_iter(field)
        .filter_map(|m| m.ok())
        .map(|m| m.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Field, Template};

    #[test]
    fn test_note_new() {
        let model = Model::new(
            123,
            "Basic",
            vec![Field::new("Front"), Field::new("Back")],
            vec![Template::new("Card 1").qfmt("{{Front}}").afmt("{{Back}}")],
        );

        let note = Note::new(model, vec!["Question", "Answer"]).unwrap();
        assert_eq!(note.fields().len(), 2);
        assert_eq!(note.fields()[0], "Question");
        assert_eq!(note.fields()[1], "Answer");
    }

    #[test]
    fn test_note_field_count_mismatch() {
        let model = Model::new(
            123,
            "Basic",
            vec![Field::new("Front"), Field::new("Back")],
            vec![Template::new("Card 1")],
        );

        let result = Note::new(model, vec!["Only one field"]);
        assert!(matches!(result, Err(Error::ModelFieldCountMismatch(_, _))));
    }

    #[test]
    fn test_note_tags() {
        let model = Model::new(
            123,
            "Basic",
            vec![Field::new("Front"), Field::new("Back")],
            vec![Template::new("Card 1")],
        );

        let note = Note::with_options(
            model,
            vec!["Q", "A"],
            None,
            Some(vec!["tag1", "tag2"]),
            None,
        )
        .unwrap();

        assert_eq!(note.tags(), &["tag1", "tag2"]);
    }

    #[test]
    fn test_validate_tags_whitespace() {
        let result = validate_tags(&["valid tag".to_string()]);
        assert!(matches!(result, Err(Error::TagContainsWhitespace)));

        let result = validate_tags(&["valid_tag".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_fields() {
        let model = Model::new(
            123,
            "Basic",
            vec![Field::new("F1"), Field::new("F2")],
            vec![Template::new("Card 1")],
        );

        let note = Note::new(model, vec!["A", "B"]).unwrap();
        assert_eq!(note.format_fields(), "A\x1fB");
    }
}
