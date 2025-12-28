//! Built-in models integration tests

use genanki_rs_rev::{
    basic_model,
    basic_and_reversed_card_model,
    basic_optional_reversed_card_model,
    basic_type_in_the_answer_model,
    cloze_model,
    Model, Note, Error,
};

#[test]
fn test_basic_model() {
    let model = basic_model();
    assert_eq!(model.id, 1559383000);
    assert!(model.name.contains("Basic"));
    assert_eq!(model.num_fields(), 2);
    assert_eq!(model.num_templates(), 1);
}

#[test]
fn test_basic_model_generates_note() -> Result<(), Error> {
    let model = basic_model();
    let note = Note::new(model, vec!["Front", "Back"])?;
    assert_eq!(note.fields().len(), 2);
    Ok(())
}

#[test]
fn test_basic_and_reversed_card_model() {
    let model = basic_and_reversed_card_model();
    assert_eq!(model.id, 1485830179);
    assert!(model.name.contains("reversed"));
    assert_eq!(model.num_fields(), 2);
    assert_eq!(model.num_templates(), 2);
}

#[test]
fn test_basic_and_reversed_generates_two_cards() -> Result<(), Error> {
    let model = basic_and_reversed_card_model();
    let note = Note::new(model, vec!["Front", "Back"])?;
    // Should generate 2 cards (front->back and back->front)
    assert_eq!(note.cards().len(), 2);
    Ok(())
}

#[test]
fn test_basic_optional_reversed_card_model() {
    let model = basic_optional_reversed_card_model();
    assert_eq!(model.id, 1382232460);
    assert!(model.name.contains("optional"));
    assert_eq!(model.num_fields(), 3);
    assert_eq!(model.num_templates(), 2);
}

#[test]
fn test_basic_type_in_the_answer_model() {
    let model = basic_type_in_the_answer_model();
    assert_eq!(model.id, 1305534440);
    assert!(model.name.contains("type"));
    assert_eq!(model.num_fields(), 2);
}

#[test]
fn test_cloze_model() {
    let model = cloze_model();
    assert_eq!(model.id, 1122529321);
    assert!(model.name.contains("Cloze"));
    assert_eq!(model.num_fields(), 1);
}

#[test]
fn test_cloze_model_generates_note() -> Result<(), Error> {
    let model = cloze_model();
    let note = Note::new(model, vec!["Text with {{c1::cloze}} deletion."])?;
    assert_eq!(note.fields().len(), 1);
    Ok(())
}

#[test]
fn test_cloze_model_with_multiple_clozes() -> Result<(), Error> {
    let model = cloze_model();
    let note = Note::new(
        model,
        vec!["{{c1::First}} and {{c2::Second}} clozes."],
    )?;
    assert!(!note.cards().is_empty());
    Ok(())
}

#[test]
fn test_builtin_models_different_ids() {
    let basic = basic_model();
    let reversed = basic_and_reversed_card_model();
    let optional = basic_optional_reversed_card_model();
    let type_in = basic_type_in_the_answer_model();
    let cloze = cloze_model();

    // All models should have unique IDs
    let ids: Vec<i64> = vec![basic.id, reversed.id, optional.id, type_in.id, cloze.id];
    let unique_ids: std::collections::HashSet<i64> = ids.iter().copied().collect();
    assert_eq!(ids.len(), unique_ids.len());
}

#[test]
fn test_builtin_models_have_css() {
    let basic = basic_model();
    assert!(!basic.css.is_empty());
}

#[test]
fn test_all_builtin_models_clone() {
    let models: Vec<Model> = vec![
        basic_model(),
        basic_and_reversed_card_model(),
        basic_optional_reversed_card_model(),
        basic_type_in_the_answer_model(),
        cloze_model(),
    ];

    for model in models {
        let cloned = model.clone();
        assert_eq!(cloned.id, model.id);
        assert_eq!(cloned.name, model.name);
    }
}
