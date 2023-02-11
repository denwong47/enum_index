//! This library provides a simple derive macro to map Enum variants to static indices in both directions.
//!
//! While Rust provides C-Like enums, they are only available for `isize` indices.
//!
//! To allow this for other types, one can:
//!
//! - `impl` a [`From`] and [`TryFrom`] both ways, `match`ing indices back and forth.
//!   This however duplicates the indices in both `impl`, reducing maintainability.
//! - use [`EnumIter`] derive macro on the enum, and perform a `.iter().find()` during
//!   lookup. However due to it being an Iterator, this will have to be recalculated
//!   everytime [`TryFrom`] is used on the enum, and carries a performance penalty.
//!
//! [`EnumIter`]: https://docs.rs/strum/latest/strum/derive.EnumIter.html
//! [`.iter().find()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
//!
//! The [`EnumIndex`] derive macro takes a special syntax during enum declaration,
//! then expands the code to `impl` [`From`] and [`TryFrom`] automatically.
//!
//! <br>
//!
//! Example
//! -------
//!
//! ```rust
//! use enum_index::*;
//!
//! type OptionalChar = Option<char>;
//!
//! #[derive(Debug, EnumIndex, PartialEq)]
//! #[index_type(OptionalChar)]
//! enum SpecialChar {
//!     #[index(Some('\0'))]
//!     Null,
//!
//!     #[index(Some('\t'))]
//!     Tab,
//!
//!     #[index(Some('\n'))]
//!     CarriageReturn,
//!
//!     #[index(None)]
//!     Nothing,
//! }
//!
//! // Variant to index - guaranteed success
//! assert_eq!(
//!     SpecialChar::Null.index(),
//!     Some('\0'),
//! );
//!
//! // Look for an existing variant
//! assert_eq!(
//!     // Returns a Result<SpecialChar, EnumIndexError>
//!     SpecialChar::try_from(&None).unwrap(),
//!     SpecialChar::Nothing
//! );
//!
//! // Look for a missing variant
//! assert!(
//!     // Returns a Result<SpecialChar, EnumIndexError>
//!     SpecialChar::try_from(&Some('A')).is_err()
//! );
//!
//! ```
//!
//! In order for this to work, the following requirements need to be met:
//!
//! - `enum_index::errors::EnumIndexError` also needs to be imported manually.
//!
//!     - Recommended to import via `use enum_index::*;` to avoid any future
//!       compaibility problems.
//!
//! - The enum must have the `#[index_type(OptionalChar)]` attribute, annotating the
//!   index type applicable to ALL indices to follow.
//!
//!     - the derive macro does not check the validity of this; the error reported will
//!       be from a `return` index being the wrong type.
//!     - the index type shall be a simple identifier. If you intend to use a complex
//!       type such as `Result` or `Option` as shown in the example above, use a `type`
//!       alias as per the example.
//!
//! - Each variant MUST have a `#[index()]` attribute, which contains a valid expression.
//!   These indices must be unique.
//!
//!     - the derive macro does not check the validity of this;
//!
//!         - in case of duplicated literal indices, a `match` condition conflict
//!           might occur,
//!         - in case of expressions resulting in the same index,
//!           only the first variant of that index will be returned by `try_from`.
//!     - it is strongly recommended to keep these index expressions as simpple as
//!       possible.
//!     - note that `String` type is a special case; you can declare the indices as
//!       `str` literals without the need of `.to_owned()` or `.to_string()`.
//!
pub mod prelude;

pub use enum_index_macros::*;
pub use enum_index_types::*;

mod tests;
