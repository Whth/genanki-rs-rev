//! Deck integration tests

use genanki_rs_rev::{Deck, Error, Field, Model, Note, Template, basic_model};

#[test]
fn test_deck_new() {
    let deck = Deck::new(1234, "Test Deck", "Test Description");
    assert_eq!(deck.id, 1234);
    assert_eq!(deck.name, "Test Deck");
    assert_eq!(deck.description, "Test Description");
    assert!(deck.is_empty());
}

#[test]
fn test_deck_with_name() {
    let deck = Deck::new(1, "Original", "Desc").with_name("New Name");
    assert_eq!(deck.name, "New Name");
}

#[test]
fn test_deck_with_description() {
    let deck = Deck::new(1, "Name", "Original").with_description("New Description");
    assert_eq!(deck.description, "New Description");
}

#[test]
fn test_deck_add_note() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let note = Note::new(model, vec!["Q", "A"])?;
    deck.add_note(note);
    assert_eq!(deck.num_notes(), 1);
    Ok(())
}

#[test]
fn test_deck_add_multiple_notes() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    deck.add_note(Note::new(model.clone(), vec!["Q1", "A1"])?);
    deck.add_note(Note::new(model.clone(), vec!["Q2", "A2"])?);
    deck.add_note(Note::new(model, vec!["Q3", "A3"])?);
    assert_eq!(deck.num_notes(), 3);
    Ok(())
}

#[test]
fn test_deck_add_notes() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let notes = vec![
        Note::new(model.clone(), vec!["Q1", "A1"])?,
        Note::new(model.clone(), vec!["Q2", "A2"])?,
        Note::new(model, vec!["Q3", "A3"])?,
    ];
    deck.add_notes(notes);
    assert_eq!(deck.num_notes(), 3);
    Ok(())
}

#[test]
fn test_deck_models_tracked() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let note = Note::new(model.clone(), vec!["Q", "A"])?;
    deck.add_note(note);
    assert_eq!(deck.num_models(), 1);
    // Same model should not increase count
    let note2 = Note::new(model, vec!["Q2", "A2"])?;
    deck.add_note(note2);
    assert_eq!(deck.num_models(), 1);
    Ok(())
}

#[test]
fn test_deck_models_access() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let note = Note::new(model.clone(), vec!["Q", "A"])?;
    deck.add_note(note);
    let models = deck.models();
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, model.id);
    Ok(())
}

#[test]
fn test_deck_notes_access() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let note = Note::new(model.clone(), vec!["Q", "A"])?;
    deck.add_note(note.clone());
    let notes = deck.notes();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].fields()[0], note.fields()[0]);
    Ok(())
}

#[test]
fn test_deck_is_empty() {
    let deck = Deck::new(1234, "Test", "");
    assert!(deck.is_empty());
}

#[test]
fn test_deck_not_empty() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let note = Note::new(model, vec!["Q", "A"])?;
    deck.add_note(note);
    assert!(!deck.is_empty());
    Ok(())
}

#[test]
fn test_deck_num_notes() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    for i in 0..5 {
        deck.add_note(Note::new(
            model.clone(),
            vec![&format!("Q{}", i), &format!("A{}", i)],
        )?);
    }
    assert_eq!(deck.num_notes(), 5);
    Ok(())
}

#[test]
fn test_deck_num_models() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model1 = Model::new(
        100,
        "Model1",
        vec![Field::new("F")],
        vec![Template::new("C")],
    );
    let model2 = Model::new(
        200,
        "Model2",
        vec![Field::new("F")],
        vec![Template::new("C")],
    );

    deck.add_note(Note::new(model1, vec!["Q"])?);
    deck.add_note(Note::new(model2.clone(), vec!["Q"])?);
    deck.add_note(Note::new(model2, vec!["Q"])?);

    assert_eq!(deck.num_models(), 2);
    Ok(())
}

#[test]
fn test_deck_cloning() {
    let deck = Deck::new(1234, "Test", "Desc");
    let cloned = deck.clone();
    assert_eq!(cloned.id, deck.id);
    assert_eq!(cloned.name, deck.name);
    assert_eq!(cloned.description, deck.description);
}

#[test]
fn test_deck_models_items() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Test", "");
    let model = basic_model();
    let note = Note::new(model.clone(), vec!["Q", "A"])?;
    deck.add_note(note);
    let models = deck.models_items();
    assert!(models.contains_key(&model.id));
    Ok(())
}
