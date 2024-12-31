//! Implementation of derive for structs.

use ::proc_macro2::TokenStream;
use ::quote::{format_ident, quote};
use ::syn::{
    parse_quote, punctuated::Punctuated, GenericParam, Generics, ItemStruct, Token, WhereClause,
};

use crate::{opt, path_expr, result_aggregate::ResAggr, Either};

opt::opt_parser! {
    /// Struct field configuration.
    FieldConfig {
        /// Field is ignored.
        ignore: opt::IgnoreField,

        /// Given expression is used instead of '()'.
        with: opt::FieldWith,

        /// Given expression is used instead of '()'.
        read_with: opt::FieldReadWith,

        /// Given expression is used instead of '()'.
        write_with: opt::FieldWriteWith,

        /// Field endian.
        endian: opt::Endian,

        /// Bytes.
        bytes: opt::Bytes,

        /// Read bytes.
        read_bytes: opt::ReadBytes,

        /// Write bytes.
        write_bytes: opt::WriteBytes,

        /// Read using provided impl.
        read_as: opt::ReadAs,

        /// Write using provided impl.
        write_as: opt::WriteAs,

        /// Read/Write using provided impl.
        barse_as: opt::BarseAs,
    },

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
    let barse_path = barse_path.map_or_else(|| parse_quote!(::barse), |p| p.path);

    let r = ::rand::random::<u32>();

    let endian_ident = format_ident!("__E_{r:X}");
    let byte_ident = format_ident!("__B_{r:X}");
    let with_ident = format_ident!("__with_{r:x}");
    let to_ident = format_ident!("__to_{r:x}");
    let from_ident = format_ident!("__from_{r:x}");

    let default_with = with.map_or_else(|| parse_quote!(#with_ident: ()), |w| w.with_pat);
    let default_with_pat = default_with.pat.as_deref().unwrap_or(&with_ident);
    let default_with_expr = path_expr(default_with_pat.clone());

    let read_with = read_with.as_deref().unwrap_or(&default_with);
    let read_with_pat = read_with.pat.as_deref().unwrap_or(&with_ident);
    let read_with_expr = path_expr(read_with_pat.clone());

    let write_with = write_with.as_deref().unwrap_or(&default_with);
    let write_with_pat = write_with.pat.as_deref().unwrap_or(&with_ident);
    let write_with_expr = path_expr(write_with_pat.clone());

    let fields = item
        .fields
        .iter()
        .enumerate()
        .filter_map(|(i, field)| {
            let cfg = match FieldConfig::default().parse_attrs(&field.attrs) {
                Ok(cfg) => cfg,
                Err(err) => {
                    aggr.push_err(err);
                    return None;
                }
            };

            aggr.conflict(&cfg.read_with, &cfg.with)
                .conflict(&cfg.write_with, &cfg.with)
                .conflict(&cfg.read_as, &cfg.barse_as)
                .conflict(&cfg.write_as, &cfg.barse_as);

            if cfg!(not(feature = "barse_as")) {
                const BARSE_AS: &str = "barse_as";
                aggr.requires_feature(BARSE_AS, &cfg.barse_as)
                    .requires_feature(BARSE_AS, &cfg.read_as)
                    .requires_feature(BARSE_AS, &cfg.write_as);
            }

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
            Some((field, cfg, name))
        })
        .collect::<Vec<_>>();

    let read_body = fields
        .iter()
        .map(|(field, cfg, name)| {
            if let Some(ignore) = &cfg.ignore {
                let expr = ignore.value.as_ref().map_or_else(
                    || Either::A(quote! { ::core::default::Default::default() }),
                    |value| Either::B(&value.value),
                );

                quote! {
                    let #name = #expr;
                }
            } else if let Some(count) = cfg.read_bytes.as_deref().or(cfg.bytes.as_deref()) {
                let ty = &field.ty;

                quote! {
                    let mut #name = [0u8; #count];
                    <#byte_ident as #barse_path::ByteSource>::read_slice(#from_ident, &mut #name)?;
                    let #name = <#ty as ::core::convert::From<[u8; #count]>>::from(#name);
                }
            } else {
                let ty = &field.ty;

                let read_with = cfg
                    .read_with
                    .as_ref()
                    .map(|w| w.expr.as_deref().unwrap_or(&default_with_expr))
                    .or(cfg
                        .with
                        .as_ref()
                        .map(|w| w.expr.as_deref().unwrap_or(&read_with_expr)))
                    .map_or_else(|| Either::A(quote! {()}), Either::B);

                let e = cfg
                    .endian
                    .as_ref()
                    .or(endian.as_ref())
                    .map_or_else(|| Either::A(&endian_ident), |e| Either::B(&e.endian));

                if let Some(using) = cfg.read_as.as_deref().or(cfg.barse_as.as_deref()) {
                    quote! {
                        let #name = #barse_path::ReadAs::<#ty, _>::read_with::<#e, #byte_ident>({ #using }, #from_ident, #read_with)?;
                    }
                } else {
                    quote! {
                        let #name = <#ty as #barse_path::Barse>::read_with::<#e, #byte_ident>(#from_ident, #read_with)?;
                    }
                }
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
        .map(|(field, cfg, name)| {
            if cfg.ignore.is_some() {
                quote! { _ = #name; }
            } else if cfg.bytes.is_some() || cfg.write_bytes.is_some() {
                let ty = &field.ty;
                quote! {{
                    let #name = <#ty as ::core::convert::AsRef<[u8]>>::as_ref(#name);
                    <#byte_ident as #barse_path::ByteSink>::write_slice(#to_ident, #name)?;
                }}
            } else {
                let ty = &field.ty;

                let write_with = cfg
                    .write_with
                    .as_ref()
                    .map(|w| w.expr.as_deref().unwrap_or(&default_with_expr))
                    .or(cfg
                        .with
                        .as_ref()
                        .map(|w| w.expr.as_deref().unwrap_or(&write_with_expr)))
                    .map_or_else(|| Either::A(quote! {()}), Either::B);

                let e = cfg
                    .endian
                    .as_ref()
                    .or(endian.as_ref())
                    .map_or_else(|| Either::A(&endian_ident), |e| Either::B(&e.endian));

                if let Some(using) =cfg.write_as.as_deref().or(cfg.barse_as.as_deref()) {
                    quote! {
                        #barse_path::WriteAs::<#ty, _>::write_with::<#e, #byte_ident>({ #using }, #name, #to_ident, #write_with)?;
                    }
                } else {
                    quote! {
                        <#ty as #barse_path::Barse>::write_with::<#e, #byte_ident>(#name, #to_ident, #write_with)?;
                    }
                }
            }
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
