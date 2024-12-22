//! Field with expression.

use ::std::ops::Deref;

use ::quote::ToTokens;
use ::syn::{parse::Parse, Token};

/// Field with expression.
#[derive(Debug, Clone)]
pub struct FieldWithExpr {
    /// '=' token.
    pub eq_token: Token![=],

    /// With expression.
    pub expr: ::syn::Expr,
}

impl Parse for FieldWithExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            eq_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl ToTokens for FieldWithExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.eq_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}

impl Deref for FieldWithExpr {
    type Target = ::syn::Expr;

    fn deref(&self) -> &Self::Target {
        &self.expr
    }
}
