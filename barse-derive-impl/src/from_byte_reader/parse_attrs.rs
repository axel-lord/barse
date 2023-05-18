use syn::{parse::Parser, punctuated::Punctuated, spanned::Spanned, Attribute, Meta, Token};

use super::Ctx;

pub fn parse_attrs<'a>(
    attrs: &'a [Attribute],
    ctx: &'a Ctx,
) -> impl 'a + Iterator<Item = Result<Meta, syn::Error>> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident(&ctx.attr_ident))
        .map(|attr| {
            Ok(match &attr.meta {
                syn::Meta::List(list) => Punctuated::<Meta, Token![,]>::parse_terminated
                    .parse2(list.tokens.clone())?
                    .into_iter(),
                value => {
                    let span = value.span();
                    return Err(syn::Error::new(
                        span,
                        "barse attribute only takes the form of #[barse(name = \"value\", ..)]",
                    ));
                }
            })
        })
        .flat_map(|meta| match meta {
            Err(err) => either::Left(std::iter::once(Err(err))),
            Ok(it) => either::Right(it.map(Ok)),
        })
}
