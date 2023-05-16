use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Attribute};

use super::Ctx;
use crate::{dyn_mangle, dyn_mangle_display};

pub mod parse_field_attrs;

pub fn variable_block(
    name: Option<&Ident>,
    mangled_name: &Ident,
    field_attrs: &[Attribute],
    ctx: &Ctx,
) -> Result<TokenStream, TokenStream> {
    let field_attrs = parse_field_attrs::parse_field_attrs(field_attrs, ctx)?;

    let reader = &ctx.reader_param;

    let mut block = quote! {
        let #reader = &mut #reader;
    };

    // flag
    if !field_attrs.flags.is_empty() {
        let flags = &field_attrs.flags;
        quote! {
                let #reader = ::barse::reader::FlagByteReader::new(#reader, [#(#flags as &dyn ::std::any::Any),*]);
            }
            .to_tokens(&mut block);
    }

    // with
    let (trait_name, method_call) = field_attrs.with.as_ref().map_or_else(
        || {
            let method = &ctx.from_byte_reader_method;
            (&ctx.from_byte_reader_trait, quote! {#method(#reader)})
        },
        |expr| {
            let method = &ctx.from_byte_reader_with_method;
            (
                &ctx.from_byte_reader_with_trait,
                quote! {#method(#reader, #expr)},
            )
        },
    );

    // as || try_as
    match &field_attrs.parse_as {
        parse_field_attrs::ParseAs::No => {
            quote! {
                ::barse::#trait_name::#method_call?
            }
            .to_tokens(&mut block);
        }
        parse_field_attrs::ParseAs::Yes(path) => {
            quote! {
                <#path as ::barse::#trait_name>::#method_call?.into()
            }
            .to_tokens(&mut block);
        }
        parse_field_attrs::ParseAs::Try(path) => {
            quote! {
                <#path as ::barse::#trait_name>::#method_call?.try_into()?
            }
            .to_tokens(&mut block);
        }
    }

    // reveal
    let reveals = if let Some(span) = field_attrs.reveal {
        let name = name.ok_or_else(|| {
            quote_spanned! {
                span=> compile_error!("bare reveal cannot be used on a struct without field names")
            }
        })?;
        Some(name)
    } else {
        None
    }
    .into_iter()
    .chain(&field_attrs.reveal_as);

    // Ad variable for this field
    Ok(quote! {
        let #mangled_name = { #block };
        #(
            let #reveals = & #mangled_name;
        )*
    })
}

pub fn parse_fields(data_struct: &syn::DataStruct, ctx: &Ctx) -> Result<TokenStream, TokenStream> {
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

                variable_block(Some(name), &mangled_name, &field.attrs, ctx)?.to_tokens(&mut body);

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
                variable_block(None, &mangled_name, &field.attrs, ctx)?.to_tokens(&mut body);

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
    Ok(body)
}
