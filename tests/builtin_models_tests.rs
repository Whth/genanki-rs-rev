use genanki_rs_rev::{basic_and_reversed_card_model, basic_model, basic_optional_reversed_card_model, basic_type_in_the_answer_model, cloze_model, Deck, Note};
use tempfile::NamedTempFile;

#[test]
fn builtin_models() {
    let mut my_deck = Deck::new(1598559905, "Country Capitals", "");

    my_deck.add_note(
        Note::new(basic_model(), vec!["Capital of Argentina", "Buenos Aires"]).unwrap(),
    );
    my_deck.add_note(
        Note::new(
            basic_and_reversed_card_model(),
            vec!["Costa Rica", "San José"],
        )
        .unwrap(),
    );
    my_deck.add_note(
        Note::new(
            basic_optional_reversed_card_model(),
            vec!["France", "Paris", "y"],
        )
        .unwrap(),
    );
    my_deck.add_note(
        Note::new(basic_type_in_the_answer_model(), vec!["Taiwan", "Taipei"]).unwrap(),
    );
    my_deck.add_note(
        Note::new(
            cloze_model(),
            vec!["{{c1::Rome}} is the capital of {{c2::Italy}}"],
        )
        .unwrap(),
    );

    let out_file = NamedTempFile::new().unwrap().into_temp_path();
    my_deck.write_to_file(out_file.to_str().unwrap()).unwrap();
}