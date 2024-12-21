//! [StructConfig] impl.

use ::quote::ToTokens;
use ::syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Ident, Token, WhereClause,
};

use crate::opt::{self, WithPat};

/// Struct-wide configuration of barse, determined by attributes.
#[derive(Debug)]
pub struct StructConfig {
    /// Manual where clause.
    pub where_clause: Option<::syn::WhereClause>,

    /// Path to Barse trait.
    pub barse_path: Option<::syn::Path>,

    /// With pattern.
    pub with: Option<WithPat>,

    /// ReadWith pattern.
    pub read_with: Option<WithPat>,

    /// WriteWith pattern.
    pub write_with: Option<WithPat>,

    /// field_prefix.
    pub field_prefix: Option<Ident>,

    /// endian override.
    pub endian: Option<::syn::Path>,
}

/// Struct option.
#[derive(Debug)]
enum StructOpt {
    /// Where clause of Barse impl.
    WhereClause(WhereClause),

    /// Path to barse module.
    BarsePath(opt::BarsePath),

    /// With pattern of type.
    With(opt::With),

    /// ReadWith pattern of type.
    ReadWith(opt::ReadWith),

    /// WriteWith pattern of type.
    WriteWith(opt::WriteWith),

    /// Field prefix.
    FieldPrefix(opt::FieldPrefix),

    /// Force endian.
    Endian(opt::Endian),
}

impl Parse for StructOpt {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        Ok(if lookahead.peek(Token![where]) {
            StructOpt::WhereClause(input.parse()?)
        } else if opt::BarsePath::peek(&lookahead) {
            StructOpt::BarsePath(input.parse()?)
        } else if opt::With::peek(&lookahead) {
            StructOpt::With(input.parse()?)
        } else if opt::ReadWith::peek(&lookahead) {
            StructOpt::ReadWith(input.parse()?)
        } else if opt::WriteWith::peek(&lookahead) {
            StructOpt::WriteWith(input.parse()?)
        } else if opt::FieldPrefix::peek(&lookahead) {
            StructOpt::FieldPrefix(input.parse()?)
        } else if opt::Endian::peek(&lookahead) {
            StructOpt::Endian(input.parse()?)
        } else {
            return Err(lookahead.error());
        })
    }
}

impl StructOpt {
    /// Parse options.
    ///
    /// # Errors
    /// If syntaxt expectations are not met.
    fn parse_opts(
        input: ParseStream<'_>,
    ) -> Result<Punctuated<StructOpt, Token![,]>, ::syn::Error> {
        let mut out = Punctuated::new();

        while !input.is_empty() {
            out.push(input.parse()?);

            if matches!(out.last(), Some(StructOpt::WhereClause(..))) && !input.is_empty() {
                return Err(input.error("epxected nothing following where clause"));
            }

            if input.peek(Token![,]) {
                out.push_punct(input.parse()?);
                continue;
            }

            if !input.is_empty() {
                return Err(input.error("expected ',' between entries"));
            }
        }

        Ok(out)
    }
}

/// Set an option if not already set.
///
/// # Errors
/// If the option has been set.
fn set_opt<T: ToTokens>(
    name: impl ToTokens,
    opt: &mut Option<T>,
    value: T,
) -> Result<(), ::syn::Error> {
    if let Some(value) = opt {
        return Err(::syn::Error::new(
            value.span(),
            format!(
                "'{}' has already been set to '{}'",
                name.to_token_stream(),
                value.to_token_stream()
            ),
        ));
    };
    *opt = Some(value);
    Ok(())
}

impl StructConfig {
    /// Get config from struct attributes.
    ///
    /// # Errors
    /// If any invalid barse attributes are encountered.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        let mut where_clause: Option<WhereClause> = None;
        let mut barse_path = None;
        let mut with = None;
        let mut read_with = None;
        let mut write_with = None;
        let mut field_prefix = None;
        let mut endian = None;

        for attr in attrs {
            if !attr.path().is_ident("barse") {
                continue;
            }

            let meta_list = attr.meta.require_list().map_err(|_| {
                ::syn::Error::new(attr.meta.span(), "expected list attribute: #[barse(...)]")
            })?;

            for opt in StructOpt::parse_opts.parse2(meta_list.tokens.clone())? {
                match opt {
                    StructOpt::WhereClause(where_clause_opt) => {
                        if let Some(where_clause) = where_clause {
                            return Err(::syn::Error::new(
                                where_clause.where_token.span(),
                                "where clause already set",
                            ));
                        } else {
                            where_clause = Some(where_clause_opt);
                        }
                    }
                    StructOpt::BarsePath(opt) => set_opt(opt.kw, &mut barse_path, opt.path)?,
                    StructOpt::With(opt) => set_opt(opt.kw, &mut with, opt.with_pat)?,
                    StructOpt::ReadWith(opt) => set_opt(opt.kw, &mut read_with, opt.with_pat)?,
                    StructOpt::WriteWith(opt) => set_opt(opt.kw, &mut write_with, opt.with_pat)?,
                    StructOpt::FieldPrefix(opt) => {
                        set_opt(opt.kw, &mut field_prefix, opt.field_prefix)?
                    }
                    StructOpt::Endian(opt) => set_opt(opt.kw, &mut endian, opt.endian)?,
                }
            }
        }

        Ok(StructConfig {
            where_clause,
            barse_path,
            with,
            read_with,
            write_with,
            field_prefix,
            endian,
        })
    }
}
