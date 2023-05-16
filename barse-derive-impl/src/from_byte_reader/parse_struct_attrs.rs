use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, Attribute, Expr, ExprLit, Meta};

use super::{invalid_attr_type, parse_attrs, Ctx};

#[derive(Default)]
pub struct StructAttrs {
    pub error: Option<syn::Type>,
}

pub fn parse_struct_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<StructAttrs, TokenStream> {
    let mut struct_attrs = StructAttrs::default();
    for item in parse_attrs::parse_attrs(attrs, ctx) {
        match item? {
            Meta::List(item) => return Err(invalid_attr_type(item.span())),
            Meta::Path(_) => (),
            Meta::NameValue(item) => {
                let Expr::Lit(ExprLit { lit: syn::Lit::Str(lit_str), .. }) = item.value
                    else {
                        let span = item.value.span();
                        return Err(quote_spanned!{
                            span=> compile_error!("value should be a string literal")
                        });
                    };

                if item.path.is_ident(&ctx.error_attr) {
                    let ty = lit_str
                        .parse::<syn::Type>()
                        .map_err(syn::Error::into_compile_error)?;
                    struct_attrs.error = Some(ty);
                }
            }
        }
    }
    Ok(struct_attrs)
}
