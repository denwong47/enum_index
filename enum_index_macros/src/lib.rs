//! This crate contains macros for export to [`enum_index`].
//!
//! [`enum_index`]: ../enum_index
//!
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{parse_macro_input, DeriveInput};

mod func;

/// A simple derive macro to add methods for bi-directional mapping between Enum variants and their respective values..
///
/// Example
/// -------
///
/// ```ignore
/// use enum_index::*;
///
/// #[derive(EnumIndex)]
/// pub struct MyStruct {
///     _populate_me: i32,
/// }
/// ```
///
/// will Expand into:
///
/// ```ignore
/// use enum_index::EnumIndex;
///
/// // POPULATE ME
/// ```
#[proc_macro_derive(EnumIndex, attributes(__change_me__))]
pub fn enum_index(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // =================================================================================
    // Build the output
    let expanded = quote! {
        use enum_index_types::EnumIndex;
        impl enum_index_types::EnumIndex for #name {

            /// Sample function that always return a fixed value.
            fn test_enum_index(&self) -> i32 {
                42069
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
