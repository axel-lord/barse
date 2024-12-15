//! [StructConfig] impl.

use ::quote::ToTokens;
use ::syn::{
    meta::ParseNestedMeta, parenthesized, parse::Parse, parse_quote, parse_quote_spanned,
    punctuated::Punctuated, spanned::Spanned, Attribute, Ident, Token, WhereClause, WherePredicate,
};

/// With pattern.
#[derive(Debug)]
pub struct WithPat {
    /// Pattern to bind with type to.
    pub pat: ::syn::Pat,

    /// Colon separating pattern and type.
    pub colon_token: Token![:],

    /// Type of with value.
    pub ty: ::syn::Type,
}

impl Parse for WithPat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: ::syn::Pat::parse_single(input)?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl ToTokens for WithPat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pat.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

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

    /// With pattern.
    pub with: Option<WithPat>,

    /// ReadWith pattern.
    pub read_with: Option<WithPat>,

    /// WriteWith pattern.
    pub write_with: Option<WithPat>,

    /// field_prefix.
    pub field_prefix: Option<Ident>,
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
    /// Get config from struct attributes.
    ///
    /// # Errors
    /// If any invalid barse attributes are encountered.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        let mut barse_path: Option<::syn::Path> = None;
        let mut byte_source_path: Option<::syn::Path> = None;
        let mut byte_sink_path: Option<::syn::Path> = None;
        let mut error_path: Option<::syn::Path> = None;
        let mut endian_path: Option<::syn::Path> = None;
        let mut where_clause: Option<::syn::WhereClause> = None;
        let mut with: Option<WithPat> = None;
        let mut read_with: Option<WithPat> = None;
        let mut write_with: Option<WithPat> = None;
        let mut field_prefix: Option<Ident> = None;

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
                    if where_clause.is_some() {
                        return Err(meta.error("'where' has already been set"));
                    }
                    
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

                if meta.path.is_ident("with") {
                    if with.is_some() {
                        return Err(meta.error("'with' has already been set"));
                    }
                    if read_with.is_some() || write_with.is_some() {
                        return Err(meta
                            .error("'with' may not be combined with 'read_with' or 'write_with'"));
                    }
                    with = Some(meta.value()?.parse()?);
                    return Ok(());
                }

                if meta.path.is_ident("write_with") {
                    if write_with.is_some() {
                        return Err(meta.error("'write_with' has already been set"));
                    }
                    if with.is_some() {
                        return Err(meta
                            .error("'with' may not be combined with 'read_with' or 'write_with'"));
                    }
                    write_with = Some(meta.value()?.parse()?);
                    return Ok(());
                }

                if meta.path.is_ident("read_with") {
                    if read_with.is_some() {
                        return Err(meta.error("'read_with' has already been set"));
                    }
                    if with.is_some() {
                        return Err(meta
                            .error("'with' may not be combined with 'read_with' or 'write_with'"));
                    }
                    read_with = Some(meta.value()?.parse()?);
                    return Ok(());
                }

                if meta.path.is_ident("field_prefix") {
                    if field_prefix.is_some() {
                        return Err(meta.error("'field_prefix' has already been set"));
                    }

                    field_prefix = Some(meta.value()?.parse()?);

                    return Ok(());
                }

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
            with,
            read_with,
            write_with,
            field_prefix,
        })
    }
}
