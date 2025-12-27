//! Model database operations

use crate::schema::{FieldDbEntry, ModelDbEntry, TemplateDbEntry};
use genanki_core::{Model, ModelType};

/// Convert a core Model to a database entry
pub fn model_to_db_entry(model: &mut Model, timestamp: f64, deck_id: i64) -> ModelDbEntry {
    let model_type = match model.model_type {
        ModelType::Basic => 0,
        ModelType::Cloze => 1,
    };

    ModelDbEntry {
        vers: vec![],
        name: model.name.clone(),
        tags: vec![],
        did: deck_id,
        usn: -1,
        req: model.req().unwrap_or_default(),
        flds: model
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| FieldDbEntry {
                name: f.name.clone(),
                media: vec![],
                sticky: f.sticky.unwrap_or(false),
                rtl: f.rtl.unwrap_or(false),
                ord: i as i64,
                font: f
                    .font
                    .clone()
                    .unwrap_or_else(|| "Liberation Sans".to_string()),
                size: f.size.unwrap_or(20),
            })
            .collect(),
        sortf: model.sort_field_index,
        tmpls: model
            .templates
            .iter()
            .enumerate()
            .map(|(i, t)| TemplateDbEntry {
                name: t.name.clone(),
                qfmt: t.qfmt.clone(),
                did: None,
                bafmt: String::new(),
                afmt: t.afmt.clone(),
                ord: i as i64,
                bqfmt: String::new(),
            })
            .collect(),
        model_db_entry_mod: timestamp as i64,
        latex_post: model.latex_post.clone(),
        model_db_entry_type: model_type,
        id: model.id.to_string(),
        css: model.css.clone(),
        latex_pre: model.latex_pre.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genanki_core::Field;

    #[test]
    fn test_model_to_db_entry() {
        let mut model = Model::new(
            123,
            "Test",
            vec![Field::new("F1"), Field::new("F2")],
            vec![],
        );
        let entry = model_to_db_entry(&mut model, 0.0, 1);
        assert_eq!(entry.id, "123");
        assert_eq!(entry.flds.len(), 2);
    }
}
