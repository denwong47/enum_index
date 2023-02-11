//! This library provides a simple derive macro to map Enum variants to add methods for bi-directional mapping between Enum variants and their respective values..
//!
//! Example
//! -------
//!
//! ```rust
//! use enum_index::*;
//!
//!
//! #[derive(EnumIndex)]
//! struct MyStruct {
//!     _populate_me: i32,
//! }
//!
//! assert!(
//!     // Write a test here
//!     true
//! );
//!
//! ```
//!
pub mod prelude;

pub use enum_index_macros::*;
pub use enum_index_types::*;

mod tests;
