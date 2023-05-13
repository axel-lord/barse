use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::Parser, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DeriveInput, Expr,
    ExprLit, Lifetime, Meta, Token,
};

use crate::{dyn_mangle, dyn_mangle_display, static_mangle};

struct Ctx {
    pub attr_ident: Ident,
    pub flag_attr: Ident,
    pub from_attr: Ident,
    pub try_from_attr: Ident,
    pub reveal_attr: Ident,
    pub error_attr: Ident,
    pub reader_param: Ident,
    pub input_lifetime: Lifetime,
}

impl Default for Ctx {
    fn default() -> Self {
        Ctx {
            attr_ident: Ident::new("barse", Span::call_site()),
            flag_attr: Ident::new("flag", Span::call_site()),
            from_attr: Ident::new("from", Span::call_site()),
            try_from_attr: Ident::new("try_from", Span::call_site()),
            reveal_attr: Ident::new("reveal", Span::call_site()),
            error_attr: Ident::new("err", Span::call_site()),
            input_lifetime: {
                let ident = static_mangle("input");
                Lifetime {
                    apostrophe: Span::call_site(),
                    ident,
                }
            },
            reader_param: static_mangle("reader"),
        }
    }
}

fn parse_attrs<'a>(
    attrs: &'a [Attribute],
    ctx: &'a Ctx,
) -> impl 'a + Iterator<Item = Result<Meta, TokenStream>> {
    attrs.iter().filter(|attr| attr.path().is_ident(&ctx.attr_ident)).map(|attr| Ok(match &attr.meta {
            syn::Meta::List(list) => Punctuated::<Meta, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())
                .map_err(syn::Error::into_compile_error)?
                .into_iter(),
            value => {
                let span = value.span();
                return Err(quote_spanned! {
                    span=> compile_error!("barse attribute only takes the form of #[barse(name = \"value\", ..)]")
                });
            }
    })).flat_map(|meta| match meta {
        Err(err) => either::Left(std::iter::once(Err(err))),
        Ok(it) => either::Right(it.map(Ok)),
    })
}

fn invalid_attr_type(span: Span) -> TokenStream {
    quote_spanned! {
        span=> compile_error!(
            "barse attribute lists items are either of the form 'name = \"value\"' or 'name'
            example: #[barse(flag = \"name\", reveal)]"
            )
    }
}

#[derive(Default)]
struct StructAttrs {
    error: Option<syn::Type>,
}

fn parse_struct_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<StructAttrs, TokenStream> {
    let mut struct_attrs = StructAttrs::default();
    for item in parse_attrs(attrs, ctx) {
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

#[derive(Default)]
struct FieldAttrs {
    flags: Vec<syn::Expr>,
    reveal: Option<Span>,
    reveal_as: Vec<Ident>,
    parse_as: Option<syn::Path>,
    try_parse_as: Option<syn::Path>,
}
fn parse_field_attrs(attrs: &[Attribute], ctx: &Ctx) -> Result<FieldAttrs, TokenStream> {
    let mut field_attrs = FieldAttrs::default();
    for item in parse_attrs(attrs, ctx) {
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

                if item.path.is_ident(&ctx.flag_attr) {
                    let expr = lit_str
                        .parse::<syn::Expr>()
                        .map_err(syn::Error::into_compile_error)?;
                    field_attrs.flags.push(expr);
                } else if item.path.is_ident(&ctx.from_attr) {
                    let path = lit_str
                        .parse::<syn::Path>()
                        .map_err(syn::Error::into_compile_error)?;
                    field_attrs.parse_as = Some(path);
                } else if item.path.is_ident(&ctx.try_from_attr) {
                    let path = lit_str
                        .parse::<syn::Path>()
                        .map_err(syn::Error::into_compile_error)?;
                    field_attrs.try_parse_as = Some(path);
                } else if item.path.is_ident(&ctx.reveal_attr) {
                    let ident = lit_str
                        .parse::<syn::Ident>()
                        .map_err(syn::Error::into_compile_error)?;
                    field_attrs.reveal_as.push(ident);
                }
            }
        }
    }

    Ok(field_attrs)
}

fn variable_block(
    name: Option<&Ident>,
    mangled_name: &Ident,
    field_attrs: &[Attribute],
    ctx: &Ctx,
) -> Result<TokenStream, TokenStream> {
    let field_attrs = parse_field_attrs(field_attrs, ctx)?;

    let reader = &ctx.reader_param;

    let mut block = quote! {
        let #reader = &mut #reader;
    };
    if !field_attrs.flags.is_empty() {
        let flags = &field_attrs.flags;
        quote! {
            let #reader = ::barse::FlagByteReader::new(#reader, [#(#flags as &dyn ::std::any::Any),*]);
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

    let reveals =
        if let Some(span) = field_attrs.reveal {
            let name = name.ok_or_else(|| quote_spanned!{
                span=> compile_error!("bare reveal cannot be used on a struct without field names")
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

    let struct_attrs = parse_struct_attrs(&ast.attrs, &ctx)?;

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

                variable_block(Some(name), &mangled_name, &field.attrs, &ctx)?.to_tokens(&mut body);

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
                variable_block(None, &mangled_name, &field.attrs, &ctx)?.to_tokens(&mut body);

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

    let reader = &ctx.reader_param;
    let err = struct_attrs
        .error
        .as_ref()
        .map_or_else(|| quote!(::barse::error::Error), syn::Type::to_token_stream);
    Ok(quote! {
        #[automatically_derived]
        impl <#(#impl_generics),*> FromByteReader<#input_lifetime> for #name #ty_generics #where_clause {
            type Err = #err;
            fn from_byte_reader<R>(mut #reader: R) -> ::barse::Result<Self, Self::Err>
            where
                R: ::barse::ByteRead<#input_lifetime>,
            {
                #body
            }
        }
    })
}
