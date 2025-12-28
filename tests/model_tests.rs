//! Model integration tests

use genanki_rs_rev::{Field, Model, Template, Error};
use genanki_rs_rev::core::ModelType;

#[test]
fn test_model_creation() {
    let model = Model::new(
        1234567890,
        "Test Model",
        vec![Field::new("Front"), Field::new("Back")],
        vec![Template::new("Card 1")
            .qfmt("{{Front}}")
            .afmt("{{Back}}")],
    );
    assert_eq!(model.id, 1234567890);
    assert_eq!(model.name, "Test Model");
    assert_eq!(model.num_fields(), 2);
    assert_eq!(model.num_templates(), 1);
}

#[test]
fn test_model_with_css() {
    let model = Model::new(
        123,
        "Test",
        vec![Field::new("F"), Field::new("B")],
        vec![Template::new("C")],
    );
    let custom_css = ".card { background: red; }";
    let model_with_css = model.css(custom_css);
    assert!(model_with_css.css.contains("red"));
}

#[test]
fn test_model_with_multiple_templates() {
    let model = Model::new(
        123,
        "Multi-card",
        vec![Field::new("Front"), Field::new("Back")],
        vec![
            Template::new("Card 1")
                .qfmt("{{Front}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Back}}"#),
            Template::new("Card 2")
                .qfmt("{{Back}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Front}}"#),
        ],
    );
    assert_eq!(model.num_templates(), 2);
}

#[test]
fn test_model_num_fields() {
    let model = Model::new(
        1,
        "Test",
        vec![Field::new("F1"), Field::new("F2"), Field::new("F3")],
        vec![Template::new("C")],
    );
    assert_eq!(model.num_fields(), 3);
}

#[test]
fn test_model_field_names() {
    let model = Model::new(
        1,
        "Test",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("C")],
    );
    let field_names = model.field_names();
    assert_eq!(field_names, vec!["Question", "Answer"]);
}

#[test]
fn test_model_template_names() {
    let model = Model::new(
        1,
        "Test",
        vec![Field::new("F"), Field::new("B")],
        vec![
            Template::new("Front->Back"),
            Template::new("Back->Front"),
        ],
    );
    let template_names: Vec<&str> = model.templates.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(template_names, vec!["Front->Back", "Back->Front"]);
}

#[test]
fn test_model_cloning() {
    let model = Model::new(
        1,
        "Test",
        vec![Field::new("F"), Field::new("B")],
        vec![Template::new("C")],
    );
    let cloned = model.clone();
    assert_eq!(cloned.id, model.id);
    assert_eq!(cloned.name, model.name);
}

#[test]
fn test_model_with_options() {
    let model = Model::with_options(
        9876543210,
        "Options Model",
        vec![Field::new("F"), Field::new("B")],
        vec![Template::new("C").qfmt("{{F}}").afmt("{{B}}")],
        Some(".card { font-size: 20px; }"),
        Some(ModelType::Basic),
        None,
        None,
        None,
    );
    assert_eq!(model.id, 9876543210);
    assert!(model.css.contains("20px"));
}

#[test]
fn test_model_required_fields() -> Result<(), Error> {
    let model = Model::new(
        1,
        "Test",
        vec![Field::new("F1"), Field::new("F2")],
        vec![Template::new("C")],
    );
    let req = model.req()?;
    // Model should have some required fields
    assert!(!req.is_empty());
    Ok(())
}

#[test]
fn test_model_with_field() {
    let model = Model::new(1, "Test", vec![], vec![])
        .with_field(Field::new("Field1"))
        .with_field(Field::new("Field2"));
    assert_eq!(model.num_fields(), 2);
}

#[test]
fn test_model_with_template() {
    let model = Model::new(1, "Test", vec![Field::new("F")], vec![])
        .with_template(Template::new("Card 1").qfmt("{{F}}").afmt("{{F}}"));
    assert_eq!(model.num_templates(), 1);
}

#[test]
fn test_model_latex_settings() {
    let model = Model::with_options(
        1,
        "Test",
        vec![Field::new("F")],
        vec![Template::new("C")],
        None,
        None,
        Some("% preamble"),
        Some("% postamble"),
        None,
    );
    assert!(model.latex_pre.contains("preamble"));
    assert!(model.latex_post.contains("postamble"));
}

#[test]
fn test_model_sort_field_index() {
    let model = Model::with_options(
        1,
        "Test",
        vec![Field::new("F1"), Field::new("F2"), Field::new("F3")],
        vec![Template::new("C")],
        None,
        None,
        None,
        None,
        Some(2),
    );
    assert_eq!(model.sort_field_index, 2);
}

#[test]
fn test_model_type_basic() {
    let model = Model::with_options(
        1,
        "Test",
        vec![Field::new("F")],
        vec![Template::new("C")],
        None,
        Some(ModelType::Basic),
        None,
        None,
        None,
    );
    assert_eq!(model.model_type, ModelType::Basic);
}

#[test]
fn test_model_type_cloze() {
    let model = Model::with_options(
        1,
        "Test",
        vec![Field::new("F")],
        vec![Template::new("C")],
        None,
        Some(ModelType::Cloze),
        None,
        None,
        None,
    );
    assert_eq!(model.model_type, ModelType::Cloze);
}

#[test]
fn test_field_with_font() {
    let field = Field::new("Test").font("Arial");
    assert_eq!(field.font, Some("Arial".to_string()));
}

#[test]
fn test_field_with_size() {
    let field = Field::new("Test").size(20);
    assert_eq!(field.size, Some(20));
}

#[test]
fn test_field_with_rtl() {
    let field = Field::new("Test").rtl(true);
    assert_eq!(field.rtl, Some(true));
}

#[test]
fn test_field_with_sticky() {
    let field = Field::new("Test").sticky(true);
    assert_eq!(field.sticky, Some(true));
}

#[test]
fn test_template_qfmt_afmt() {
    let template = Template::new("Card 1")
        .qfmt("{{Question}}")
        .afmt("{{Answer}}");
    assert_eq!(template.qfmt, "{{Question}}");
    assert_eq!(template.afmt, "{{Answer}}");
}

#[test]
fn test_template_name() {
    let template = Template::new("My Template");
    assert_eq!(template.name, "My Template");
}
