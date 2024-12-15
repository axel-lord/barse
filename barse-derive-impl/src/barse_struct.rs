//! Implementation of derive for structs.

use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{
    parse_quote, punctuated::Punctuated, GenericParam, Generics, ItemStruct, PredicateType,
    TypeParamBound, WhereClause,
};

use crate::barse_struct::struct_config::StructConfig;

mod struct_config;

mod field_config;

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
        with,
        read_with,
        write_with,
        field_prefix,
    } = StructConfig::from_attrs(&item.attrs)?;
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

    let read_body = item.fields.iter().map(|field| {

    });

    let (impl_generics, ty_generics, split_where_clause) = item.generics.split_for_impl();
    let where_clause = where_clause.as_ref().or(split_where_clause);

    let e = format_ident!("__E_{r:X}");
    let b = format_ident!("__B_{r:X}");
    let to = format_ident!("__to_{r:x}");
    let from = format_ident!("__from_{r:x}");

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
                todo!()
            }

            fn write<#e, #b>(&self, #to: &mut #b, #write_with) -> ::core::result::Result<(), #error_path::<#b::Err>>
            where
                #e: #endian_path,
                #b: #byte_sink_path,
            {
                todo!()
            }
        }
    })
}
