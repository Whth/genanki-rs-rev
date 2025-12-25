//! A crate for easily generating flashcard decks for the popular open source flashcard platform Anki. It is based on the code of genanki, a python library.
//!
//! # Getting Started
//!
//! To use genanki-rs, add the following to your `Cargo.toml`
//! ```toml
//! genanki-rs = "0.4"
//! ```
//! ## Minimal Example
//! The following example creates a simple deck, containing 2 question-answer flashcards:
//! ```rust
//! use genanki_rs::{basic_model, Deck, Error, Note};
//!
//! fn main() -> Result<()> {
//!     let mut deck = Deck::new(1234, "Example Deck", "Example Deck containing 2 Flashcards");
//!     deck.add_note(Note::new(basic_model(), vec!["What is the capital of France?", "Paris"])?);
//!     deck.add_note(Note::new(basic_model(), vec!["What is the capital of Germany?", "Berlin"])?);
//!     deck.write_to_file("output.apkg")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Concepts
//!
//! ### Notes
//! The basic unit in Anki is the `Note`, which contains a fact to memorize. `Note`s correspond to one or more `Card`s.
//!
//! Here's how you create a `Note`:
//!
//! ```rust,ignore
//! use genanki_rs::{Note, Error};
//!
//! fn main() -> Result<()> {
//!     // let my_model = ...
//!     let my_note = Note::new(my_model, vec!["Capital of Argentina", "Buenos Aires"])?;
//!     Ok(())
//! }
//! ```
//!
//! You pass in a `Model`, discussed below, and a set of `fields` (encoded as HTML).
//!
//! ### Models
//! A `Model` defines the fields and cards for a type of `Note`. For example:
//!
//! ```rust
//! use genanki_rs::{Field, Model, Template, Error};
//!
//! fn main() -> Result<()> {
//!     let my_model = Model::new(
//!         1607392319,
//!         "Simple Model",
//!         vec![Field::new("Question"), Field::new("Answer")],
//!         vec![Template::new("Card 1")
//!             .qfmt("{{Question}}")
//!             .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
//!     );
//!     // let my_note = ...
//!     Ok(())
//! }
//! ```
//!
//! This note-type has two fields and one card. The card displays the
//! `Question` field on the front and the `Question` and `Answer` fields on the
//! back, separated by a `<hr>`. You can also pass custom `css` by calling
//! [`Model::css`] to supply custom CSS.
//!
//! ```rust
//! # use genanki_rs::{Field, Template, Model};
//! let custom_css = ".card {\n font-family: arial;\n font-size: 20px;\n text-align: center;\n color: black;\n}\n";
//! let my_model_with_css = Model::new(
//!     1607392319,
//!     "Simple Model",
//!     vec![Field::new("Question"), Field::new("Answer")],
//!     vec![Template::new("Card 1")
//!         .qfmt("{{Question}}")
//!         .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)])
//!     .css(custom_css);
//! ```
//!
//! You need to pass a model `id` and a model `name` so that Anki can keep track of your model. It's important that you use a unique model `id`
//! for each `Model` you define.
//!
//! ### Generating a Deck/Package
//! To import your notes into Anki, you need to add them to a `Deck`:
//!
//! ```rust,no_run
//! use genanki_rs::{Deck, Error};
//! # use genanki_rs::Note;
//! # fn make_note() -> Note { todo!() }
//!
//! fn main() -> Result<()> {
//!     let my_note = make_note();
//!     let mut my_deck = Deck::new(
//!         2059400110,
//!         "Country Capitals",
//!         "Deck for studying country capitals",
//!     );
//!     my_deck.add_note(my_note);
//!     Ok(())
//! }
//! ```
//!
//! Once again, you need a unique deck `id`, a deck `name` and a deck `description`.
//!
//! Then, create a `Package` for your `Deck` and write it to a file:
//!
//! ```rust,ignore
//! my_deck.write_to_file("output.apkg")?;
//! ```
//!
//! You can then load `output.apkg` into Anki using File -> Import...
//!
//! ### Media Files
//! To add sounds or images, create a `Package` and pass the `decks` and `media_files` you want to include:
//!
//! ```rust,ignore
//! use genanki_rs::{Deck, Error, Package};
//!
//! fn main() -> Result<()> {
//!     // ...
//!     // my_deck.add(my_note)
//!     let mut my_package = Package::new(vec![my_deck], vec!["sound.mp3", "images/image.jpg"])?;
//!     my_package.write_to_file("output.apkg")?;
//!     Ok(())
//! }
//! ```
//!
//! `media_files` should have the path (relative or absolute) to each file. To use them in notes, first add a field to your model, and reference that field in your template:
//!
//! ```rust
//! # use genanki_rs::{Template, Field, Model};
//! let my_model = Model::new(
//!     1607392319,
//!     "Simple Model",
//!     vec![
//!         Field::new("Question"),
//!         Field::new("Answer"),
//!         Field::new("MyMedia"),                           // ADD THIS
//!     ],
//!     vec![Template::new("Card 1")
//!         .qfmt("{{Question}}{{Question}}<br>{{MyMedia}}") // AND THIS
//!         .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
//! );
//! ```
//!
//! Then, set the `MyMedia` field on your `Note` to `[sound:sound.mp3]` for audio and `<img src="image.jpg">` for images (e.g):
//!
//! ```rust
//! # use genanki_rs::{Field, Template, Model, Error, Note};
//! # fn main() -> Result<()> {
//! # let my_model = Model::new(
//! #    1607392319,
//! #    "Simple Model",
//! #    vec![
//! #        Field::new("Question"),
//! #        Field::new("Answer"),
//! #        Field::new("MyMedia"),                           // ADD THIS
//! #    ],
//! #    vec![Template::new("Card 1")
//! #        .qfmt("{{Question}}{{Question}}<br>{{MyMedia}}") // AND THIS
//! #        .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
//! # );
//! let my_note = Note::new(my_model.clone(), vec!["Capital of Argentina", "Buenos Aires", "[sound:sound.mp3]"])?;
//! // or
//! let my_note = Note::new(my_model.clone(), vec!["Capital of Argentina", "Buenos Aires", r#"<img src="image.jpg">"#])?;
//! # Ok(())
//! # }
//! ```
//!
//! You *cannot* put `<img src="{MyMedia}">` in the template and `image.jpg` in the field. See these sections in the Anki manual for more information: [Importing Media](https://docs.ankiweb.net/#/importing?id=importing-media) and [Media & LaTeX](https://docs.ankiweb.net/#/templates/fields?id=media-amp-latex).
//!
//! You should only put the filename (aka basename) and not the full path in the field; `<img src="images/image.jpg">` will *not* work. Media files should have unique filenames.
//!
//! ### sort_field
//! Anki has a value for each `Note` called the `sort_field`. Anki uses this
//! value to sort the cards in the Browse interface. Anki also is happier if
//! you avoid having two notes with the same `sort_field`, although this isn't
//! strictly necessary. By default, the `sort_field` is the first field, but
//! you can change it by calling [`Note::sort_field`].
//!
//! You can also call [`Model::sort_field_index`], passing the
//! `sort_field_index` to change the sort field. `0` means the first field in
//! the Note, `1` means the second, etc.
//!

mod builders;
mod builtin_models;
mod card;
mod db_entries;
mod deck;
mod error;
mod model;
mod note;
mod package;
mod util;
pub mod constants;

pub use builders::{Field, Template};
pub use builtin_models::*;
pub use deck::Deck;
pub use error::{Error, Result};
pub use model::{Model, ModelType};
pub use note::Note;
pub use package::Package;
