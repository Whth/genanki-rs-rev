use genanki_rs_rev::{Deck, Field, Model, ModelType, Note, Template};
use std::collections::HashSet;
use tempfile::NamedTempFile;

fn css() -> String {
    r#".card {
 font-family: arial;
 font-size: 20px;
 text-align: center;
 color: black;
 background-color: white;
}

.cloze {
 font-weight: bold;
 color: blue;
}
.nightMode .cloze {
 color: lightblue;
}
"#
    .to_owned()
}

fn cloze_model() -> Model {
    Model::new_with_options(
        998877661,
        "Cloze Model",
        vec![Field::new("Text"), Field::new("Extra")],
        vec![
            Template::new("My Cloze Card")
                .qfmt("{{cloze:Text}}")
                .afmt("{{cloze:Text}}<br>{{Extra}}"),
        ],
        Some(&css()),
        Some(ModelType::Cloze),
        None,
        None,
        None,
    )
}

fn multi_field_cloze_model() -> Model {
    Model::new_with_options(
        1047194615,
        "Multi Field Cloze Model",
        vec![Field::new("Text1"), Field::new("Text2")],
        vec![
            Template::new("Cloze")
                .qfmt("{{cloze:Text1}} and {{cloze:Text2}}")
                .afmt("{{cloze:Text1}} and {{cloze:Text2}}"),
        ],
        Some(&css()),
        Some(ModelType::Cloze),
        None,
        None,
        None,
    )
}

#[test]
fn cloze() {
    let mut notes = vec![];
    let model = cloze_model();
    assert!(matches!(model.model_type, ModelType::Cloze));

    // Question: NOTE ONE: [...]
    // Answer:   NOTE ONE: single deletion
    let fields = vec!["NOTE ONE: {{c1::single deletion}}", ""];
    let cloze_note = Note::new(model.clone(), fields).unwrap();
    let card_ord_set = cloze_note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<HashSet<i64>>();
    assert_eq!(card_ord_set.len(), 1);
    assert_eq!(*card_ord_set.get(&0).unwrap(), 0);
    notes.push(cloze_note);

    // Question: NOTE TWO: [...]              2nd deletion     3rd deletion
    // Answer:   NOTE TWO: **1st deletion**   2nd deletion     3rd deletion
    //
    // Question: NOTE TWO: 1st deletion       [...]            3rd deletion
    // Answer:   NOTE TWO: 1st deletion     **2nd deletion**   3rd deletion
    //
    // Question: NOTE TWO: 1st deletion       2nd deletion     [...]
    // Answer:   NOTE TWO: 1st deletion       2nd deletion   **3rd deletion**
    let fields = vec![
        "NOTE TWO: {{c1::1st deletion}} {{c2::2nd deletion}} {{c3::3rd deletion}}",
        "",
    ];
    let cloze_note = Note::new(model.clone(), fields).unwrap();
    let mut sorted = cloze_note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<Vec<i64>>();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1, 2]);
    notes.push(cloze_note);

    // Question: NOTE THREE: C1-CLOZE
    // Answer:   NOTE THREE: 1st deletion
    let fields = vec!["NOTE THREE: {{c1::1st deletion::C1-CLOZE}}", ""];
    let cloze_note = Note::new(model.clone(), fields).unwrap();
    let card_ord_set = cloze_note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<HashSet<i64>>();
    assert_eq!(card_ord_set.len(), 1);
    assert_eq!(*card_ord_set.get(&0).unwrap(), 0);
    notes.push(cloze_note);

    // Question: NOTE FOUR: [...] foo 2nd deletion bar [...]
    // Answer:   NOTE FOUR: 1st deletion foo 2nd deletion bar 3rd deletion
    let fields = vec![
        "NOTE FOUR: {{c1::1st deletion}} foo {{c2::2nd deletion}} bar {{c1::3rd deletion}}",
        "",
    ];
    let cloze_note = Note::new(model.clone(), fields).unwrap();
    let mut sorted = cloze_note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<Vec<i64>>();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1]);
    notes.push(cloze_note);

    let mut deck = Deck::new(0, "test", "");
    notes.iter().for_each(|note| deck.add_note(note.clone()));
    let out_file = NamedTempFile::new().unwrap().into_temp_path();
    deck.write_to_file(out_file.to_str().unwrap()).unwrap();
}

#[test]
fn cloze_multi_field() {
    let fields = vec![
        "{{c1::Berlin}} is the capital of {{c2::Germany}}",
        "{{c3::Paris}} is the capital of {{c4::France}}",
    ];
    let note = Note::new(multi_field_cloze_model(), fields).unwrap();
    let mut sorted = note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<Vec<i64>>();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1, 2, 3]);
}

#[test]
fn cloze_indices_do_not_start_at_1() {
    let fields = vec![
        "{{c2::Mitochondria}} are the {{c3::powerhouses}} of the cell",
        "",
    ];
    let note = Note::new(cloze_model(), fields).unwrap();
    let mut sorted = note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<Vec<i64>>();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![1, 2]);
}

#[test]
fn cloze_newlines_in_deletion() {
    let fields = vec![
        "{{c1::Washington, D.C.}} is the capital of {{c2::the\nUnited States\nof America}}",
        "",
    ];
    let note = Note::new(cloze_model(), fields).unwrap();
    let mut sorted = note
        .cards()
        .iter()
        .map(|card| card.ord())
        .collect::<Vec<i64>>();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0, 1]);
}

#[test]
fn build_all_fields() {
    // A simple test to make sure we can call all the setters on the builder.
    //
    // It doesn't actually verify any behavior, it's basically just a smoke test.
    Model::new(12345, "test model", vec![Field::new("front")], vec![])
        .with_template(Template::new("template"))
        .css(css())
        .latex_post("")
        .latex_pre("")
        .sort_field_index(1)
        .model_type(ModelType::FrontBack);
}
