//! [FieldConfig] impl.

use ::quote::ToTokens;
use ::syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Token,
};

use crate::kw;

/// Value protion of field ignore.
#[derive(Debug)]
pub struct IgnoreFieldValue {
    /// '=' token.
    pub _eq_token: Token![=],

    /// Expression used when reading.
    pub value: ::syn::Expr,
}

impl Parse for IgnoreFieldValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// Option to ignore a field.
#[derive(Debug)]
pub struct IgnoreField {
    /// ignore keyword.
    pub kw: kw::ignore,

    /// Optional value.
    pub value: Option<IgnoreFieldValue>,
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

/// Field  option.
#[derive(Debug)]
enum FieldOpt {
    /// Ignore this field for writes, using '= expr' or default for reads.
    Ignore(IgnoreField),
}

impl Parse for FieldOpt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        Ok(if lookahead.peek(kw::ignore) {
            FieldOpt::Ignore(input.parse()?)
        } else {
            return Err(lookahead.error());
        })
    }
}

/// Field configuration determined by attribute.
#[derive(Debug)]
pub struct FieldConfig {
    /// Field is ignored.
    pub ignore: Option<IgnoreField>,
}

impl FieldConfig {
    /// Get config from field attributes
    ///
    /// # Errors
    /// If any invalid field attributes are encountered.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        let mut ignore = None;
        for attr in attrs {
            if !attr.path().is_ident("barse") {
                continue;
            }

            let meta_list = attr.meta.require_list().map_err(|_| {
                ::syn::Error::new(attr.meta.span(), "expected list attribute: #[barse(...)]")
            })?;

            for opt in Punctuated::<FieldOpt, Token![,]>::parse_terminated
                .parse2(meta_list.tokens.clone())?
            {
                match opt {
                    FieldOpt::Ignore(ignore_field) => {
                        if ignore.is_none() {
                            ignore = Some(ignore_field);
                        } else {
                            return Err(::syn::Error::new(
                                ignore_field.kw.span(),
                                "ignore has already been set",
                            ));
                        }
                    }
                }
            }
        }
        Ok(Self { ignore })
    }
}
