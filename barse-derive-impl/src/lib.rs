#[doc = include_str!("../README.md")]
use ::proc_macro2::TokenStream;
use ::syn::{spanned::Spanned, DataEnum, DataStruct, DeriveInput, ItemEnum, ItemStruct};

/// Derive barse for a struct or enum.
pub fn derive_barse(item: TokenStream) -> TokenStream {
    ::syn::parse2(item)
        .and_then(derive_barse_impl)
        .unwrap_or_else(::syn::Error::into_compile_error)
}

fn derive_barse_impl(derive_input: DeriveInput) -> Result<TokenStream, ::syn::Error> {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    match data {
        ::syn::Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => derive_barse_struct(ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        }),
        ::syn::Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => derive_barse_enum(ItemEnum {
            attrs,
            vis,
            enum_token,
            ident,
            generics,
            brace_token,
            variants,
        }),
        ::syn::Data::Union(data_union) => Err(::syn::Error::new(
            data_union.union_token.span(),
            "Barse cannot be derived for Union.",
        )),
    }
}

fn derive_barse_enum(_item: ItemEnum) -> Result<TokenStream, ::syn::Error> {
    todo!()
}

fn derive_barse_struct(_item: ItemStruct) -> Result<TokenStream, ::syn::Error> {
    todo!()
}
