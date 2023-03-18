use quote::ToTokens;
use syn;

use enum_index_types::errors;

use crate::func;

/// An #[index(value)] attribute.
#[derive(Clone, Debug)]
pub struct IndexAttribute(syn::Expr);
impl IndexAttribute {
    pub fn expr(&self) -> &syn::Expr {
        &self.0
    }
}
impl syn::parse::Parse for IndexAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = func::parse_parenthesized_attribute(input, "index")?;

        let value: syn::Expr = {
            content.parse().or_else(|err| {
                Err(syn::parse::Error::new(
                    err.span(),
                    "value provided in #[index(T)] is not a valid expression.",
                ))
            })
        }?;
        // content.parse::<Token![,]>()?;

        Ok(Self(value))
    }
}
impl TryFrom<&syn::Variant> for IndexAttribute {
    type Error = errors::EnumIndexError;
    fn try_from(value: &syn::Variant) -> Result<Self, Self::Error> {
        value
            .attrs
            .iter()
            .find(|attr| {
                attr.path().get_ident().map(|i| i.to_string()) == Some(String::from("index"))
            })
            .ok_or(errors::EnumIndexError::IndexNotGiven(
                value.ident.to_string(),
            ))
            .and_then(|attr| {
                syn::parse2(attr.into_token_stream()).map_err(|err| {
                    errors::EnumIndexError::SynParseError(value.ident.to_string(), err)
                })
            })
    }
}
