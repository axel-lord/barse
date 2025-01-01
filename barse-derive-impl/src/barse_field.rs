//! Shared field processing.

use ::proc_macro2::{Span, TokenStream};
use ::quote::{format_ident, ToTokens};
use quote::quote;

use crate::{impl_idents::ImplIdents, opt, result_aggregate::ResAggr, unit_expr, Either};

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
}

/// Fields that have been processed.
#[derive(Debug, Default)]
pub struct ProcessedFields {
    /// 'ident: name,' expansion for fields.
    pub name_expansion: TokenStream,

    /// read body for fields.
    pub read_body: TokenStream,

    /// write body for fields.
    pub write_body: TokenStream,
}

/// Values that need to be supplied to fields.
#[derive(Debug)]
pub struct FieldDeps<'a> {
    /// Field prefix, may be set to avoid collisions with globals.
    pub field_prefix: Option<&'a ::syn::Ident>,

    /// Path to barse crate/module.
    pub barse_path: &'a ::syn::Path,

    /// With expression used when 'read_with' is used alone.
    pub read_with_expr: &'a ::syn::Expr,

    /// With expression used whe 'write_with' is used alone.
    pub write_with_expr: &'a ::syn::Expr,

    /// Endian path.
    pub endian: Option<&'a ::syn::Path>,

    /// Impl idents.
    pub impl_idents: &'a ImplIdents,
}

impl ProcessedFields {
    /// Generate code from fields.
    pub fn new(fields: &::syn::Fields, deps: FieldDeps, aggr: &mut ResAggr) -> ProcessedFields {
        let mut f = ProcessedFields::default();
        let ProcessedFields {
            name_expansion,
            read_body,
            write_body,
        } = &mut f;
        let FieldDeps {
            field_prefix,
            barse_path,
            read_with_expr,
            endian,
            write_with_expr,
            impl_idents:
                ImplIdents {
                    _r,
                    endian_ident,
                    byte_ident,
                    with_ident: _,
                    to_ident,
                    from_ident,
                    discriminant_ident: _,
                },
        } = deps;
        let default_expr = unit_expr();

        for (i, field) in fields.iter().enumerate() {
            let cfg = match FieldConfig::default().parse_attrs(&field.attrs) {
                Ok(cfg) => cfg,
                Err(err) => {
                    aggr.push_err(err);
                    continue;
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

            let name = match (&field.ident, field_prefix) {
                // No prefix, normal fields.
                (Some(ident), None) => ident.clone(),

                // Prefix, normal fields.
                (Some(ident), Some(field_prefix)) => format_ident!("{field_prefix}{ident}"),

                // No Prefix, tuple fields.
                (None, None) => {
                    aggr.push_err(::syn::Error::new(
                        Span::call_site(),
                        "field prefix needs to exist for tuple structs",
                    ));
                    format_ident!("_{i}")
                }

                // Prefix, tuple fields.
                (None, Some(field_prefix)) => format_ident!("{field_prefix}{i}"),
            };

            field
                .ident
                .as_ref()
                .filter(|&ident| ident != &name)
                .map_or_else(
                    || {
                        quote! { #name, }
                    },
                    |ident| {
                        quote! { #ident: #name, }
                    },
                )
                .to_tokens(name_expansion);

            let ty = &field.ty;

            if let Some(ignore) = &cfg.ignore {
                // Field should be ignored.
                let expr = ignore.value.as_ref().map_or_else(
                    || Either::A(quote! { <#ty as ::core::default::Default>::default() }),
                    |value| Either::B(&value.value),
                );

                quote! {
                    let #name = #expr;
                }
                .to_tokens(read_body);
                quote! {
                    _ = #name;
                }
                .to_tokens(write_body);
                continue;
            }

            let e = cfg
                .endian
                .as_deref()
                .or(endian)
                .map_or_else(|| Either::A(&endian_ident), Either::B);

            if let Some(count) = cfg.read_bytes.as_deref().or(cfg.bytes.as_deref()) {
                // Field should be read as bytes.
                quote! {
                    let mut #name = [0u8; #count];
                    <#byte_ident as #barse_path::ByteSource>::read_slice(#from_ident, &mut #name)?;
                    let #name = <#ty as ::core::convert::From<[u8; #count]>>::from(#name);
                }
                .to_tokens(read_body);
            } else {
                // Field is read as either barse or AsRead
                let with_expr = cfg
                    .with
                    .as_ref()
                    .map(|w| w.expr.as_deref().unwrap_or(read_with_expr));

                let read_with = cfg
                    .read_with
                    .as_ref()
                    .map(|w| w.expr.as_deref().unwrap_or(read_with_expr))
                    .or(with_expr)
                    .unwrap_or(&default_expr);

                let call_expr = cfg
                    .read_as
                    .as_deref()
                    .or(cfg.barse_as.as_deref())
                    .map_or_else(
                        || {
                            quote! {
                                <#ty as #barse_path::Barse>::read_with::<#e, #byte_ident>(
                                    #from_ident,
                                    #read_with
                                )
                            }
                        },
                        |using| {
                            quote! {
                                #barse_path::ReadAs::<#ty, _>::read_with::<#e, #byte_ident>(
                                    { #using },
                                    #from_ident,
                                    #read_with
                                )
                            }
                        },
                    );

                quote! { let #name = #call_expr?; }.to_tokens(read_body);
            }

            if cfg.bytes.is_some() || cfg.write_bytes.is_some() {
                // Field should be written as bytes.
                quote! {{
                    let #name = <#ty as ::core::convert::AsRef<[u8]>>::as_ref(#name);
                    <#byte_ident as #barse_path::ByteSink>::write_slice(#to_ident, #name)?;
                }}
                .to_tokens(write_body);
            } else {
                let with_expr = cfg
                    .with
                    .as_ref()
                    .map(|w| w.expr.as_deref().unwrap_or(write_with_expr));

                let write_with = cfg
                    .write_with
                    .as_ref()
                    .map(|w| w.expr.as_deref().unwrap_or(write_with_expr))
                    .or(with_expr)
                    .unwrap_or(&default_expr);

                if let Some(using) = cfg.write_as.as_deref().or(cfg.barse_as.as_deref()) {
                    quote! {
                        #barse_path::WriteAs::<#ty, _>::write_with::<#e, #byte_ident>(
                            { #using },
                            #name,
                            #to_ident,
                            #write_with
                        )?;
                    }
                } else {
                    quote! {
                        <#ty as #barse_path::Barse>::write_with::<#e, #byte_ident>(
                            #name,
                            #to_ident,
                            #write_with
                        )?;
                    }
                }
                .to_tokens(write_body);
            }
        }
        f
    }
}
