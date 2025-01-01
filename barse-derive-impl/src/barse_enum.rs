//! Implementation of derive for enums.

use ::proc_macro2::{Span, TokenStream};
use ::quote::{quote, ToTokens};
use ::syn::{
    parse::{Parse as _, Parser},
    parse_quote,
    punctuated::Punctuated,
    GenericParam, Generics, ItemEnum, Token, WhereClause,
};

use crate::{
    barse_field::{FieldDeps, ProcessedFields},
    impl_idents::ImplIdents,
    opt, path_expr,
    result_aggregate::ResAggr,
    Either,
};

opt::opt_parser! {
    /// Enum configuration.
    EnumConfig {
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

        /// Enum is read/written as discriminant then variant.
        discriminant: opt::EnumDiscriminant,
    },

    /// Enum variant configuration.
    VariantConfig {
        /// Set prefix prepended to field names in expressions. (_ by default for tuple structs).
        field_prefix: opt::FieldPrefix,

        /// Set a fixed endian in use by struct (fields may overwrite to another fixed endian).
        endian: opt::Endian,

        /// Condition for this variant.
        variant_if: opt::VariantIf,

        /// Discriminant of this variant.
        discriminant: opt::VariantDiscriminant,

        /// Field is ignored.
        ignore: opt::IgnoreField,
    },
}

/// Derive barse for an enum.
///
/// # Errors
/// Should derive not be possible.
#[expect(clippy::missing_panics_doc)]
pub fn derive_barse_enum(mut item: ItemEnum) -> Result<TokenStream, ::syn::Error> {
    let EnumConfig {
        where_clause,
        barse_path,
        with,
        read_with,
        write_with,
        field_prefix,
        endian,
        discriminant,
    } = EnumConfig::default().parse_attrs(&item.attrs)?;

    let name = &item.ident;
    let mut aggr = ResAggr::<()>::new();

    aggr.conflict(&read_with, &with)
        .conflict(&write_with, &with);

    let impl_idents @ ImplIdents {
        _r,
        endian_ident,
        byte_ident,
        with_ident,
        to_ident,
        from_ident,
        discriminant_ident,
    } = &ImplIdents::new();

    let underscore = ::syn::Ident::new("_", Span::call_site());
    let barse_path = barse_path.map_or_else(
        || {
            ::syn::Path::parse_mod_style
                .parse2(quote! {::barse})
                .expect("::barse should be a valid path")
        },
        |p| p.path,
    );

    let default_with = with.map_or_else(|| impl_idents.default_with(), |w| w.with_pat);
    let false_expr = ::syn::Expr::from(
        ::syn::ExprLit::parse
            .parse2(quote! {false})
            .expect("'false' should parse as a literal"),
    );

    let read_with = read_with.as_deref().unwrap_or(&default_with);
    let read_with_pat = read_with.pat.as_deref().unwrap_or(with_ident);
    let read_with_expr = path_expr(read_with_pat.clone());

    let write_with = write_with.as_deref().unwrap_or(&default_with);
    let write_with_pat = write_with.pat.as_deref().unwrap_or(with_ident);
    let write_with_expr = path_expr(write_with_pat.clone());

    let mut read_body = TokenStream::default();
    let mut write_body = TokenStream::default();

    let discr_endian = endian
        .as_deref()
        .map_or_else(|| Either::A(endian_ident), Either::B);
    if let Some(ty) = discriminant.as_deref() {
        quote! {
            let #discriminant_ident = <#ty as #barse_path::Barse>::read_with::<#discr_endian, _>(#from_ident, ())?;
        }
        .to_tokens(&mut read_body);
    }

    for variant in &item.variants {
        let cfg = match VariantConfig::default().parse_attrs(&variant.attrs) {
            Ok(cfg) => cfg,
            Err(err) => {
                aggr.push_err(err);
                continue;
            }
        };

        aggr.conflict(&discriminant, &cfg.variant_if)
            .conflict(&cfg.discriminant, &cfg.variant_if);

        let field_prefix =
            cfg.field_prefix
                .as_deref()
                .or(field_prefix.as_deref())
                .or(match variant.fields {
                    ::syn::Fields::Named(_) | ::syn::Fields::Unit => None,
                    ::syn::Fields::Unnamed(_) => Some(&underscore),
                });

        let variant_endian = cfg.endian.as_deref().or(endian.as_deref());

        let ProcessedFields {
            name_expansion,
            read_body: variant_read_body,
            write_body: variant_write_body,
        } = ProcessedFields::new(
            &variant.fields,
            FieldDeps {
                field_prefix,
                barse_path: &barse_path,
                read_with_expr: &read_with_expr,
                write_with_expr: &write_with_expr,
                endian: variant_endian,
                impl_idents,
            },
            &mut aggr,
        );

        let discriminant_value = cfg
            .discriminant
            .as_deref()
            .or_else(|| variant.discriminant.as_ref().map(|d| &d.1));

        let discriminant_expr = discriminant_value.map(|expr| {
            ::syn::Expr::from(
                ::syn::ExprBinary::parse
                    .parse2(quote! {#discriminant_ident == #expr})
                    .expect("'#discriminant_ident == #expr' should be a binary expression"),
            )
        });

        let variant_name = &variant.ident;

        if discriminant.is_some() && discriminant_expr.is_none() && cfg.ignore.is_none() {
            aggr.push_err(::syn::Error::new(
                variant.ident.span(),
                "discriminated barse enums should have \
                    explicit discriminants through either the '= $expr' \
                    syntax or #[barse(discriminant = $expr)] attribute",
            ));
        }

        let read_if_expr = discriminant_expr
            .as_ref()
            .or(cfg.variant_if.as_deref())
            .unwrap_or(&false_expr);
        let read_return = match variant.fields {
            ::syn::Fields::Named(_) => quote! { Self::#variant_name { #name_expansion } },
            ::syn::Fields::Unnamed(_) => quote! { Self::#variant_name ( #name_expansion ) },
            ::syn::Fields::Unit => quote! { Self::#variant_name },
        };
        quote! {
            if #read_if_expr {
                #variant_read_body
                return Ok( #read_return );
            }
        }
        .to_tokens(&mut read_body);

        let write_expansion = match &variant.fields {
            ::syn::Fields::Named(_) => quote! { Self::#variant_name { #name_expansion } },
            ::syn::Fields::Unnamed(_) => quote! { Self::#variant_name ( #name_expansion ) },
            ::syn::Fields::Unit => quote! { Self::#variant_name },
        };
        let write_discriminant = discriminant.as_deref().and_then(|ty| {
            let discr = discriminant_value?;
            Some(quote! {
                <#ty as #barse_path::Barse>::write_with::<#discr_endian, _>(&#discr, #to_ident, ())?;
            })
        });
        quote! {
            if let #write_expansion = self {
                #write_discriminant
                #variant_write_body
                return Ok(());
            }
        }
        .to_tokens(&mut write_body);
    }

    quote! {
        Err(#barse_path::WrappedErr::Other(#barse_path::Error::Msg("no read variant matched")))
    }
    .to_tokens(&mut read_body);
    quote! {
        Err(#barse_path::WrappedErr::Other(#barse_path::Error::Msg("no write variant matched")))
    }
    .to_tokens(&mut write_body);

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
                #write_body
            }
        }
    })
}
