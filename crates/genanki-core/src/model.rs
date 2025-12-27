//! Card models in Anki
//!
//! A model defines the structure of notes, including fields and templates.

use crate::config::{ModelConfig};
use crate::error::{Error, Result};
use fancy_regex::Regex;
use ramhorns::Template as RamTemplate;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Re-export ModelType from config for convenience
pub use crate::config::ModelType;

/// Template for a card
#[derive(Clone, Debug)]
pub struct Template {
    pub name: String,
    pub qfmt: String,
    pub afmt: String,
}

impl Template {
    /// Create a new template
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            qfmt: String::new(),
            afmt: String::new(),
        }
    }

    /// Set the question format
    pub fn qfmt(mut self, qfmt: &str) -> Self {
        self.qfmt = qfmt.to_string();
        self
    }

    /// Set the answer format
    pub fn afmt(mut self, afmt: &str) -> Self {
        self.afmt = afmt.to_string();
        self
    }

    /// Load template from file
    pub fn load_qfmt_from_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.qfmt = content;
        Ok(self)
    }

    /// Load answer format from file
    pub fn load_afmt_from_file<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.afmt = content;
        Ok(self)
    }
}

/// Field in a model
#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub font: Option<String>,
    pub size: Option<i64>,
    pub rtl: Option<bool>,
    pub sticky: Option<bool>,
}

impl Field {
    /// Create a new field
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            font: None,
            size: None,
            rtl: None,
            sticky: None,
        }
    }

    /// Set the font
    pub fn font(mut self, font: &str) -> Self {
        self.font = Some(font.to_string());
        self
    }

    /// Set the font size
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }

    /// Set right-to-left
    pub fn rtl(mut self, rtl: bool) -> Self {
        self.rtl = Some(rtl);
        self
    }

    /// Set sticky
    pub fn sticky(mut self, sticky: bool) -> Self {
        self.sticky = Some(sticky);
        self
    }
}

/// A model defines the structure of notes
#[derive(Clone)]
pub struct Model {
    pub id: i64,
    pub name: String,
    pub fields: Vec<Field>,
    pub templates: Vec<Template>,
    pub css: String,
    pub model_type: ModelType,
    pub latex_pre: String,
    pub latex_post: String,
    pub sort_field_index: i64,
}

impl Model {
    /// Create a new model
    pub fn new(id: i64, name: &str, fields: Vec<Field>, templates: Vec<Template>) -> Self {
        Self {
            id,
            name: name.to_string(),
            fields,
            templates,
            css: String::new(),
            model_type: ModelType::Basic,
            latex_pre: ModelConfig::default().latex_pre.to_string(),
            latex_post: ModelConfig::default().latex_post.to_string(),
            sort_field_index: 0,
        }
    }

    /// Create a new model with options
    #[allow(clippy::too_many_arguments)]
    pub fn with_options(
        id: i64,
        name: &str,
        fields: Vec<Field>,
        templates: Vec<Template>,
        css: Option<&str>,
        model_type: Option<ModelType>,
        latex_pre: Option<&str>,
        latex_post: Option<&str>,
        sort_field_index: Option<i64>,
    ) -> Self {
        let config = ModelConfig::default();
        Self {
            id,
            name: name.to_string(),
            fields,
            templates,
            css: css.unwrap_or(&config.css).to_string(),
            model_type: model_type.unwrap_or(ModelType::Basic),
            latex_pre: latex_pre.unwrap_or(config.latex_pre).to_string(),
            latex_post: latex_post.unwrap_or(config.latex_post).to_string(),
            sort_field_index: sort_field_index.unwrap_or(0),
        }
    }

    /// Add a field
    pub fn with_field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Add a template
    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.push(template);
        self
    }

    /// Set CSS
    pub fn css(mut self, css: impl ToString) -> Self {
        self.css = css.to_string();
        self
    }

    /// Set model type
    pub fn model_type(mut self, model_type: ModelType) -> Self {
        self.model_type = model_type;
        self
    }

    /// Set LaTeX preamble
    pub fn latex_pre(mut self, latex_pre: impl ToString) -> Self {
        self.latex_pre = latex_pre.to_string();
        self
    }

    /// Set LaTeX postscript
    pub fn latex_post(mut self, latex_post: impl ToString) -> Self {
        self.latex_post = latex_post.to_string();
        self
    }

    /// Set sort field index
    pub fn sort_field_index(mut self, sort_field_index: i64) -> Self {
        self.sort_field_index = sort_field_index;
        self
    }

    /// Get field names
    pub fn field_names(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.name.as_str()).collect()
    }

    /// Get number of fields
    pub fn num_fields(&self) -> usize {
        self.fields.len()
    }

    /// Get number of templates
    pub fn num_templates(&self) -> usize {
        self.templates.len()
    }

    /// Calculate required fields for each template
    pub fn req(&self) -> Result<Vec<(usize, String, Vec<usize>)>> {
        let sentinel = "SeNtInEl".to_string();
        let field_names: Vec<String> = self.fields.iter().map(|field| field.name.clone()).collect();
        let field_values: HashMap<&str, String> = field_names
            .iter()
            .map(|field| (field.as_str(), format!("{}{}", &field, &sentinel)))
            .collect();

        let mut req = Vec::new();

        for (template_ord, template) in self.templates.iter().enumerate() {
            let rendered = RamTemplate::new(template.qfmt.clone())?
                .render::<HashMap<&str, String>>(&field_values);

            // Try "all" first
            let required_fields: Vec<usize> = field_values
                .iter()
                .enumerate()
                .filter(|(_, (_, value))| !contains_other_fields(&rendered, value, &sentinel))
                .map(|(idx, _)| idx)
                .collect();

            if !required_fields.is_empty() {
                req.push((template_ord, "all".to_string(), required_fields));
                continue;
            }

            // Try "any"
            let required_fields: Vec<usize> = field_values
                .iter()
                .enumerate()
                .filter(|(_, (_, value))| rendered.contains(value.as_str()))
                .map(|(idx, _)| idx)
                .collect();

            if required_fields.is_empty() {
                return Err(Error::TemplateFormat(template.name.clone()));
            }

            req.push((template_ord, "any".to_string(), required_fields))
        }

        Ok(req)
    }
}

fn contains_other_fields(rendered: &str, current_field: &str, sentinel: &str) -> bool {
    let pattern = format!(r"(?!{field}\b)\b(\w)*{sentinel}+", field = current_field, sentinel = sentinel);
    Regex::new(&pattern)
        .unwrap()
        .is_match(rendered)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_new() {
        let model = Model::new(
            123,
            "Test Model",
            vec![Field::new("Question"), Field::new("Answer")],
            vec![Template::new("Card 1")],
        );
        assert_eq!(model.id, 123);
        assert_eq!(model.name, "Test Model");
        assert_eq!(model.num_fields(), 2);
    }

    #[test]
    fn test_model_with_options() {
        let model = Model::with_options(
            123,
            "Test Model",
            vec![Field::new("Question")],
            vec![Template::new("Card 1")],
            Some(".card { color: red; }"),
            Some(ModelType::Cloze),
            None,
            None,
            None,
        );
        assert_eq!(model.model_type, ModelType::Cloze);
        assert!(model.css.contains("red"));
    }

    #[test]
    fn test_field_builder() {
        let field = Field::new("Test")
            .font("Arial")
            .size(20)
            .rtl(true)
            .sticky(true);

        assert_eq!(field.font, Some("Arial".to_string()));
        assert_eq!(field.size, Some(20));
        assert_eq!(field.rtl, Some(true));
        assert_eq!(field.sticky, Some(true));
    }

    #[test]
    fn test_template_builder() {
        let template = Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt("{{Answer}}");

        assert_eq!(template.qfmt, "{{Question}}");
        assert_eq!(template.afmt, "{{Answer}}");
    }
}
