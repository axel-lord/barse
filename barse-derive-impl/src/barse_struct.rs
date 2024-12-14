//! Implementation of derive for structs.

use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{
    parse_quote, punctuated::Punctuated, GenericParam, Generics, ItemStruct, PredicateType,
    TypeParamBound, WhereClause,
};

use crate::barse_struct::struct_config::StructConfig;

mod struct_config;

/// Derive barse for a struct.
///
/// # Errors
/// Should derive not be possible.
pub fn derive_barse_struct(mut item: ItemStruct) -> Result<TokenStream, ::syn::Error> {
    let StructConfig {
        barse_path,
        byte_sink_path,
        byte_source_path,
        error_path,
        endian_path,
        where_clause,
    } = StructConfig::parse_attrs(&item.attrs)?;
    let name = &item.ident;

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

    let (impl_generics, ty_generics, split_where_clause) = item.generics.split_for_impl();
    let where_clause = where_clause.as_ref().or(split_where_clause);

    let e = format_ident!("__E_{r:X}");
    let b = format_ident!("__B_{r:X}");
    let to = format_ident!("__to_{r:x}");
    let from = format_ident!("__from_{r:x}");
    let with = format_ident!("__with_{r:x}");

    Ok(quote! {
        impl #impl_generics #barse_path for #name #ty_generics #where_clause {
            type ReadWith = ();
            type WriteWith = ();

            fn read<#e, #b>(#from: &mut #b, #with: Self::ReadWith) -> ::core::result::Result<Self, #error_path::<#b::Err>>
            where
                #e: #endian_path,
                #b: #byte_source_path,
            {
                todo!()
            }

            fn write<#e, #b>(&self, #to: &mut #b, #with: Self::WriteWith) -> ::core::result::Result<(), #error_path::<#b::Err>>
            where
                #e: #endian_path,
                #b: #byte_sink_path,
            {
                todo!()
            }
        }
    })
}
