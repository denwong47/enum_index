use quote::{quote, ToTokens};
use syn;
use syn::DeriveInput;

use enum_index_types::errors;

use crate::func;

/// Struct to parse the follow up attribute of #[index_type(T)].
///
/// To use #[derive(EnumIndex)], there must be a follow up attribute of
/// #[index_type(T)]. This is to indicate the return type of value.
///
/// `T` must be a valid type for ALL values provided subsequently.
///
/// See: https://stackoverflow.com/questions/56188700/how-do-i-make-my-custom-derive-macro-accept-trait-generic-parameters
#[derive(Debug, Clone)]
pub struct EnumIndexParams {
    pub is_ref: bool,
    pub value_ident: syn::Ident,
    pub index_type: syn::Type,
}

impl syn::parse::Parse for EnumIndexParams {
    fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::parse::Error> {
        let paren_content = func::parse_parenthesized_attribute(input, "index_type")?;

        let is_ref = paren_content.lookahead1().peek(syn::Token![&]);

        // Check if the specified type is a &T.
        if is_ref {
            let _: syn::Token![&] = paren_content.parse()?;
        }

        let value_ident: syn::Ident = {
            paren_content.parse().or_else(|err| {
                Err(syn::parse::Error::new(
                    err.span(),
                    "value type provided in #[index_type(T)] is not a valid Identifier.",
                ))
            })
        }?;

        let is_complex = paren_content.lookahead1().peek(syn::Token![<]);
        if is_complex {
            let msg = format!(
                "Complex type of `{}<>` detected; please use a Type alias for complex value types.",
                value_ident
            );
            return Err(syn::parse::Error::new(value_ident.span(), msg.as_str()));
        }

        let index_type: syn::Type = {
            syn::parse2(value_ident.to_token_stream())
            .or_else(
                |err| Err(syn::parse::Error::new(
                    err.span(),
                    "value type provided in #[index_type(T)] is not a valid Type. If you are using complex types such as `Option<usize>`, create a type alias for it first and use that instead."
                ))
            )
        }?;

        Ok(Self {
            is_ref,
            value_ident,
            index_type,
        })
    }
}
impl EnumIndexParams {
    /// Static method to look for the first attribute that matches its own requirement.
    fn find_attribute(input: &DeriveInput) -> Option<&syn::Attribute> {
        input.attrs.iter().find(|attr| {
            attr.path().segments.len() == 1 && attr.path().segments[0].ident == "index_type"
        })
    }

    /// Check if the type matches
    pub fn is_type(&self, value: &str) -> bool {
        func::is_type(&self.index_type, value)
    }

    /// Return TokenStream of itself as &T.
    pub fn return_type_ref(&self) -> syn::Type {
        syn::Type::from(syn::TypeReference {
            and_token: syn::Token![&](self.value_ident.span()),
            lifetime: None, //Some(syn::Lifetime::new("'static", self.value_ident.span())),
            mutability: None,
            elem: Box::new(if self.is_type("String") {
                syn::parse2(quote!(str)).unwrap()
            } else {
                self.index_type.clone()
            }),
        })
    }

    /// Return TokenStream of itself as T.
    pub fn return_type_owned(&self) -> syn::Type {
        self.index_type.clone()
    }

    /// Return TokenStream as &T if originally specified as such, otherwise return T.
    pub fn return_type_specified(&self) -> syn::Type {
        match self.is_ref {
            false => self.return_type_owned(),
            true => self.return_type_ref(),
        }
    }
}
impl TryFrom<&DeriveInput> for EnumIndexParams {
    type Error = errors::EnumIndexError;

    fn try_from(value: &DeriveInput) -> Result<Self, Self::Error> {
        let attr = Self::find_attribute(value).ok_or(
            errors::EnumIndexError::ParamsMetaNotFound(value.ident.to_string()),
        )?;

        syn::parse2(attr.into_token_stream())
            .map_err(|err| errors::EnumIndexError::SynParseError(value.ident.to_string(), err))
    }
}
