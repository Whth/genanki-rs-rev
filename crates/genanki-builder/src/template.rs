//! Template builder

use genanki_core::Template;

/// Builder for templates
pub struct TemplateBuilder {
    name: String,
    qfmt: Option<String>,
    afmt: Option<String>,
}

impl TemplateBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            qfmt: None,
            afmt: None,
        }
    }

    pub fn qfmt(mut self, qfmt: &str) -> Self {
        self.qfmt = Some(qfmt.to_string());
        self
    }

    pub fn afmt(mut self, afmt: &str) -> Self {
        self.afmt = Some(afmt.to_string());
        self
    }

    pub fn build(self) -> Template {
        Template {
            name: self.name,
            qfmt: self.qfmt.unwrap_or_default(),
            afmt: self.afmt.unwrap_or_default(),
        }
    }
}

pub struct TemplateDefaults;
