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

/// Parse a #[attr(...)] token structure from a strream.
pub fn parse_parenthesized_attribute<'a>(
    input: syn::parse::ParseStream<'a>,
    expects_ident: &str,
) -> syn::Result<syn::parse::ParseBuffer<'a>> {
    let buffer;

    let _pound_token: syn::Token![#] = input.parse().map_err(|_| {
        input.error(format!(
            "`#[{}(...)]` expected, but `#` not found.",
            expects_ident
        ))
    })?;
    let _bracket_token: syn::token::Bracket = syn::bracketed!(buffer in input);

    let content: syn::parse::ParseBuffer = buffer;

    let _is_ident: bool = content.lookahead1().peek(syn::Ident);

    if !_is_ident {
        return Err(content.error(format!(
            "`#[{}(...)]` expected, but `{}` not found.",
            expects_ident, expects_ident
        )));
    }

    let index_type_ident: syn::Ident = content.parse()?;

    if &index_type_ident.to_string() != expects_ident {
        return Err(syn::parse::Error::new(
            index_type_ident.span(),
            format!(
                "`#[{}(...)]` expected, but `{}` found instead.",
                expects_ident, index_type_ident
            ),
        ));
    }

    #[allow(unused_variables)]
    let paren_content: syn::parse::ParseBuffer<'a>;
    let _parens = syn::parenthesized!(paren_content in content);

    Ok(paren_content)
}
