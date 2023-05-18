use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Attribute, Expr, ExprLit, Meta};

use crate::from_byte_reader::{ident_map, invalid_attr_type, parse_attrs, Ctx};

#[derive(Default)]
pub struct FieldAttrs {
    pub flags: Vec<syn::Expr>,
    pub reveal: Option<Span>,
    pub reveal_as: Vec<Ident>,
    pub parse_as: ParseAs,
    pub with: Option<syn::Expr>,
}

#[derive(Default)]
pub enum ParseAs {
    #[default]
    No,
    Yes(syn::TypePath),
    Try(syn::TypePath),
}

impl ParseAs {
    pub fn to_path(&self) -> Option<syn::TypePath> {
        match self {
            Self::Yes(path) | Self::Try(path) => Some(path.clone()),
            Self::No => None,
        }
    }

    pub fn conv_tokens(&self) -> Option<TokenStream> {
        Some(match self {
            ParseAs::No => return None,
            ParseAs::Yes(path) => {
                let span = path.span();
                quote_spanned! {
                    span=> .into()
                }
            }
            ParseAs::Try(path) => {
                let span = path.span();
                quote_spanned! {
                    span=> .try_into()?
                }
            }
        })
    }
}

pub fn parse_field_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<FieldAttrs, TokenStream> {
    let mut field_attrs = FieldAttrs::default();
    for item in parse_attrs::parse_attrs(attrs, ctx) {
        match item? {
            Meta::List(item) => return Err(invalid_attr_type(item.span())),

            Meta::Path(item) => {
                if item.is_ident(&ctx.reveal_attr) {
                    if let Some(span_1) = field_attrs.reveal {
                        let first = quote_spanned! {
                            span_1=> compile_error!("bare reveal attribute used more than once")
                        };
                        let span_2 = item.span();
                        let second = quote_spanned! {
                            span_2=> compile_error!("bare reveal attribute used more than once")
                        };
                        return Err(quote! {
                            #first
                            #second
                        });
                    }
                    field_attrs.reveal = Some(item.span());
                }
            }
            Meta::NameValue(item) => {
                let Expr::Lit(ExprLit { lit: syn::Lit::Str(lit_str), .. }) = item.value
                    else {
                        let span = item.value.span();
                        return Err(quote_spanned!{
                            span=> compile_error!("value should be a string literal")
                        });
                    };

                let err_map = syn::Error::into_compile_error;

                ident_map! (item.path, {
                    &ctx.flag_attr => {
                        let expr = lit_str.parse().map_err(err_map)?;
                        field_attrs.flags.push(expr);
                    },
                    &ctx.from_attr => {
                        let path = lit_str.parse().map_err(err_map)?;
                        field_attrs.parse_as = ParseAs::Yes(path);
                    },
                    &ctx.try_from_attr => {
                        let path = lit_str.parse().map_err(err_map)?;
                        field_attrs.parse_as = ParseAs::Try(path);
                    },
                    &ctx.reveal_attr => {
                    let ident = lit_str.parse().map_err(err_map)?;
                    field_attrs.reveal_as.push(ident);
                    },
                    &ctx.with_attr => {
                        let expr = lit_str.parse().map_err(err_map)?;
                        field_attrs.with = Some(expr);
                    },
                });
            }
        }
    }

    Ok(field_attrs)
}
