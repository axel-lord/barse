//! Implementation of derive for structs.

use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{
    parse::Parser as _, parse_quote, punctuated::Punctuated, GenericParam, Generics, ItemStruct,
    Token, WhereClause,
};

use crate::{
    barse_field::{FieldDeps, ProcessedFields},
    impl_idents::ImplIdents,
    opt, path_expr,
    result_aggregate::ResAggr,
    Either,
};

opt::opt_parser! {
    /// Struct configuration.
    StructConfig {
        /// Replace where clause of barse impl.
        where_clause: opt::CustomWhere,

        /// Replace path to barse crate/module.
        barse_path: opt::BarsePath,

        /// Set a ReadWith and WriteWith value.
        with: opt::With,

        /// Set a ReadWith value.
        read_with: opt::ReadWith,

        /// Set a WriteWith value.
        write_with: opt::WriteWith,

        /// Set prefix prepended to field names in expressions. (_ by default for tuple structs).
        field_prefix: opt::FieldPrefix,

        /// Set a fixed endian in use by struct (fields may overwrite to another fixed endian).
        endian: opt::Endian,
    },
}

/// Derive barse for a struct.
///
/// # Errors
/// Should derive not be possible.
///
/// # Panics
/// On bad implementation.
pub fn derive_barse_struct(mut item: ItemStruct) -> Result<TokenStream, ::syn::Error> {
    let StructConfig {
        where_clause,
        barse_path,
        with,
        read_with,
        write_with,
        field_prefix,
        endian,
    } = StructConfig::default().parse_attrs(&item.attrs)?;

    let mut aggr = ResAggr::<()>::new();

    aggr.conflict(&read_with, &with)
        .conflict(&write_with, &with);

    let name = &item.ident;
    let field_prefix = field_prefix.map_or_else(
        || match item.fields {
            ::syn::Fields::Unnamed(_) => Some(format_ident!("_")),
            ::syn::Fields::Named(_) | ::syn::Fields::Unit => None,
        },
        |f| Some(f.field_prefix),
    );
    let barse_path = barse_path.map_or_else(
        || {
            ::syn::Path::parse_mod_style
                .parse2(quote! {::barse})
                .expect("::barse should be a valid path")
        },
        |p| p.path,
    );

    let impl_idents @ ImplIdents {
        _r,
        endian_ident,
        byte_ident,
        with_ident,
        to_ident,
        from_ident,
        discriminant_ident: _,
    } = &ImplIdents::new();

    let default_with = with.map_or_else(|| impl_idents.default_with(), |w| w.with_pat);

    let read_with = read_with.as_deref().unwrap_or(&default_with);
    let read_with_pat = read_with.pat.as_deref().unwrap_or(with_ident);
    let read_with_expr = path_expr(read_with_pat.clone());

    let write_with = write_with.as_deref().unwrap_or(&default_with);
    let write_with_pat = write_with.pat.as_deref().unwrap_or(with_ident);
    let write_with_expr = path_expr(write_with_pat.clone());

    let ProcessedFields {
        name_expansion,
        read_body,
        write_body,
    } = ProcessedFields::new(
        &item.fields,
        FieldDeps {
            field_prefix: field_prefix.as_ref(),
            barse_path: &barse_path,
            read_with_expr: &read_with_expr,
            write_with_expr: &write_with_expr,
            endian: endian.as_deref(),
            impl_idents,
        },
        &mut aggr,
    );

    let read_return = {
        match item.fields {
            ::syn::Fields::Named(_) => quote! {Ok(Self{#name_expansion})},
            ::syn::Fields::Unnamed(_) => quote! {Ok(Self(#name_expansion))},
            ::syn::Fields::Unit => quote! {Ok(Self)},
        }
    };

    let write_prefix = {
        match item.fields {
            ::syn::Fields::Named(_) => quote! { let Self { #name_expansion } = self; },
            ::syn::Fields::Unnamed(_) => quote! { let Self ( #name_expansion ) = self; },
            ::syn::Fields::Unit => TokenStream::default(),
        }
    };

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

            if !where_clause.predicates.empty_or_trailing() {
                where_clause.predicates.push_punct(<Token![,]>::default());
            }

            where_clause
                .predicates
                .push(parse_quote!(#ident: #barse_path::Barse));
        }
    }

    let (impl_generics, ty_generics, split_where_clause) = item.generics.split_for_impl();
    let where_clause = where_clause
        .as_ref()
        .map_or_else(|| Either::A(split_where_clause), Either::B);

    let read_with_ty = &read_with.ty;
    let write_with_ty = &write_with.ty;

    // Error if any minor errors encountered.
    aggr.into_inner()?;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #barse_path::Barse for #name #ty_generics #where_clause {
            type ReadWith = #read_with_ty;
            type WriteWith = #write_with_ty;

            fn read_with<#endian_ident, #byte_ident>(
                #from_ident: &mut #byte_ident,
                #read_with_pat: #read_with_ty
            ) -> ::core::result::Result<Self, #barse_path::WrappedErr::<#byte_ident::Err>>
            where
                #endian_ident: #barse_path::Endian,
                #byte_ident: #barse_path::ByteSource,
            {
                #read_body
                #read_return
            }

            fn write_with<#endian_ident, #byte_ident>(
                &self,
                #to_ident: &mut #byte_ident,
                #write_with_pat: #write_with_ty
            ) -> ::core::result::Result<(), #barse_path::WrappedErr::<#byte_ident::Err>>
            where
                #endian_ident: #barse_path::Endian,
                #byte_ident: #barse_path::ByteSink,
            {
                #write_prefix
                #write_body
                Ok(())
            }
        }
    })
}
