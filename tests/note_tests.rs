use genanki_rs_rev::{Error, Field, Model, Note, Template};
use genanki_rs_rev::constans::{APKG_COL, APKG_SCHEMA};
use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};
use tempfile::{NamedTempFile, TempPath};

fn write_to_db_setup(db_file: &TempPath) -> (Connection, f64, i64, std::ops::RangeFrom<usize>) {
    let conn = Connection::open(&db_file).unwrap();
    conn.execute_batch(APKG_SCHEMA).unwrap();
    conn.execute_batch(APKG_COL).unwrap();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    (conn, timestamp, 0, ((timestamp * 1000.0) as usize..))
}

#[test]
fn ok() {
    let my_model = Model::new(
        1376484377,
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![
            Template::new("Card 1")
                .qfmt("{{Question}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#),
        ],
    );
    let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires"]).unwrap();
    let db_file = NamedTempFile::new().unwrap().into_temp_path();
    let (mut conn, timestamp, deck_id, mut id_gen) = write_to_db_setup(&db_file);
    let transaction = conn.transaction().unwrap();
    my_note
        .write_to_db(&transaction, timestamp, deck_id, &mut id_gen)
        .unwrap();
    transaction.commit().unwrap();
}

#[test]
fn tags_new() {
    let _ = Note::new_with_options(
        Model::new(0, "test", vec![], vec![]),
        vec![],
        None,
        Some(vec!["foo", "bar", "baz"]),
        None,
    )
    .unwrap();
}

#[test]
#[should_panic]
fn tags_new_panic() {
    let _ = Note::new_with_options(
        Model::new(0, "test", vec![], vec![]),
        vec![],
        None,
        Some(vec!["fo o", "bar", "baz"]),
        None,
    )
    .unwrap();
}

#[test]
fn num_fields_equals_model_ok() {
    let model = Model::new(
        1894808898,
        "Test Model",
        vec![
            Field::new("Question"),
            Field::new("Answer"),
            Field::new("Extra"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Question}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#),
        ],
    );

    let note = Note::new(
        model,
        vec![
            "Capital of Germany",
            "Berlin",
            "Berlin was divided by a wall until 1989",
        ],
    )
    .unwrap();
    let db_file = NamedTempFile::new().unwrap().into_temp_path();
    let (mut conn, timestamp, deck_id, mut id_gen) = write_to_db_setup(&db_file);
    let transaction = conn.transaction().unwrap();
    note.write_to_db(&transaction, timestamp, deck_id, &mut id_gen)
        .unwrap();
    transaction.commit().unwrap();
}

#[test]
#[should_panic]
fn num_fields_less_than_model_panic() {
    let model = Model::new(
        1894808898,
        "Test Model",
        vec![
            Field::new("Question"),
            Field::new("Answer"),
            Field::new("Extra"),
        ],
        vec![
            Template::new("Card 1")
                .qfmt("{{Question}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#),
        ],
    );

    let note = Note::new(model, vec!["Capital of Germany", "Berlin"]).unwrap();
    let db_file = NamedTempFile::new().unwrap().into_temp_path();
    let (mut conn, timestamp, deck_id, mut id_gen) = write_to_db_setup(&db_file);
    let transaction = conn.transaction().unwrap();
    note.write_to_db(&transaction, timestamp, deck_id, &mut id_gen)
        .unwrap();
    transaction.commit().unwrap();
}

#[test]
#[should_panic]
fn num_fields_more_than_model_panic() {
    let model = Model::new(
        1894808898,
        "Test Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![
            Template::new("Card 1")
                .qfmt("{{Question}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#),
        ],
    );

    let note = Note::new(
        model,
        vec![
            "Capital of Germany",
            "Berlin",
            "Berlin was divided by a wall until 1989",
        ],
    )
    .unwrap();
    let db_file = NamedTempFile::new().unwrap().into_temp_path();
    let (mut conn, timestamp, deck_id, mut id_gen) = write_to_db_setup(&db_file);
    let transaction = conn.transaction().unwrap();
    note.write_to_db(&transaction, timestamp, deck_id, &mut id_gen)
        .unwrap();
    transaction.commit().unwrap();
}

fn find_invalid_html_tags_in_field(field: &str) -> Vec<String> {
    use fancy_regex::Regex;
    let regex = Regex::new(r"<(?!/?[a-z0-9]+(?: .*|/?)>)(?:.|\n)*?>").unwrap();
    regex
        .find_iter(field)
        .map(|m| m.unwrap().as_str().to_string())
        .collect()
}

#[test]
fn find_invalid_html_tags_in_field_ok() {
    assert_eq!(
        find_invalid_html_tags_in_field("<h1>"),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ok_with_space() {
    assert_eq!(
        find_invalid_html_tags_in_field(" <h1> "),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ok_multiple() {
    assert_eq!(
        find_invalid_html_tags_in_field("<h1>test</h1>"),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ok_br() {
    assert_eq!(
        find_invalid_html_tags_in_field("<br>"),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ok_br2() {
    assert_eq!(
        find_invalid_html_tags_in_field("<br/>"),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ok_br3() {
    assert_eq!(
        find_invalid_html_tags_in_field("<br />"),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ok_attrs() {
    assert_eq!(
        find_invalid_html_tags_in_field(r#"<h1 style="color: red">STOP</h1>"#),
        Vec::<String>::new()
    );
}

#[test]
fn find_invalid_html_tags_in_field_ng_empty() {
    assert_eq!(
        find_invalid_html_tags_in_field(" hello <> goodbye"),
        vec!["<>"]
    );
}

#[test]
fn find_invalid_html_tags_in_field_ng_empty_space() {
    assert_eq!(
        find_invalid_html_tags_in_field(" hello < > goodbye"),
        vec!["< >"]
    );
}

#[test]
fn find_invalid_html_tags_in_field_ng_invalid_characters() {
    assert_eq!(find_invalid_html_tags_in_field("<@h1>"), vec!["<@h1>"]);
}

#[test]
fn find_invalid_html_tags_in_field_ng_invalid_characters_end() {
    assert_eq!(find_invalid_html_tags_in_field("<h1@>"), vec!["<h1@>"]);
}

#[test]
fn option_builder() -> anyhow::Result<()> {
    // Make sure we can call the different builder-style methods on Note.
    // Doesn't actually verify any behavior though.
    let model = Model::new(
        1234,
        "model",
        vec![Field::new("a"), Field::new("b")],
        vec![Template::new("template")],
    );
    let _note = Note::new(model, vec!["a", "b"])?
        .guid("1234")
        .tags(["tag_a"])
        .with_tag("tag_b")
        .sort_field(true);

    Ok(())
}