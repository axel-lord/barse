use quote::ToTokens;
use syn::Attribute;

use super::{parse_attrs, parse_fields::parse_field_attrs::ParseAs, Ctx};

#[derive(Default)]
pub struct StructAttrs {
    pub error: Option<syn::Type>,
    pub parse_as: ParseAs,
    pub with: Option<syn::Type>,
    pub reveal: Option<syn::Pat>,
}

pub fn parse_struct_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<StructAttrs, syn::Error> {
    let mut struct_attrs = StructAttrs::default();
    for item in parse_attrs::parse_attrs(attrs, ctx) {
        let item: syn::MetaNameValue = syn::parse2(item?.to_token_stream())?;

        let lit_str: syn::LitStr = syn::parse2(item.value.to_token_stream())?;

        if let Some(ident) = item.path.get_ident() {
            if ident == &ctx.error_attr {
                let ty = lit_str.parse()?;
                struct_attrs.error = Some(ty);

                continue;
            }

            if ident == &ctx.from_attr {
                let path = lit_str.parse()?;
                struct_attrs.parse_as = ParseAs::Yes(path);

                continue;
            }

            if ident == &ctx.try_from_attr {
                let path = lit_str.parse()?;
                struct_attrs.parse_as = ParseAs::Try(path);

                continue;
            }
        }
    }
    Ok(struct_attrs)
}
