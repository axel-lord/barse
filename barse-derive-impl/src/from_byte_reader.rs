use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Lifetime};

use crate::static_mangle;

use self::struct_attrs::StructAttrs;

mod parse_attrs;
mod parse_fields;
mod struct_attrs;

pub struct Ctx {
    pub mod_name: Ident,
    pub attr_ident: Ident,
    pub flag_attr: Ident,
    pub from_attr: Ident,
    pub try_from_attr: Ident,
    pub reveal_attr: Ident,
    pub error_attr: Ident,
    pub with_attr: Ident,
    pub reader_param: Ident,
    pub byte_read_trait: Ident,
    pub from_byte_reader_trait: Ident,
    pub from_byte_reader_with_trait: Ident,
    pub from_byte_reader_method: Ident,
    pub from_byte_reader_with_method: Ident,
    pub input_lifetime: Lifetime,
    pub with_param_name: Ident,
}

pub struct TraitInfo<'a> {
    pub trait_kind: &'a Ident,
    pub fn_name: &'a Ident,
    pub fn_args: Option<TokenStream>,
}

impl<'a> TraitInfo<'a> {
    pub fn new(with: Option<&syn::Type>, ctx: &'a Ctx) -> Self {
        with.map_or_else(
            || TraitInfo {
                trait_kind: &ctx.from_byte_reader_trait,
                fn_name: &ctx.from_byte_reader_method,
                fn_args: None,
            },
            |with_type| {
                let with_param_name = &ctx.with_param_name;
                TraitInfo {
                    trait_kind: &ctx.from_byte_reader_with_trait,
                    fn_name: &ctx.from_byte_reader_with_method,
                    fn_args: Some(quote! { #with_param_name: #with_type }),
                }
            },
        )
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Ctx {
            mod_name: id("barse"),
            attr_ident: id("barse"),
            flag_attr: id("flag"),
            from_attr: id("from"),
            try_from_attr: id("try_from"),
            reveal_attr: id("reveal"),
            error_attr: id("err"),
            with_attr: id("with"),
            reader_param: static_mangle("reader"),
            byte_read_trait: id("ByteRead"),
            from_byte_reader_trait: id("FromByteReader"),
            from_byte_reader_with_trait: id("FromByteReaderWith"),
            from_byte_reader_method: id("from_byte_reader"),
            from_byte_reader_with_method: id("from_byte_reader_with"),
            input_lifetime: lt(static_mangle("input")),
            with_param_name: static_mangle("with"),
        }
    }
}

pub fn id(val: &str) -> Ident {
    Ident::new(val, Span::call_site())
}
pub fn lt(ident: Ident) -> syn::Lifetime {
    syn::Lifetime {
        apostrophe: Span::call_site(),
        ident,
    }
}

pub struct TraitImpl<'ast, 'ctx> {
    pub ctx: &'ctx Ctx,
    pub name: &'ast Ident,
    pub generics: &'ast syn::Generics,
    pub input_lifetime_param: syn::GenericParam,
    pub body: TokenStream,
    pub err: TokenStream,
    pub trait_info: TraitInfo<'ast>,
}

impl<'ast: 'ctx, 'ctx> TraitImpl<'ast, 'ctx> {
    pub fn new(ast: &'ast DeriveInput, ctx: &'ast Ctx) -> Result<Self, syn::Error> {
        let name = &ast.ident;

        let Data::Struct(data_struct) = &ast.data else {
            return Err(syn::Error::new(
                ast.span(),
                "FromByteReader can only be derived for structs",
            ))
        };

        let struct_attrs = StructAttrs::new(&ast.attrs, ctx)?;

        let body = parse_fields::parse_fields(data_struct, ctx)?;

        let trait_info = TraitInfo::new(struct_attrs.with.as_ref(), ctx);

        let generics = &ast.generics;

        let mod_name = &ctx.mod_name;

        let err = struct_attrs
            .error
            .as_ref()
            .map_or_else(|| quote!(::#mod_name::Error), syn::Type::to_token_stream);

        let input_lifetime_param = {
            let mut input_lifetime_param = syn::LifetimeParam::new(ctx.input_lifetime.clone());
            let mut lifetimes = ast.generics.lifetimes().peekable();

            if lifetimes.peek().is_some() {
                input_lifetime_param.colon_token = Some(syn::token::Colon::default());
                input_lifetime_param.bounds = lifetimes
                    .map(|lifetime| lifetime.lifetime.clone())
                    .collect();
            }
            syn::GenericParam::Lifetime(input_lifetime_param)
        };

        Ok(Self {
            ctx,
            name,
            generics,
            input_lifetime_param,
            body,
            err,
            trait_info,
        })
    }
}

impl<'ast, 'ctx> ToTokens for TraitImpl<'ast, 'ctx> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            ctx,
            generics,
            body,
            input_lifetime_param,
            err,
            trait_info:
                TraitInfo {
                    trait_kind,
                    fn_name,
                    fn_args,
                },
        } = self;

        let Ctx {
            mod_name,
            reader_param,
            byte_read_trait,
            input_lifetime,
            ..
        } = ctx;

        let impl_generics = std::iter::once(input_lifetime_param).chain(&generics.params);

        let (_, ty_generics, where_clause) = generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl <#(#impl_generics),*> ::#mod_name::#trait_kind <#input_lifetime> for #name #ty_generics #where_clause {
                type Err = #err;
                fn #fn_name <R>(mut #reader_param: R, #fn_args) -> ::#mod_name::Result<Self, Self::Err>
                where
                    R: ::#mod_name::#byte_read_trait<#input_lifetime>,
                {
                    #body
                }
            }
        }.to_tokens(tokens);
    }
}

pub fn impl_trait(ast: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let ctx = Ctx::default();

    Ok(TraitImpl::new(ast, &ctx)?.into_token_stream())
}
