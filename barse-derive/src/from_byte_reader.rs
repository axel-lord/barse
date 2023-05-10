use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, Data, DeriveInput, Expr, Lifetime,
    MetaNameValue, Token,
};

use crate::{dyn_mangle, static_mangle};

fn field_init(reader: &Ident) -> TokenStream {
    quote! {
        ::barse::FromByteReader::from_byte_reader(&mut #reader)?
    }
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

    let reader = static_mangle("reader");
    let input = {
        let ident = static_mangle("input");
        Lifetime {
            apostrophe: Span::call_site(),
            ident,
        }
    };
    let attr_ident = Ident::new("barse", Span::call_site());
    let flag_ident = Ident::new("flag", Span::call_site());

    let body: TokenStream = match data_struct.fields {
        syn::Fields::Named(ref fields) => {
            let mut body = TokenStream::new();
            let mut return_value = TokenStream::new();

            for field in &fields.named {
                let name = field.ident.as_ref().ok_or_else(|| {
                    let span = field.span();
                    quote_spanned!(span=> compile_error!("unnamed field in non-tuple struct"))
                })?;

                let mangled_name = dyn_mangle(name);
                let mut flags = Vec::new();
                for attr in &field.attrs {
                    if attr.path().is_ident(&attr_ident) {
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
                        if item.path.is_ident(&flag_ident) {
                            let Expr::Path(path) = item.value else {
                                let span = item.value.span();
                                return Err(quote_spanned!{
                                    span=> compile_error!("only identifiers are accepted as flag values")
                                });
                            };

                            let Some(ident) = path.path.get_ident() else {
                                let span = path.span();
                                return Err(quote_spanned!{
                                    span=> compile_error!("flag value should be a single identifier")
                                });
                            };

                            flags.push(dyn_mangle(ident));
                        }
                    }
                }

                let field_reader = if flags.is_empty() {
                    quote!(&mut #reader)
                } else {
                    quote! {
                        ::barse::FlagByteReader::new(&mut #reader, [#(& #flags),*])
                    }
                };

                let init = quote! {
                    ::barse::FromByteReader::from_byte_reader(#field_reader)?
                };

                // Add variable for this field
                quote!(let #mangled_name = #init;).to_tokens(&mut body);

                // Add this field to return value
                quote!(#name: #mangled_name,).to_tokens(&mut return_value);
            }

            // Add return value to body
            quote! {
                Ok(Self { #return_value })
            }
            .to_tokens(&mut body);

            // return body
            body
        }
        syn::Fields::Unnamed(ref fields) => {
            let mut body = TokenStream::new();
            let mut return_value = TokenStream::new();

            for (field_num, _) in fields.unnamed.iter().enumerate() {
                let mangled_name =
                    dyn_mangle(&Ident::new(&format!("{field_num}"), Span::call_site()));

                let init = field_init(&reader);

                // Ad variable for this field
                quote!(let #mangled_name = #init;).to_tokens(&mut body);

                // Ad this field to return value
                quote!(#mangled_name,).to_tokens(&mut return_value);
            }

            // Add retuen value to body
            quote!(Ok(Self(#return_value))).to_tokens(&mut body);

            // return body
            body
        }
        syn::Fields::Unit => quote! {Ok(Self)},
    };

    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

    let lifetimes = ast.generics.lifetimes().collect::<Vec<_>>();
    let impl_generics = std::iter::once(if lifetimes.is_empty() {
        quote!(#input)
    } else {
        quote!(#input: #(#lifetimes),*)
    })
    .chain(
        ast.generics
            .params
            .clone()
            .into_iter()
            .map(|p| p.to_token_stream()),
    );

    Ok(quote! {
        #[automatically_derived]
        impl <#(#impl_generics),*> FromByteReader<#input> for #name #ty_generics #where_clause {
            fn from_byte_reader<R>(mut #reader: R) -> ::barse::Result<Self>
            where
                R: ::barse::ByteRead<#input>,
            {
                #body
            }
        }
    })
}
