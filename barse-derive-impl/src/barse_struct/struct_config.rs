//! [StructConfig] impl.

use ::quote::ToTokens;
use ::syn::{
    meta::ParseNestedMeta, parenthesized, parse_quote, parse_quote_spanned, punctuated::Punctuated,
    spanned::Spanned, Attribute, Token, WhereClause, WherePredicate,
};

/// Struct-wide configuration of barse, determined by attributes.
#[derive(Debug)]
pub struct StructConfig {
    /// Path to Barse trait.
    pub barse_path: ::syn::Path,

    /// Path to ByteSink trait.
    pub byte_sink_path: ::syn::Path,

    /// Path to ByteSource trait.
    pub byte_source_path: ::syn::Path,

    /// Path to ::barse::Error type.
    pub error_path: ::syn::Path,

    /// Path to Endian trait.
    pub endian_path: ::syn::Path,

    /// Manual where clause.
    pub where_clause: Option<::syn::WhereClause>,
}

/// Parse a path config value.
///
/// If the meta does not have name as it's ident Ok(false) will be returned,
/// else Ok(true) will be returned.
///
/// # Errors
/// If the path is not a name = value attribute
/// or if it is already set.
fn parse_path(
    name: &str,
    path: &mut Option<::syn::Path>,
    meta: &ParseNestedMeta,
) -> Result<bool, ::syn::Error> {
    if !meta.path.is_ident(name) {
        return Ok(false);
    };

    if let Some(path) = path {
        return Err(meta.error(format!(
            "{name} is already set to '{}'",
            path.to_token_stream()
        )));
    }

    *path = Some(meta.value()?.parse()?);

    Ok(true)
}

impl StructConfig {
    /// Parse struct attributes.
    ///
    /// # Errors
    /// If any invalid barse attributes are encountered.
    pub fn parse_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        let mut barse_path: Option<::syn::Path> = None;
        let mut byte_source_path: Option<::syn::Path> = None;
        let mut byte_sink_path: Option<::syn::Path> = None;
        let mut error_path: Option<::syn::Path> = None;
        let mut endian_path: Option<::syn::Path> = None;
        let mut where_clause: Option<::syn::WhereClause> = None;

        for attr in attrs {
            if !attr.path().is_ident("barse") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if parse_path("barse_path", &mut barse_path, &meta)? {
                    return Ok(());
                }

                if parse_path("byte_source_path", &mut byte_source_path, &meta)? {
                    return Ok(());
                }

                if parse_path("byte_sink_path", &mut byte_sink_path, &meta)? {
                    return Ok(());
                }

                if parse_path("error_path", &mut error_path, &meta)? {
                    return Ok(());
                }

                if parse_path("endian_path", &mut endian_path, &meta)? {
                    return Ok(());
                }

                if meta.path.is_ident("where") {
                    let content;
                    parenthesized!(content in meta.input);
                    let predicates =
                        Punctuated::<WherePredicate, Token![,]>::parse_terminated(&content)?;

                    where_clause = Some(WhereClause {
                        where_token: parse_quote_spanned! {meta.path.span()=> where},
                        predicates,
                    });
                    return Ok(());
                };

                if meta.path.is_ident("crate_path") {
                    let path: syn::Path = meta.value()?.parse()?;

                    barse_path.get_or_insert_with(|| parse_quote!(#path :: Barse));
                    byte_source_path.get_or_insert_with(|| parse_quote!(#path :: ByteSource));
                    byte_sink_path.get_or_insert_with(|| parse_quote!(#path :: ByteSink));
                    error_path.get_or_insert_with(|| parse_quote!(#path :: Error));
                    endian_path.get_or_insert_with(|| parse_quote!(#path :: Endian));
                    return Ok(());
                };

                Err(meta.error(format!(
                    "attribute '{}' is unknown/does not apply to structs",
                    meta.path.to_token_stream()
                )))
            })?;
        }

        Ok(StructConfig {
            barse_path: barse_path.unwrap_or_else(|| parse_quote!(::barse::Barse)),
            byte_sink_path: byte_sink_path.unwrap_or_else(|| parse_quote!(::barse::ByteSink)),
            byte_source_path: byte_source_path.unwrap_or_else(|| parse_quote!(::barse::ByteSource)),
            error_path: error_path.unwrap_or_else(|| parse_quote!(::barse::Error)),
            endian_path: endian_path.unwrap_or_else(|| parse_quote!(::barse::Endian)),
            where_clause,
        })
    }
}
