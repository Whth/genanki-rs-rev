# genanki-rs-rev

🔧 **A maintained fork of [genanki-rs](https://github.com/yannickfunk/genanki-rs) with updated dependencies.**

This repository is a continuation of the original [`genanki-rs`](https://github.com/yannickfunk/genanki-rs) project,
aimed at keeping the crate up-to-date with the latest Rust ecosystem dependencies and ensuring compatibility with modern
toolchains.

> ⚠️ **Note:** This is not an official fork. It is maintained independently to support users who need recent dependency
> versions.

A crate for easily generating flashcard decks for the popular open source flashcard platform Anki. It is based on the
code of genanki, a python library.

## Getting Started

To use genanki-rs, add with cargo

```bash
cargo add genanki-rs-rev
```

### Minimal Example

The following example creates a simple deck, containing 2 question-answer flashcards:

```rust
use genanki_rs_rev::{basic_model, Deck, Error, Note, Package};

fn main() -> Result<(), Error> {
    let mut deck = Deck::new(1234, "Example Deck", "Example Deck containing 2 Flashcards");
    deck.add_note(Note::new(basic_model(), vec!["What is the capital of France?", "Paris"])?);
    deck.add_note(Note::new(basic_model(), vec!["What is the capital of Germany?", "Berlin"])?);

    let package = Package::new(vec![deck], std::collections::HashMap::new())?;
    package.write_to_file("output.apkg")?;
    Ok(())
}
```

## Concepts

### Notes

The basic unit in Anki is the `Note`, which contains a fact to memorize. `Note`s correspond to one or more `Card`s.

Here's how you create a `Note`:

```rust
use genanki_rs_rev::{Note, Error, Model};

fn main() -> Result<(), Error> {
    // let my_model = ...
    let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires"])?;
    Ok(())
}
```

You pass in a `Model`, discussed below, and a set of `fields` (encoded as HTML).

### Models

A `Model` defines the fields and cards for a type of `Note`. For example:

```rust
use genanki_rs_rev::{Field, Model, Template, Error};

fn main() -> Result<(), Error> {
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );
    // let my_note = ...
    Ok(())
}
```

This note-type has two fields and one card. The card displays the
`Question` field on the front and the `Question` and `Answer` fields on the
back, separated by a `<hr>`. You can also pass custom `css` by calling
[`Model::css`] to supply custom CSS.

```rust
use genanki_rs_rev::{Field, Template, Model};
fn main() {
    let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
    let my_model_with_css = Model::new(
        1607392319,
        "Simple Model",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)])
        .css(custom_css);
}
```

You need to pass a model `id` and a model `name` so that Anki can keep track of your model. It's important that you use
a unique model `id`
for each `Model` you define.

### Generating a Deck/Package

To import your notes into Anki, you need to add them to a `Deck`:

```rust
use genanki_rs_rev::{Deck, Error, Note};

fn main() -> Result<(), Error> {
    let my_note = make_note();
    let mut my_deck = Deck::new(
        2059400110,
        "Country Capitals",
        "Deck for studying country capitals",
    );
    my_deck.add_note(my_note);
    Ok(())
}
```

Once again, you need a unique deck `id`, a deck `name` and a deck `description`.

Then, create a `Package` for your `Deck` and write it to a file:

```rust,ignore
let package = Package::new(vec![my_deck], std::collections::HashMap::new())?;
package.write_to_file("output.apkg")?;
```

You can then load `output.apkg` into Anki using File -> Import...

### Media Files

To add sounds or images, create a `Package` and pass the `decks` and `media_files` you want to include:

```rust
use genanki_rs_rev::{Deck, Error, Package, MediaFiles};

fn main() -> Result<(), Error> {
    // ...
    // my_deck.add(my_note)

    let mut media = MediaFiles::new();
    media.add("sound.mp3".to_string(), std::fs::read("sound.mp3")?);
    media.add("image.jpg".to_string(), std::fs::read("image.jpg")?);

    let package = Package::new(vec![my_deck], media.files().clone())?;
    package.write_to_file("output.apkg")?;
    Ok(())
}
```

To use media files in notes, first add a field to your model, and reference that field in your template:

```rust
use genanki_rs_rev::{Template, Field, Model};

fn main() {
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![
            Field::new("Question"),
            Field::new("Answer"),
            Field::new("MyMedia"),                           // ADD THIS
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}{{Question}}<br>{{MyMedia}}") // AND THIS
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );
}
```

Then, set the `MyMedia` field on your `Note` to `[sound:sound.mp3]` for audio and `<img src="image.jpg">` for images (
e.g):

```rust
use genanki_rs_rev::{Field, Template, Model, Error, Note};
fn main() -> Result<(), Error> {
    let my_model = Model::new(
        1607392319,
        "Simple Model",
        vec![
            Field::new("Question"),
            Field::new("Answer"),
            Field::new("MyMedia"),                           // ADD THIS
        ],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}{{Question}}<br>{{MyMedia}}") // AND THIS
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );
    let my_note = Note::new(my_model.clone(), vec!["Capital of Argentina", "Buenos Aires", "[sound:sound.mp3]"])?;
    // or
    let my_note = Note::new(my_model.clone(), vec!["Capital of Argentina", "Buenos Aires", r#"<img src="image.jpg">"#])?;
    Ok(())
}
```

You *cannot* put `<img src="{MyMedia}">` in the template and `image.jpg` in the field. See these sections in the Anki
manual for more information: [Importing Media](https://docs.ankiweb.net/#/importing?id=importing-media)
and [Media & LaTeX](https://docs.ankiweb.net/#/templates/fields?id=media-amp-latex).

You should only put the filename (aka basename) and not the full path in the field; `<img src="images/image.jpg">` will
*not* work. Media files should have unique filenames.

### sort_field

Anki has a value for each `Note` called the `sort_field`. Anki uses this
value to sort the cards in the Browse interface. Anki also is happier if
you avoid having two notes with the same `sort_field`, although this isn't
strictly necessary. By default, the `sort_field` is the first field, but
you can change it by calling [`Note::sort_field`].

You can also call [`Model::sort_field_index`], passing the
`sort_field_index` to change the sort field. `0` means the first field in
the Note, `1` means the second, etc.

## Advanced Usage

### Using Builder Pattern

The crate provides a builder pattern for more complex configurations:

```rust
use genanki_rs_rev::{DeckBuilder, ModelBuilder, NoteBuilder, FieldBuilder, TemplateBuilder, BasicModels, MediaFiles, Error};

fn main() -> Result<(), Error> {
    let model = ModelBuilder::new("Custom Model", 1234567890)
        .add_field(FieldBuilder::new("Front"))
        .add_field(FieldBuilder::new("Back"))
        .add_template(
            TemplateBuilder::new("Card 1")
                .qfmt("{{Front}}")
                .afmt(r#"{{FrontSide}}<hr id="answer">{{Back}}"#)
        )
        .css(".card { font-family: Arial; }")
        .build()?;

    let deck = DeckBuilder::new("My Deck", 9876543210)
        .description("A test deck")
        .add_model(model)
        .build();

    let note = NoteBuilder::new()
        .model(deck.models().first().unwrap().clone())
        .fields(vec!["Question", "Answer"])
        .build()?;

    let mut deck = deck;
    deck.add_note(note);

    let package = Package::new(vec![deck], std::collections::HashMap::new())?;
    package.write_to_file("my_deck.apkg")?;
    Ok(())
}
```

### Built-in Models

The crate includes several pre-defined models for common use cases:

- `basic_model()` - Simple front/back card
- `basic_and_reversed_card_model()` - Front/back with reversed card
- `basic_optional_reversed_card_model()` - Front/back with optional reversed
- `basic_type_in_the_answer_model()` - Type answer on back
- `cloze_model()` - Cloze deletion cards

```rust
use genanki_rs_rev::{basic_model, basic_and_reversed_card_model, cloze_model, Note, Error, Package, Deck};

fn main() -> Result<(), Error> {
    // Basic model
    let basic = basic_model();
    let note1 = Note::new(basic, vec!["Front", "Back"])?;

    // Basic and reversed
    let reversed = basic_and_reversed_card_model();
    let note2 = Note::new(reversed, vec!["Front", "Back"])?;

    // Cloze model
    let cloze = cloze_model();
    let note3 = Note::new(cloze, vec!["The capital of France is {{c1::Paris}}."])?;

    let mut deck = Deck::new(1, "Multi-model Deck", "Contains different card types");
    deck.add_note(note1);
    deck.add_note(note2);
    deck.add_note(note3);

    let package = Package::new(vec![deck], std::collections::HashMap::new())?;
    package.write_to_file("multi_model.apkg")?;
    Ok(())
}
```

## License

MIT License - see LICENSE file for details.
