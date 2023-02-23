//! This is where [`EnumIndex`] is actually declared.
//!
//! This crate is a `proc-macro` library, hence cannot export anything other than the
//! macro itself.
//!
//! Structs declared in this crate is for [`syn::parse::Parse`] use; it does not need to
//! be exported.
//!
//! [`syn::parse::Parse`]: https://docs.rs/syn/latest/syn/parse/trait.Parse.html

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;
use syn::{parse_macro_input, DeriveInput};

mod func;
mod structs;
use structs::IndexAttribute;

/// A simple derive macro to map static indexs of a single, consistent type to Rust enum
/// variants.
///
/// Example
/// -------
///
/// ```ignore
/// use enum_index::EnumIndex;
///
/// #[derive(EnumIndex)]
/// #[index_type(String)]
/// pub enum MyEnum {
///     #[index("Member 0")]
///     MEMBER0,
///     #[index("Member 1")]
///     MEMBER1,
/// }
/// ```
///
/// will Expand into:
///
/// ```ignore
/// use enum_index::EnumIndex;
/// use enum_index_types::errors;
///
/// pub enum MyEnum {
///     MEMBER0,
///     MEMBER1,
/// }
/// impl MyEnum {
///     fn index(&self) -> String {
///         match self {
///             Self::MEMBER0 => "Member 0".to_string(),
///             Self::MEMBER1 => "Member 1".to_string(),
///         }
///     }
///     fn from_index(index: &str) -> Option<Self> {
///         match index {
///             "Member 0" => Some(Self::MEMBER0),
///             "Member 1" => Some(Self::MEMBER1),
///             _ => None,
///         }
///     }
/// }
/// impl From<MyEnum> for String {
///     fn from(index: MyEnum) -> Self {
///         index.index()
///     }
/// }
/// impl TryFrom<&str> for MyEnum {
///     type Error = errors::EnumIndexError;
///     fn try_from(index: &str) -> Result<Self, Self::Error> {
///         Self::from_index(&index)
///             .ok_or(
///                 errors::EnumIndexError::IndexNotFound({
///                     let res = ::alloc::fmt::format(format_args!("{0:?}", index));
///                     res
///                 }),
///             )
///     }
/// }
/// ```
///
/// This allows quick look up from both ways without any use of iteration, while keeping
/// boilerplates to a minimum.
///
/// Works with typical primitive types such as `u32`, `usize`, `i64`, `char` as well as
/// `String`. `&str` is supported by `String`; simply declare the type as `String` and use
/// `str` literals for indexs.
///
/// Complex types such as `Option<usize>` will need to be aliased in `#[index_type(T)]`.
#[proc_macro_derive(EnumIndex, attributes(index_type, index))]
pub fn enum_index(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // =================================================================================
    // Parsing `#[index_type(T)]`

    // Find the `#[index_type(T)]` supplied to us.
    // Let it panic if there is one.
    let params = {
        structs::EnumIndexParams::try_from(&input)
            .map_err(
                // Because of thiserror, our error message is pretty nice actually.
                |err| {
                    panic!("{}", err);
                },
            )
            .unwrap()
    };

    let enum_data = {
        func::get_enum_data(&input).unwrap_or_else(
            // Because of thiserror, our error message is pretty nice actually.
            |err| panic!("{}", err),
        )
    };
    let return_type = params.return_type_specified();
    let return_type_ref = params.return_type_ref();

    // =================================================================================
    // Resolving #[index(index)]:

    let (match_to_index, match_from_index, match_from_name) = {
        enum_data.variants.iter().map(|variant| {
            (
                variant,
                IndexAttribute::try_from(variant).unwrap_or_else(|err| panic!("{}", err)),
            )
        })
    }
    .map(
        // Now that we know we have a index for each variant, lets roll them together
        // in to a single TokenStream.
        |(variant, index_attribute)| {
            let variant_name = &variant.ident;
            let index = index_attribute.expr();

            (
                // match_to_index
                if params.is_type("String") && func::is_lit_str(index) {
                    quote!(
                        Self::#variant_name=>#index.to_string(),
                    )
                } else {
                    quote!(
                        Self::#variant_name=>#index,
                    )
                },
                // match_from_index
                quote!(
                    #index=>Some(Self::#variant_name),
                ),
                // match_from_name
                {
                    let ident_str = format_ident!("{}", variant_name).to_string();
                    let ident_ref = &ident_str;
                    quote!(
                        #ident_ref=>Some(Self::#variant_name),
                    )
                },
            )
        },
    )
    .fold((quote!(), quote!(), quote!()), |mut lhs, rhs| {
        lhs.0.extend(rhs.0);
        lhs.1.extend(rhs.1);
        lhs.2.extend(rhs.2);
        lhs
    });

    let name = &input.ident;

    // =================================================================================
    // Build the output
    let expanded = quote! {
        impl #name {
            fn index(&self) -> #return_type {
                match self {
                    #match_to_index
                }
            }
            fn from_index(index: #return_type_ref) -> Option<Self> {
                match index {
                    #match_from_index
                    _ => None
                }
            }
        }
        impl EnumIndex for #name {}
        impl VariantByName for #name {
            fn by_name(name: &str) -> Option<Self> {
                match name {
                    #match_from_name
                    _ => None
                }
            }
        }
        impl From<#name> for #return_type {
            fn from(index: #name) -> Self {
                index.index()
            }
        }
        impl TryFrom<#return_type_ref> for #name {
            type Error = EnumIndexError;
            fn try_from(index: #return_type_ref) -> Result<Self, Self::Error> {
                Self::from_index(&index)
                .ok_or(
                    EnumIndexError::IndexNotFound(format!("{:?}", index))
                )
            }
        }
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.index().serialize(serializer)
            }
        }
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #return_type::deserialize(deserializer)
                .and_then(
                    | index | Self::try_from(&index as #return_type_ref)
                              .map_err(
                                |err| serde::de::Error::custom(err)
                              )
                )
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
