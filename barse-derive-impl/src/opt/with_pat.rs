//! With pattern.

use ::quote::ToTokens;
use ::syn::{
    parse::{Parse, ParseStream},
    parse_quote, Token,
};

/// With pattern of the pat: Type format.
#[derive(Debug, Clone)]
pub struct WithPatType {
    /// Pattern to bind with type to.
    pub pat: ::syn::Pat,

    /// Colon separating pattern and type.
    pub colon_token: Token![:],

    /// Type of with value.
    pub ty: ::syn::Type,
}

impl Parse for WithPatType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: input.call(::syn::Pat::parse_single)?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl ToTokens for WithPatType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pat.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

/// With pattern.
#[derive(Debug, Clone)]
pub enum WithPat {
    /// With pattern of the pat: Type format.
    PatType {
        /// Pattern and type.
        with_pat_type: WithPatType,
    },
    /// With pattern as only a type.
    TypeOnly {
        /// Parsed type.
        ty: ::syn::Type,
    },
}

impl WithPat {
    /// Ensure pattern exists.
    pub fn ensure_pat(self, ident: impl FnOnce() -> ::syn::Ident) -> WithPatType {
        match self {
            WithPat::PatType { with_pat_type } => with_pat_type,
            WithPat::TypeOnly { ty } => {
                let ident = ident();
                WithPatType {
                    pat: parse_quote!(#ident),
                    colon_token: <Token![:]>::default(),
                    ty,
                }
            }
        }
    }
}

impl Parse for WithPat {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        #[inline(always)]
        /// # Errors
        /// On failed parse.
        fn parse_(input: ParseStream) -> ::syn::Result<WithPat> {
            use ::syn::parse::discouraged::Speculative;
            let fork = input.fork();
            if let Ok(with_pat_type) = fork.parse() {
                input.advance_to(&fork);
                Ok(WithPat::PatType { with_pat_type })
            } else {
                Ok(WithPat::TypeOnly { ty: input.parse()? })
            }
        }

        parse_(input).map_err(|err| {
            ::syn::Error::new(err.span(), "expected either 'pattern: Type' or 'Type'")
        })
    }
}

impl ToTokens for WithPat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            WithPat::PatType { with_pat_type } => with_pat_type.to_tokens(tokens),
            WithPat::TypeOnly { ty } => ty.to_tokens(tokens),
        }
    }
}

