//! Field builder with defaults

use crate::core::{Field, config::FieldDefaults as CoreFieldDefaults};

/// Builder for fields with enhanced API
pub struct FieldBuilder {
    name: String,
    font: Option<String>,
    size: Option<i64>,
    rtl: Option<bool>,
    sticky: Option<bool>,
}

impl FieldBuilder {
    /// Create a new field builder
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

    /// Apply defaults
    pub fn with_defaults(self) -> Field {
        let defaults = CoreFieldDefaults::default();
        Field {
            name: self.name,
            font: Some(self.font.unwrap_or(defaults.font.to_string())),
            size: Some(self.size.unwrap_or(defaults.size)),
            rtl: Some(self.rtl.unwrap_or(defaults.rtl)),
            sticky: Some(self.sticky.unwrap_or(defaults.sticky)),
        }
    }

    /// Build with current values (None becomes default)
    pub fn build(self) -> Field {
        self.with_defaults()
    }
}

/// Constants for default field values
pub struct FieldDefaultsConstants;

impl FieldDefaultsConstants {
    pub const FONT: &'static str = "Liberation Sans";
    pub const SIZE: i64 = 20;
    pub const RTL: bool = false;
    pub const STICKY: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_builder() {
        let field = FieldBuilder::new("Test").font("Arial").size(30).build();

        assert_eq!(field.name, "Test");
        assert_eq!(field.font, Some("Arial".to_string()));
        assert_eq!(field.size, Some(30));
    }
}
