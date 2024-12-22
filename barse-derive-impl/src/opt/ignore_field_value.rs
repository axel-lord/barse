//! Ignore a field.

use ::quote::ToTokens;
use ::syn::{parse::Parse, Token};

/// Value protion of field ignore.
#[derive(Debug, Clone)]
pub struct IgnoreFieldValue {
    /// '=' token.
    pub eq_token: Token![=],

    /// Expression used when reading.
    pub value: ::syn::Expr,
}
impl Parse for IgnoreFieldValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ToTokens for IgnoreFieldValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.eq_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}
