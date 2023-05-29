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
}

struct TraitInfo<'a> {
    trait_kind: &'a Ident,
    fn_name: &'a Ident,
    fn_args: Option<TokenStream>,
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
        }
    }
}

impl<'a> TraitInfo<'a> {
    pub fn new(with: Option<&syn::Type>, ctx: &'a Ctx) -> Self {
        with.map_or_else(
            || TraitInfo {
                trait_kind: &ctx.from_byte_reader_trait,
                fn_name: &ctx.from_byte_reader_method,
                fn_args: None,
            },
            |_| TraitInfo {
                trait_kind: &ctx.from_byte_reader_with_trait,
                fn_name: &ctx.from_byte_reader_with_method,
                fn_args: None,
            },
        )
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

pub fn impl_trait(ast: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;

    let Data::Struct(data_struct) = &ast.data else {
        return Err(syn::Error::new(ast.span(), "FromByteReader can only be derived for structs"));
    };

    let ctx = Ctx::default();

    let struct_attrs = StructAttrs::new(&ast.attrs, &ctx)?;
    let body = parse_fields::parse_fields(data_struct, &ctx)?;

    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

    let input_lifetime = &ctx.input_lifetime;
    let lifetimes = ast.generics.lifetimes().collect::<Vec<_>>();
    let impl_generics = std::iter::once(if lifetimes.is_empty() {
        input_lifetime.to_token_stream()
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

    let mod_name = &ctx.mod_name;

    let reader = &ctx.reader_param;
    let err = struct_attrs
        .error
        .as_ref()
        .map_or_else(|| quote!(::#mod_name::Error), syn::Type::to_token_stream);

    let TraitInfo {
        trait_kind,
        fn_name,
        fn_args,
    } = TraitInfo::new(struct_attrs.with.as_ref(), &ctx);

    let byte_read = &ctx.byte_read_trait;

    Ok(quote! {
        #[automatically_derived]
        impl <#(#impl_generics),*> ::#mod_name::#trait_kind <#input_lifetime> for #name #ty_generics #where_clause {
            type Err = #err;
            fn #fn_name <R>(mut #reader: R, #fn_args) -> ::#mod_name::Result<Self, Self::Err>
            where
                R: ::#mod_name::#byte_read<#input_lifetime>,
            {
                #body
            }
        }
    })
}
