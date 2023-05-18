use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Data, DeriveInput, Lifetime};

use crate::static_mangle;

mod parse_attrs;
mod parse_fields;
mod parse_struct_attrs;

pub struct Ctx {
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

pub fn impl_trait(ast: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;

    let Data::Struct(data_struct) = &ast.data else {
        return Err(syn::Error::new(ast.span(), "FromByteReader can only be derived for structs"));
    };

    let ctx = Ctx::default();

    let struct_attrs = parse_struct_attrs::parse_struct_attrs(&ast.attrs, &ctx)?;
    let body = parse_fields::parse_fields(data_struct, &ctx)?;

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
