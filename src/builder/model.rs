//! Model builder

use crate::core::{Field, Model, ModelType, Template};

/// Builder for models
pub struct ModelBuilder {
    id: i64,
    name: String,
    fields: Vec<Field>,
    templates: Vec<Template>,
    css: Option<String>,
    model_type: Option<ModelType>,
}

impl ModelBuilder {
    pub fn new(id: i64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            fields: Vec::new(),
            templates: Vec::new(),
            css: None,
            model_type: None,
        }
    }

    pub fn with_field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.push(template);
        self
    }

    pub fn with_templates(mut self, templates: Vec<Template>) -> Self {
        self.templates = templates;
        self
    }

    pub fn css(mut self, css: &str) -> Self {
        self.css = Some(css.to_string());
        self
    }

    pub fn model_type(mut self, model_type: ModelType) -> Self {
        self.model_type = Some(model_type);
        self
    }

    pub fn build(self) -> Model {
        Model::with_options(
            self.id,
            &self.name,
            self.fields,
            self.templates,
            self.css.as_deref(),
            self.model_type,
            None,
            None,
            None,
        )
    }
}

/// Factory for basic built-in models
pub struct BasicModels;

impl BasicModels {
    pub fn basic() -> Model {
        Model::with_options(
            1559383000,
            "Basic (genanki)",
            vec![
                Field::new("Front").font("Arial"),
                Field::new("Back").font("Arial"),
            ],
            vec![
                Template::new("Card 1")
                    .qfmt("{{Front}}")
                    .afmt("{{FrontSide}}\n\n<hr id=answer>\n\n{{Back}}"),
            ],
            Some(
                ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n",
            ),
            None,
            None,
            None,
            None,
        )
    }

    pub fn cloze() -> Model {
        Model::with_options(
            1122529321,
            "Cloze (genanki)",
            vec![Field::new("Text").font("Arial")],
            vec![
                Template::new("Cloze")
                    .qfmt("{{cloze:Text}}")
                    .afmt("{{cloze:Text}}"),
            ],
            Some(
                ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n background-color: white;\n}\n\n .cloze {\n font-weight: bold;\n color: blue;\n}\n.nightMode .cloze {\n color: lightblue;\n}",
            ),
            Some(ModelType::Cloze),
            None,
            None,
            None,
        )
    }
}
