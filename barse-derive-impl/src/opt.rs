//! #[barse(...)] options.

use ::std::ops::ControlFlow;

use ::proc_macro2::Span;
use ::syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    token, Token, WherePredicate,
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

        /// Endian to use.
        endian: ::syn::Path,
    },

    /// Field with expression.
    FieldWithExpr {
        kw: kw::with,

        /// '=' token.
        eq_token: Token![=],

        /// With expression.
        with_expr: ::syn::Expr,
    }
}

opt_lite! {
    /// Option to ignore a field.
    IgnoreField {
        kw: kw::ignore,

        /// Expression used instead of default().
        value: Option<IgnoreFieldValue>,
    },

    /// Option to set a custom where clause.
    CustomWhere {
        kw: token::Where,

        /// Where predicates.
        predicates: Punctuated<WherePredicate, Token![,]>,
    }
}

/// Value protion of field ignore.
#[derive(Debug, Clone)]
pub struct IgnoreFieldValue {
    /// '=' token.
    pub eq_token: Token![=],

    /// Expression used when reading.
    pub value: ::syn::Expr,
}

/// With option.
#[derive(Debug, Clone)]
pub enum FieldWith {
    /// With is an expression.
    Expr(FieldWithExpr),

    /// With is a forward (nothing follows keyword.
    Fwd(kw::with),
}

impl Parse for IgnoreField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            value: if input.peek(Token![=]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for CustomWhere {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            predicates: input.call(Punctuated::parse_terminated)?,
        })
    }
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

impl FieldWith {
    /// Get keyword.
    pub const fn kw(&self) -> &kw::with {
        match self {
            FieldWith::Expr(FieldWithExpr { kw, .. }) | FieldWith::Fwd(kw) => kw,
        }
    }
}

impl Parse for FieldWith {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(if input.peek2(Token![=]) {
            FieldWith::Expr(input.parse()?)
        } else {
            FieldWith::Fwd(input.parse()?)
        })
    }
}

impl ToTokens for FieldWith {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldWith::Expr(field_with_expr) => field_with_expr.to_tokens(tokens),
            FieldWith::Fwd(kw) => kw.to_tokens(tokens),
        }
    }
}

impl Opt for FieldWith {
    fn peek(lookahead: &syn::parse::Lookahead1) -> bool {
        lookahead.peek(kw::with)
    }

    fn name() -> impl ::core::fmt::Display {
        <kw::with>::default().into_token_stream()
    }

    fn kw_span(&self) -> Span {
        self.kw().span
    }
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
    ),* $(,)?) => {$(
        opt_lite! {
            $(#[doc = $sdoc])*
            $nm {
                $(#[doc = $kwdoc])*
                kw: $kw,
                $(
                $(#[doc = $fdoc])*
                $f_nm: $f_ty,
                )*
            }
        }

        impl Parse for $nm {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                Ok(Self { kw: input.parse()?, $($f_nm: input.parse()?,)* })
            }
        }
    )*};
}

/// Generate option struct without parse impl.
macro_rules! opt_lite {
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
    ),* $(,)?) => {$(
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

        impl Opt for $nm {
            fn peek(lookahead: &::syn::parse::Lookahead1) -> bool {
                lookahead.peek($kw)
            }

            fn name() -> impl ::core::fmt::Display {
                <$kw>::default().into_token_stream()
            }

            fn kw_span(&self) -> Span {
                ::syn::spanned::Spanned::span(&self.kw)
            }
        }

        impl ToTokens for $nm {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                self.kw.to_tokens(tokens);
                $(self.$f_nm.to_tokens(tokens);)*
            }
        }
    )*};
}

/// Option trait.
pub trait Opt {
    /// Check if item in lookahead should be this option.
    fn peek(lookahead: &::syn::parse::Lookahead1) -> bool;

    /// Get name of option.
    fn name() -> impl ::core::fmt::Display;

    /// Get keyword span of option.
    fn kw_span(&self) -> Span;
}

/// Option parsing chain.
pub struct Chain<'a> {
    /// Lookahead for parse.
    lookahead: ::syn::parse::Lookahead1<'a>,
    /// Input for parse.
    input: &'a ParseBuffer<'a>,
    /// Current flow of chain.
    flow: ::core::ops::ControlFlow<()>,
}

impl<'a> Chain<'a> {
    /// Construct a new chain.
    pub fn new(input: &'a ParseBuffer<'a>) -> Self {
        let lookahead = input.lookahead1();
        Self {
            lookahead,
            input,
            flow: ::core::ops::ControlFlow::Continue(()),
        }
    }

    /// Add an option to be parsed.
    ///
    /// # Errors
    /// If lookahead matches an option and it cannot be parsed.
    pub fn parse_opt<O: Opt + Parse>(mut self, opt: &mut Option<O>) -> Result<Self, ::syn::Error> {
        if self.flow.is_break() {
            return Ok(self);
        }

        if !O::peek(&self.lookahead) {
            return Ok(self);
        }

        let val = O::parse(self.input)?;

        if opt.is_some() {
            return Err(::syn::Error::new(
                val.kw_span(),
                format!("'{}' has already been set", O::name()),
            ));
        }

        *opt = Some(val);
        self.flow = ControlFlow::Break(());

        Ok(self)
    }

    /// Finish parsing options.
    ///
    /// # Errors
    /// If no option was parsed.
    pub fn finish(self) -> Result<(), ::syn::Error> {
        if self.flow.is_break() {
            Ok(())
        } else {
            Err(self.lookahead.error())
        }
    }
}

/// Parse options from a [proc_macro2::TokenStream].
macro_rules! parse_opts {
    ($tokens:expr, $($opt:expr),+ $(,)?) => {{
        #![allow(unused_imports, reason = "user may have imported Parse for other reasons")]
        use ::syn::parse::Parse as _;
        let parse_opts_ = |input: ::syn::parse::ParseStream| -> Result<(), ::syn::Error> {
            while !input.is_empty() {
                $crate::opt::Chain::new(input)
                    $(.parse_opt(&mut $opt)?)*
                    .finish()?;

                if input.peek(::syn::Token![,]) {
                    _ = input.parse::<::syn::Token![,]>()?;
                    continue;
                }

                if !input.is_empty() {
                    return Err(input.error("expected ',' between entries"));
                }
            }
            Ok(())
        };
        ::syn::parse::Parser::parse2(parse_opts_, $tokens)?;
    }};
}

use opt;
use opt_lite;
pub(crate) use parse_opts;
