//! Implementation of derive for structs.

use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{
    parse_quote, punctuated::Punctuated, GenericParam, Generics, ItemStruct, PredicateType,
    TypeParamBound, WhereClause,
};

use crate::barse_struct::{field_config::FieldConfig, struct_config::StructConfig};

mod struct_config;

mod field_config;

/// Derive barse for a struct.
///
/// # Errors
/// Should derive not be possible.
///
/// # Panics
/// On bad implementation.
pub fn derive_barse_struct(mut item: ItemStruct) -> Result<TokenStream, ::syn::Error> {
    let StructConfig {
        barse_path,
        byte_sink_path,
        byte_source_path,
        error_path,
        endian_path,
        where_clause,
        with,
        read_with,
        write_with,
        field_prefix,
    } = StructConfig::from_attrs(&item.attrs)?;
    let name = &item.ident;
    let field_prefix = field_prefix.or_else(|| match item.fields {
        ::syn::Fields::Unnamed(_) => Some(format_ident!("_")),
        ::syn::Fields::Named(_) | ::syn::Fields::Unit => None,
    });

    let r = ::rand::random::<u32>();

    if where_clause.is_none() {
        let Generics {
            params,
            where_clause,
            ..
        } = &mut item.generics;
        for param in params {
            let GenericParam::Type(param) = param else {
                continue;
            };
            let where_clause = where_clause.get_or_insert_with(|| WhereClause {
                where_token: Default::default(),
                predicates: Punctuated::new(),
            });

            let ident = &param.ident;

            where_clause
                .predicates
                .push(::syn::WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: parse_quote!(#ident),
                    colon_token: Default::default(),
                    bounds: [TypeParamBound::Trait(parse_quote!(#barse_path))]
                        .into_iter()
                        .collect(),
                }));
        }
    }

    let e = format_ident!("__E_{r:X}");
    let b = format_ident!("__B_{r:X}");
    let to = format_ident!("__to_{r:x}");
    let from = format_ident!("__from_{r:x}");

    let fields = item
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let cfg = FieldConfig::from_attrs(&field.attrs);
            let name = field.ident.as_ref().map_or_else(
                || {
                    field_prefix.as_ref().map_or_else(
                        || panic!("field_prefix needs to exist for tuple structs"),
                        |field_prefix| format_ident!("{field_prefix}{i}"),
                    )
                },
                |ident| {
                    field_prefix.as_ref().map_or_else(
                        || ident.clone(),
                        |field_prefix| format_ident!("{field_prefix}{ident}"),
                    )
                },
            );
            (field, cfg, name)
        })
        .collect::<Vec<_>>();

    let read_body = fields
        .iter()
        .map(|(field, _cfg, name)| {
            let ty = &field.ty;
            quote! {
                let #name = <#ty as #barse_path>::read::<#e, #b>(#from, ())?;
            }
        })
        .collect::<TokenStream>();

    let read_return = {
        let fields = fields.iter().map(|(field, _cfg, name)| {
            field
                .ident
                .as_ref()
                .filter(|&ident| ident != name)
                .map_or_else(
                    || {
                        quote! { #name }
                    },
                    |ident| {
                        quote! { #ident: #name }
                    },
                )
        });

        match item.fields {
            ::syn::Fields::Named(_) => quote! {Ok(Self{#(#fields),*})},
            ::syn::Fields::Unnamed(_) => quote! {Ok(Self(#(#fields),*))},
            ::syn::Fields::Unit => quote! {Ok(Self)},
        }
    };

    let write_body = fields
        .iter()
        .map(|(field, _cfg, name)| {
            let ty = &field.ty;
            quote! { <#ty as #barse_path>::write::<#e, #b>(#name, #to, ())?; }
        })
        .collect::<TokenStream>();

    let write_prefix = {
        let fields = fields.iter().map(|(field, _cfg, name)| {
            field
                .ident
                .as_ref()
                .filter(|&ident| ident != name)
                .map_or_else(|| quote! { #name }, |ident| quote! { #ident: #name })
        });

        match item.fields {
            ::syn::Fields::Named(_) => quote! { let Self { #(#fields),* } = self; },
            ::syn::Fields::Unnamed(_) => quote! { let Self ( #(#fields),* ) = self; },
            ::syn::Fields::Unit => TokenStream::default(),
        }
    };

    let (impl_generics, ty_generics, split_where_clause) = item.generics.split_for_impl();
    let where_clause = where_clause.as_ref().or(split_where_clause);

    let with = with.unwrap_or_else(|| {
        let with = format_ident!("__with_{r:x}");
        parse_quote!(#with: ())
    });

    let read_with = read_with.as_ref().unwrap_or(&with);
    let write_with = write_with.as_ref().unwrap_or(&with);

    let read_with_ty = &read_with.ty;
    let write_with_ty = &write_with.ty;

    Ok(quote! {
        impl #impl_generics #barse_path for #name #ty_generics #where_clause {
            type ReadWith = #read_with_ty;
            type WriteWith = #write_with_ty;

            fn read<#e, #b>(#from: &mut #b, #read_with) -> ::core::result::Result<Self, #error_path::<#b::Err>>
            where
                #e: #endian_path,
                #b: #byte_source_path,
            {
                #read_body
                #read_return
            }

            fn write<#e, #b>(&self, #to: &mut #b, #write_with) -> ::core::result::Result<(), #error_path::<#b::Err>>
            where
                #e: #endian_path,
                #b: #byte_sink_path,
            {
                #write_prefix
                #write_body
                Ok(())
            }
        }
    })
}
