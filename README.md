# EnumIndex derive macro

![CI Checks](https://github.com/denwong47/enum_index/actions/workflows/CI.yml/badge.svg?branch=main)

This library provides a simple derive macro to add methods for bi-directional mapping between Enum variants and their respective values.

While Rust provides C-Like enums, they are only available for `isize` values.

To allow this for other types, one can:

- `impl` a [`From`] and [`TryFrom`] both ways, `match`ing values back and forth.
  This however duplicates the values in both `impl`, reducing maintainability.
- use [`EnumIter`] derive macro on the enum, and perform a `.iter().find()` during
  lookup. However due to it being an Iterator, this will have to be recalculated
  everytime [`TryFrom`] is used on the enum, and carries a performance penalty.

[`EnumIter`]: https://docs.rs/strum/latest/strum/derive.EnumIter.html
[`.iter().find()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find

The [`EnumIndex`] derive macro takes a special syntax during enum declaration,
then expands the code to `impl` [`From`] and [`TryFrom`] automatically.

<br>

Example
-------

```rust
use enum_index::*;

type OptionalChar = Option<char>;

#[derive(Debug, EnumIndex, PartialEq)]
#[index_type(OptionalChar)]
enum SpecialChar {
    #[index(Some('\0'))]
    Null,

    #[index(Some('\t'))]
    Tab,

    #[index(Some('\n'))]
    CarriageReturn,

    #[index(None)]
    Nothing,
}

// Variant to value - guaranteed success
assert_eq!(
    SpecialChar::Null.value(),
    Some('\0'),
);

// Look for an existing variant
assert_eq!(
    // Returns a Result<SpecialChar, EnumIndexError>
    SpecialChar::try_from(&None).unwrap(),
    SpecialChar::Nothing
);

// Look for a missing variant
assert!(
    // Returns a Result<SpecialChar, EnumIndexError>
    SpecialChar::try_from(&Some('A')).is_err()
);
//!
```

In order for this to work, the following requirements need to be met:

- The enum must have the `#[index_type(OptionalChar)]` attribute, annotating the
  value type applicable to ALL values to follow.

    - the derive macro does not check the validity of this; the error reported will
      be from a `return` value being the wrong type.
    - the value type shall be a simple identifier. If you intend to use a complex
      type such as `Result` or `Option` as shown in the example above, use a `type`
      alias as per the example.

- Each variant MUST have a `#[index()]` attribute, which contains a valid expression.
  These values must be unique.

    - the derive macro does not check the validity of this;

        - in case of duplicated literal values, a `match` condition conflict
          might occur,
        - in case of expressions resulting in the same value,
          only the first variant of that value will be returned by `try_from`.
    - it is strongly recommended to keep these value expressions as simpple as
      possible.
    - note that `String` type is a special case; you can declare the values as
      `str` literals without the need of `.to_owned()` or `.to_string()`.
