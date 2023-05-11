use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DeriveInput, Expr,
    ExprLit, Lifetime, MetaNameValue, Token,
};

use crate::{dyn_mangle, dyn_mangle_display, static_mangle};

struct Ctx {
    pub attr_ident: Ident,
    pub flag_ident: Ident,
    pub as_ident: Ident,
    pub try_as_ident: Ident,
    pub reader: Ident,
    pub input_lifetime: Lifetime,
}

impl Default for Ctx {
    fn default() -> Self {
        Ctx {
            attr_ident: Ident::new("barse", Span::call_site()),
            flag_ident: Ident::new("flag", Span::call_site()),
            as_ident: Ident::new("as", Span::call_site()),
            try_as_ident: Ident::new("try_as", Span::call_site()),
            input_lifetime: {
                let ident = static_mangle("input");
                Lifetime {
                    apostrophe: Span::call_site(),
                    ident,
                }
            },
            reader: static_mangle("reader"),
        }
    }
}

#[derive(Default)]
struct FieldAttrs {
    flags: Vec<Ident>,
    parse_as: Option<syn::Path>,
    try_parse_as: Option<syn::Path>,
}
fn parse_field_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<FieldAttrs, TokenStream> {
    let mut field_attrs = FieldAttrs::default();
    for attr in attrs {
        if attr.path().is_ident(&ctx.attr_ident) {
            continue;
        };

        let values = match &attr.meta {
            syn::Meta::Path(_) => {
                let span = attr.span();
                return Err(quote_spanned! {
                    span=> compile_error!("barse attributes should be a list of names and values")
                });
            }
            syn::Meta::List(list) => {
                let parser = Punctuated::<MetaNameValue, Token![,]>::parse_terminated;
                let list = parser
                    .parse2(list.tokens.clone())
                    .map_err(syn::Error::into_compile_error)?;
                let iter = list.into_iter();
                either::Either::Left(iter)
            }
            syn::Meta::NameValue(value) => {
                let iter = std::iter::once(value.clone());
                either::Either::Right(iter)
            }
        };

        for item in values {
            let Expr::Lit(ExprLit { lit: syn::Lit::Str(lit_str), .. }) = item.value else {
                let span = item.value.span();
                return Err(quote_spanned!{
                    span=> compile_error!("value should be a string literal")
                });
            };

            if item.path.is_ident(&ctx.flag_ident) {
                let ident = lit_str
                    .parse::<syn::Ident>()
                    .map_err(syn::Error::into_compile_error)?;
                field_attrs.flags.push(dyn_mangle(&ident));
            } else if item.path.is_ident(&ctx.as_ident) {
                let path = lit_str
                    .parse::<syn::Path>()
                    .map_err(syn::Error::into_compile_error)?;
                field_attrs.parse_as = Some(path);
            } else if item.path.is_ident(&ctx.try_as_ident) {
                let path = lit_str
                    .parse::<syn::Path>()
                    .map_err(syn::Error::into_compile_error)?;
                field_attrs.try_parse_as = Some(path);
            }
        }
    }

    Ok(field_attrs)
}

fn variable_block(
    mangled_name: &Ident,
    field_attrs: &[Attribute],
    ctx: &Ctx,
) -> Result<TokenStream, TokenStream> {
    let field_attrs = parse_field_attrs(field_attrs, ctx)?;

    let reader = &ctx.reader;

    let mut block = quote! {
        let #reader = &mut #reader;
    };
    if !field_attrs.flags.is_empty() {
        let flags = &field_attrs.flags;
        quote! {
            let #reader = ::barse::FlagByteReader::new(#reader, [#(& #flags),*]);
        }
        .to_tokens(&mut block);
    }

    if let Some(path) = &field_attrs.parse_as {
        quote! {
            <#path as ::barse::FromByteReader>::from_byte_reader(#reader)?.into()
        }
        .to_tokens(&mut block);
    } else if let Some(path) = &field_attrs.try_parse_as {
        quote! {
            <#path as ::barse::FromByteReader>::from_byte_reader(#reader)?.try_into()?
        }
        .to_tokens(&mut block);
    } else {
        quote! {
            ::barse::FromByteReader::from_byte_reader(#reader)?
        }
        .to_tokens(&mut block);
    }

    // Ad variable for this field
    Ok(quote! {
        let #mangled_name = { #block };
    })
}

#[allow(clippy::too_many_lines)]
pub fn impl_trait(ast: &DeriveInput) -> Result<TokenStream, TokenStream> {
    let name = &ast.ident;

    let Data::Struct(data_struct) = &ast.data else {
        let span = ast.span();
        return Err(quote_spanned! {
            span=> compile_error!("FromByteReader can only be derived for structs")
        });
    };

    let ctx = Ctx::default();

    let mut body = TokenStream::new();
    match data_struct.fields {
        syn::Fields::Named(ref fields) => {
            let mut return_value = TokenStream::new();

            for field in &fields.named {
                let name = field.ident.as_ref().ok_or_else(|| {
                    let span = field.span();
                    quote_spanned!(span=> compile_error!("unnamed field in non-tuple struct"))
                })?;

                let mangled_name = dyn_mangle(name);

                variable_block(&mangled_name, &field.attrs, &ctx)?.to_tokens(&mut body);

                // Add this field to return value
                quote! {
                    #name: #mangled_name,
                }
                .to_tokens(&mut return_value);
            }

            // Add return value to body
            quote! {
                Ok(Self { #return_value })
            }
            .to_tokens(&mut body);
        }
        syn::Fields::Unnamed(ref fields) => {
            let mut return_value = TokenStream::new();

            for (field_num, field) in fields.unnamed.iter().enumerate() {
                let mangled_name = dyn_mangle_display(field_num);

                // Initialize variable
                variable_block(&mangled_name, &field.attrs, &ctx)?.to_tokens(&mut body);

                // Ad this field to return value
                quote! {
                    #mangled_name,
                }
                .to_tokens(&mut return_value);
            }

            // Add return value to body
            quote! {
                Ok(Self(#return_value))
            }
            .to_tokens(&mut body);
        }
        syn::Fields::Unit => quote! {
            Ok(Self)
        }
        .to_tokens(&mut body),
    };

    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

    let input_lifetime = &ctx.input_lifetime;
    let lifetimes = ast.generics.lifetimes().collect::<Vec<_>>();
    let impl_generics = std::iter::once(if lifetimes.is_empty() {
        quote!(#input_lifetime)
    } else {
        quote!(#input_lifetime: #(#lifetimes),*)
    })
    .chain(
        ast.generics
            .params
            .clone()
            .into_iter()
            .map(|p| p.to_token_stream()),
    );

    let reader = &ctx.reader;
    Ok(quote! {
        #[automatically_derived]
        impl <#(#impl_generics),*> FromByteReader<#input_lifetime> for #name #ty_generics #where_clause {
            type Err = ::barse::error::Error;
            fn from_byte_reader<R>(mut #reader: R) -> ::barse::Result<Self>
            where
                R: ::barse::ByteRead<#input_lifetime>,
            {
                #body
            }
        }
    })
}
