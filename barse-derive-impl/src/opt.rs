//! #[barse(...)] options.

use ::std::ops::ControlFlow;

use ::proc_macro2::{Span, TokenStream};
use ::syn::{
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned as _,
    token, Token, WherePredicate,
};
use quote::ToTokens;

use crate::{
    kw,
    opt::opt_macro::{opt, opt_lite},
};

mod opt_macro;

opt! {
    /// Path to barse module.
    BarsePath {
        /// Opt keyword.
        kw: kw::barse_path,

        /// '=' token.
        eq_token: Token![=],

        /// Path to Barse trait.
        path: ::syn::Path,
    },
    /// With pattern of type.
    With {
        /// Opt keyword.
        kw: kw::with,

        /// '=' token.
        eq_token: Token![=],

        /// with pattern.
        with_pat: WithPat,
    },

    /// ReadWith pattern of type.
    ReadWith {
        /// Opt keyword.
        kw: kw::read_with,

        /// '=' token.
        eq_token: Token![=],

        /// With pattern.
        with_pat: WithPat,
    },

    /// WriteWith pattern of type.
    WriteWith {
        /// Opt keyword.
        kw: kw::write_with,

        /// '=' token.
        eq_token: Token![=],

        /// With pattern.
        with_pat: WithPat,
    },

    /// Field prefix.
    FieldPrefix {
        /// Opt keyword.
        kw: kw::field_prefix,

        /// '=' token.
        eq_token: Token![=],

        /// Field prefix.
        field_prefix: ::syn::Ident,
    },

    /// Force endian.
    Endian {
        /// Opt keyword.
        kw: kw::endian,

        /// '=' token.
        eq_token: Token![=],

        /// Endian to use.
        endian: ::syn::Path,
    },

    /// Read and write field as bytes.
    Bytes {
        /// Opt keyword.
        kw: kw::bytes,

        /// '=' token.
        eq_token: Token![=],

        /// How many bytes to parse.
        count: ::syn::Expr,
    },

    /// Read field as bytes.
    ReadBytes {
        /// Opt Keyword.
        kw: kw::read_bytes,

        /// '=' token.
        eq_token: Token![=],

        /// How many bytes to parse.
        count: ::syn::Expr,
    },

    /// Write field as bytes.
    WriteBytes {
        /// Opt Keyword.
        kw: kw::write_bytes,
    },

    /// Field with expression.
    FieldWithExpr {
        /// '=' token.
        eq_token: Token![=],

        /// With expression.
        expr: ::syn::Expr,
    },

    /// Value portion of field ignore.
    IgnoreFieldValue {
        /// '=' token.
        eq_token: Token![=],

        /// Expression used when reading.
        value: ::syn::Expr,
    },

    /// With pattern pattern and colon.
    WithPatPat {
        /// Pattern to bind type to.
        #[attr = deref]
        pat: ::syn::Ident,

        /// ':' token.
        colon_token: token::Colon,
    },

    /// Read using a specific impl.
    ReadAs {
        /// Opt keyword
        kw: kw::read_as,

        /// Function path.
        as_expr: ::syn::Expr,
    },

    /// Write using a specific impl.
    WriteAs {
        /// Opt keyword
        kw: kw::write_as,

        /// Function path.
        as_expr: ::syn::Expr,
    },

    /// Read/Write using a specific impl.
    BarseAs {
        /// Opt keyword
        kw: token::As,

        /// Function path.
        as_expr: ::syn::Expr,
    },
}

opt_lite! {
    /// Option to ignore a field.
    IgnoreField {
        /// Opt keyword.
        kw: kw::ignore,

        /// Expression used instead of default().
        value: Option<IgnoreFieldValue>,
    },

    /// Option to set a custom where clause.
    CustomWhere {
        /// Opt keyword.
        kw: token::Where,

        /// Where predicates.
        predicates: Punctuated<WherePredicate, Token![,]>,
    },

    /// Option to forward or use an expression for field with.
    FieldWith {
        /// Opt keyword.
        kw: kw::with,

        /// With expression to use.
        expr: Option<FieldWithExpr>,
    },

    /// Option to forward or use an expression for field read with.
    FieldReadWith {
        /// Opt keyword.
        kw: kw::read_with,

        /// With expression to use.
        expr: Option<FieldWithExpr>,
    },

    /// Option to forward or use an expression for field write with.
    FieldWriteWith {
        /// Opt keyword.
        kw: kw::write_with,

        /// With expression to use.
        expr: Option<FieldWithExpr>,
    },

    /// With pattern for types.
    WithPat {
        /// Optional pattern.
        pat: Option<WithPatPat>,

        /// Type of pattern.
        ty: ::syn::Type,
    }
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

impl Parse for FieldWith {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            expr: if input.peek(Token![=]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for FieldReadWith {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            expr: if input.peek(Token![=]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for FieldWriteWith {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            kw: input.parse()?,
            expr: if input.peek(Token![=]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for WithPat {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let err = |err: ::syn::Error| {
            ::syn::Error::new(err.span(), "expected either 'pattern: Type' or 'Type'")
        };
        Ok(Self {
            pat: if input.peek(::syn::Ident) && input.peek2(Token![:]) && !input.peek3(Token![:]) {
                Some(input.parse().map_err(err)?)
            } else {
                None
            },
            ty: input.parse().map_err(err)?,
        })
    }
}

/// Parse barse attributes and run given function on tokens.
///
/// # Errors
/// If the barse attribute is not of the [Barse(...)] format.
/// Or if the given function errors.
pub fn parse_attrs(
    attrs: &[::syn::Attribute],
    mut f: impl FnMut(TokenStream) -> Result<(), ::syn::Error>,
) -> Result<(), ::syn::Error> {
    let mut parse_attr = |attr: &::syn::Attribute| -> Result<(), ::syn::Error> {
        let meta_list = attr.meta.require_list().map_err(|_| {
            ::syn::Error::new(attr.meta.span(), "expected list attribute: #[barse(...)]")
        })?;

        f(meta_list.tokens.clone())?;
        Ok(())
    };
    let mut errors = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("barse") {
            continue;
        }

        if let Err(err) = parse_attr(attr) {
            errors.push(err);
        }
    }
    errors
        .into_iter()
        .reduce(|mut acc, err| {
            acc.combine(err);
            acc
        })
        .map_or(Ok(()), Err)
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
        self.flow = ControlFlow::Break(());

        if opt.is_some() {
            return Err(::syn::Error::new(
                val.kw_span(),
                format!("'{}' has already been set", O::name()),
            ));
        }

        *opt = Some(val);

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

/// Create structs used to parse attributes.
macro_rules! opt_parser {
    ($(
        $(#[doc = $sdoc:expr])*
        $nm:ident {$(
            $(#[doc = $fdoc:expr])*
            $fnm:ident: $fty:ty
        ),+ $(,)?}
    ),* $(,)?) => {$(
        $(#[doc = $sdoc])*
        #[derive(Debug, Clone, Default)]
        pub struct $nm {$(
            $(#[doc = $fdoc])*
            pub $fnm: Option<$fty>,
        )*}

        impl $nm {
            #[doc = "Parse options from attributes\n# Errors\nIf option parsing fails."]
            pub fn parse_attrs(mut self, attrs: &[::syn::Attribute]) -> Result<Self, ::syn::Error> {
                $crate::opt::parse_attrs(attrs, |tokens| {
                    $crate::opt::parse_opts!(
                        tokens,
                        $(self. $fnm,)*
                    );
                    Ok(())
                })?;
                Ok(self)
            }
        }

    )*};
}

pub(crate) use opt_parser;
pub(crate) use parse_opts;
