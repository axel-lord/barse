use quote::ToTokens;
use syn::Attribute;

use super::{parse_attrs, Ctx};

#[derive(Default, Debug)]
pub struct StructAttrs {
    pub error: Option<syn::Type>,
    pub with: Option<syn::Type>,
    pub reveal: Option<syn::Pat>,
    pub predicates: Vec<syn::WherePredicate>,
}

impl StructAttrs {
    pub fn new(attrs: &[Attribute], ctx: &Ctx) -> Result<StructAttrs, syn::Error> {
        let mut struct_attrs = StructAttrs::default();
        for item in parse_attrs::parse_attrs(attrs, ctx) {
            let item: syn::MetaNameValue = syn::parse2(item?.to_token_stream())?;

            let lit_str: syn::LitStr = syn::parse2(item.value.into_token_stream())?;

            let Some(ident) = item.path.get_ident() else {
                continue;
            };

            if ident == &ctx.error_attr {
                let ty = lit_str.parse()?;
                struct_attrs.error = Some(ty);

                continue;
            }

            if ident == &ctx.with_attr {
                let ty = lit_str.parse()?;
                struct_attrs.with = Some(ty);

                continue;
            }

            if ident == &ctx.reveal_attr {
                let pat = lit_str.parse_with(syn::Pat::parse_single)?;
                struct_attrs.reveal = Some(pat);

                continue;
            }

            if ident == &ctx.predicate_attr {
                let predicate = lit_str.parse()?;
                struct_attrs.predicates.push(predicate);

                continue;
            }
        }
        Ok(struct_attrs)
    }
}
