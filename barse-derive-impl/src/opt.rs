//! #[barse(...)] options.

use ::syn::{
    parse::{Parse, ParseStream},
    Token,
};
use quote::ToTokens;

use crate::kw;

mod with_pat;

pub use with_pat::WithPat;

opt! {
    /// Path to barse module.
    BarsePath {
        kw: kw::barse_path,

        /// '=' token.
        eq_token: Token![=],

        /// Path to Barse trait.
        path: ::syn::Path,
    },
    /// With pattern of type.
    With {
        kw: kw::with,

        /// '=' token.
        eq_token: Token![=],

        /// with pattern.
        with_pat: WithPat,
    },

    /// ReadWith pattern of type.
    ReadWith {
        kw: kw::read_with,

        /// '=' token.
        eq_token: Token![=],

        /// With pattern.
        with_pat: WithPat,
    },

    /// WriteWith pattern of type.
    WriteWith {
        kw: kw::write_with,

        /// '=' token.
        eq_token: Token![=],

        /// With pattern.
        with_pat: WithPat,
    },

    /// Field prefix.
    FieldPrefix {
        kw: kw::field_prefix,

        /// '=' token.
        eq_token: Token![=],

        /// Field prefix.
        field_prefix: ::syn::Ident,
    },

    /// Force endian.
    Endian {
        kw: kw::endian,

        /// '=' token.
        eq_token: Token![=],

        /// Ednian to use.
        endian: ::syn::Path,
    },
}

/// Generate option structs.
macro_rules! opt {
    ($(
        $(#[doc = $sdoc:expr])*
        $nm:ident {
            $(#[doc = $kwdoc:expr])*
            kw: $kw:path,
            $(
            $(#[doc = $fdoc:expr])*
            $f_nm:ident: $f_ty:ty,
            )*
        }
    ),* $(,)?) => {
    $(
        #[derive(Debug, Clone)]
        $(#[doc = $sdoc])*
        pub struct $nm {
            #[doc = "Option keyword."]
            pub kw: $kw,

        $(
            $(#[doc = $fdoc])*
            pub $f_nm: $f_ty,
        )*

        }

        impl $nm {
            /// Check if value in lookahead matches kw.
            pub fn peek(lookahead: &::syn::parse::Lookahead1) -> bool {
                lookahead.peek($kw)
            }
        }

        impl Parse for $nm {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                Ok(Self { kw: input.parse()?, $($f_nm: input.parse()?,)* })
            }
        }

        impl ToTokens for $nm {

            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            $(
                self. $f_nm .to_tokens(tokens);
            )*
            }
        }
    )*
    };
}
use opt;
