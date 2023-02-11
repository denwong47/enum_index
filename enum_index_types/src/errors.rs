/// Error types for this crate.
///
use syn;
use thiserror::Error;

/// A enum of possible error types caused during [`EnumIndex`] macro expansion.
///
/// [`EnumIndex`]: ../../enum_index_macros/derive.EnumIndex.html
///
/// During macro expansion, no Result<> can be returned; hence the actual variant used
/// do not matter as much - they will panic in the same way.
#[derive(Error, Debug)]
pub enum EnumIndexError {
    // Build time errors
    #[error("the meta attribute `index_type` is missing from Enum `{0}`. All uses of #[derive(EnumIndex)] must be accompanied by an associated #[index_type(T)] meta, where T is the shared type among all indices for members.")]
    ParamsMetaNotFound(String),

    #[error("`{0}` must be derived on an enum only.")]
    NotAnEnum(String),

    #[error("`{0}` is not a valid literal for #[index(value)].")]
    IndexNotALiteral(String),

    #[error("variant `{0}` was not provided a value. State a value for this variant with #[index(value)].")]
    IndexNotGiven(String),

    #[error("parsing of tokens for `{0}` failed during build: {1:?}")]
    SynParseError(String, syn::parse::Error),

    // Runtime errors
    #[error("the index `{0}` is not available in this Enum.")]
    IndexNotFound(String),

    #[error("unknown error occured.")]
    Unknown,
}
