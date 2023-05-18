use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{spanned::Spanned, Attribute, Meta, MetaNameValue};

use crate::from_byte_reader::{parse_attrs, Ctx};

#[derive(Default)]
pub struct FieldAttrs {
    pub flags: Vec<syn::Expr>,
    pub reveal: Option<Span>,
    pub reveal_as: Vec<Ident>,
    pub parse_as: ParseAs,
    pub with: Option<syn::Expr>,
}

impl FieldAttrs {
    fn parse_name_value_attr(&mut self, item: MetaNameValue, ctx: &Ctx) -> Result<(), syn::Error> {
        let lit_str: syn::LitStr = syn::parse2(item.value.into_token_stream())?;

        let ident: syn::Ident = syn::parse2(item.path.into_token_stream())?;

        if ident == ctx.flag_attr {
            let expr = lit_str.parse()?;
            self.flags.push(expr);

            return Ok(());
        }

        if ident == ctx.from_attr {
            let path = lit_str.parse()?;
            self.parse_as = ParseAs::Yes(path);

            return Ok(());
        }

        if ident == ctx.try_from_attr {
            let path = lit_str.parse()?;
            self.parse_as = ParseAs::Try(path);

            return Ok(());
        }

        if ident == ctx.reveal_attr {
            let ident = lit_str.parse()?;
            self.reveal_as.push(ident);

            return Ok(());
        }

        if ident == ctx.with_attr {
            let expr = lit_str.parse()?;
            self.with = Some(expr);

            return Ok(());
        }

        Ok(())
    }
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

pub fn parse_field_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<FieldAttrs, syn::Error> {
    let mut field_attrs = FieldAttrs::default();
    for item in parse_attrs::parse_attrs(attrs, ctx) {
        match item? {
            Meta::List(item) => {
                return Err(syn::Error::new(
                    item.span(),
                    "barse attribute lists items are either of the form \
                    'name = \"value\"' or 'name' example: #[barse(flag = \"name\", reveal)]",
                ))
            }

            Meta::Path(item) => {
                if item.is_ident(&ctx.reveal_attr) {
                    if let Some(span_1) = field_attrs.reveal {
                        let mut err_1 =
                            syn::Error::new(span_1, "bare reveal attribute used more than once");

                        let err_2 = syn::Error::new(
                            item.span(),
                            "bare reveal attribute used more than once",
                        );

                        err_1.combine(err_2);
                        return Err(err_1);
                    }
                    field_attrs.reveal = Some(item.span());
                }
            }
            Meta::NameValue(item) => field_attrs.parse_name_value_attr(item, ctx)?,
        }
    }

    Ok(field_attrs)
}
