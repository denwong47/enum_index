/// Error types for this crate.
///
use syn;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnumIndexError {
    // Build time errors
    #[error("parsing of tokens for `{0}` failed during build: {1:?}")]
    SynParseError(String, syn::parse::Error),

    // Runtime errors
    #[error("sample error for {0}.")]
    SampleError(String),

    #[error("unknown value error occured.")]
    Unknown,
}
