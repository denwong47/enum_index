//! Traits module.
//!
//! All Indices must be unique, and capable of `eq()`.
//! The trait and the macro does NOT check indices for equality - it is down to the
//! implementer to ensure that none of the indices are == another.
//!
//! .. note::
//!     This is currently not in use due to complications for `String` - you will want
//!     to `index(&self)` into `String` but `from_index(index: &str)` instead, which
//!     contradicts what this trait defined.

/// Marker Trait only.
pub trait EnumIndex {}
