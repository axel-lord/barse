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
    pub with_attr: Ident,
    pub reader_param: Ident,
    pub from_byte_reader_trait: Ident,
    pub from_byte_reader_with_trait: Ident,
    pub from_byte_reader_method: Ident,
    pub from_byte_reader_with_method: Ident,
    pub input_lifetime: Lifetime,
}

impl Default for Ctx {
    fn default() -> Self {
        pub fn id(val: &str) -> Ident {
            Ident::new(val, Span::call_site())
        }

        pub fn lt(ident: Ident) -> syn::Lifetime {
            syn::Lifetime {
                apostrophe: Span::call_site(),
                ident,
            }
        }
        Ctx {
            attr_ident: id("barse"),
            flag_attr: id("flag"),
            from_attr: id("from"),
            try_from_attr: id("try_from"),
            reveal_attr: id("reveal"),
            error_attr: id("err"),
            with_attr: id("with"),
            reader_param: static_mangle("reader"),
            from_byte_reader_trait: id("FromByteReader"),
            from_byte_reader_with_trait: id("FromByteReaderWith"),
            from_byte_reader_method: id("from_byte_reader"),
            from_byte_reader_with_method: id("from_byte_reader_with"),
            input_lifetime: lt(static_mangle("input")),
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
enum ParseAs {
    #[default]
    No,
    Yes(syn::Path),
    Try(syn::Path),
}

#[derive(Default)]
struct FieldAttrs {
    flags: Vec<syn::Expr>,
    reveal: Option<Span>,
    reveal_as: Vec<Ident>,
    parse_as: ParseAs,
    with: Option<syn::Expr>,
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
        ParseAs::No => {
            quote! {
                ::barse::#trait_name::#method_call?
            }
            .to_tokens(&mut block);
        }
        ParseAs::Yes(path) => {
            quote! {
                <#path as ::barse::#trait_name>::#method_call?.into()
            }
            .to_tokens(&mut block);
        }
        ParseAs::Try(path) => {
            quote! {
                <#path as ::barse::#trait_name>::#method_call?.try_into()?
            }
            .to_tokens(&mut block);
        }
    }

    // reveal
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
        .map_or_else(|| quote!(::barse::Error), syn::Type::to_token_stream);
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

macro_rules! ident_map {
    ($check:expr, {$($against:expr => $on_match: expr),+ $(,)?}) => {
        '__ident_map: {
            $(
            if $check.is_ident($against) {
                $on_match;
                break '__ident_map;
            }
            )*
        }
    };
}
use ident_map;
