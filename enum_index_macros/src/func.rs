/// Shared functions.
///
extern crate proc_macro;

use syn;
#[allow(unused_imports)]
use syn::DeriveInput;

#[allow(unused_imports)]
use enum_index_types::*;

/// Attempt to get the syn::DataEnum from input.
///
/// Returns an Err if input data is not an Enum.
///
/// Contains 3 attributes:
/// - enum_token
/// - brace_token
/// - variants
pub fn get_enum_data(input: &DeriveInput) -> Result<&syn::DataEnum, errors::EnumIndexError> {
    if let syn::Data::Enum(data_enum) = &input.data {
        Ok(&data_enum)
    } else {
        Err(errors::EnumIndexError::NotAnEnum(input.ident.to_string()))
    }
}

/// Check if Type matches.
pub fn is_type(t: &syn::Type, value: &str) -> bool {
    if let syn::Type::Path(t_path) = t {
        t_path
            .path
            .segments
            .iter()
            .map(|seg| seg.ident.to_string())
            .reduce(|lhs, rhs| lhs + "::" + &rhs)
            .map(|s| s.as_str() == value)
            .unwrap_or(false)
    } else {
        false
    }
}

/// Check if an expression is a literal str.
///
/// This is mostly for the special case when #[index_type(String)].
pub fn is_lit_str(e: &syn::Expr) -> bool {
    match e {
        syn::Expr::Group(g) => is_lit_str(&g.expr),
        syn::Expr::Lit(l) => {
            if let syn::Lit::Str(_) = l.lit {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
