//! Note builder

use genanki_core::{Note, Model};
use anyhow::Result;

/// Builder for notes
pub struct NoteBuilder {
    model: Option<Model>,
    fields: Vec<String>,
    tags: Vec<String>,
    guid: Option<String>,
    sort_field: bool,
}

impl NoteBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            fields: Vec::new(),
            tags: Vec::new(),
            guid: None,
            sort_field: false,
        }
    }

    pub fn model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }

    pub fn field(mut self, field: &str) -> Self {
        self.fields.push(field.to_string());
        self
    }

    pub fn fields(mut self, fields: Vec<&str>) -> Self {
        self.fields = fields.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn guid(mut self, guid: &str) -> Self {
        self.guid = Some(guid.to_string());
        self
    }

    pub fn sort_field(mut self, sort: bool) -> Self {
        self.sort_field = sort;
        self
    }

    pub fn build(self) -> Result<Note> {
        let model = self.model.ok_or_else(|| anyhow::anyhow!("Model is required"))?;

        if self.fields.is_empty() {
            return Err(anyhow::anyhow!("Fields are required"));
        }

        Ok(Note::with_options(
            model,
            self.fields.iter().map(|s| s.as_str()).collect(),
            Some(self.sort_field),
            if self.tags.is_empty() {
                None
            } else {
                Some(self.tags.iter().map(|s| s.as_str()).collect())
            },
            self.guid.as_deref(),
        )?)
    }
}

impl Default for NoteBuilder {
    fn default() -> Self {
        Self::new()
    }
}
