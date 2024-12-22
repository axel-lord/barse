//! [FieldConfig] impl.

use ::syn::{spanned::Spanned, Attribute};

use crate::opt;

/// Field configuration determined by attribute.
#[derive(Debug, Default)]
pub struct FieldConfig {
    /// Field is ignored.
    pub ignore: Option<opt::IgnoreField>,

    /// Given expression is used instead of '()'.
    pub with: Option<opt::FieldWith>,

    /// Given expression is used instead of '()'.
    pub read_with: Option<opt::FieldReadWith>,

    /// Given expression is used instead of '()'.
    pub write_with: Option<opt::FieldWriteWith>,

    /// Field endian.
    pub endian: Option<opt::Endian>,
}

impl FieldConfig {
    /// Get config from field attributes
    ///
    /// # Errors
    /// If any invalid field attributes are encountered.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        let mut cfg = FieldConfig::default();
        for attr in attrs {
            if !attr.path().is_ident("barse") {
                continue;
            }

            let meta_list = attr.meta.require_list().map_err(|_| {
                ::syn::Error::new(attr.meta.span(), "expected list attribute: #[barse(...)]")
            })?;

            opt::parse_opts!(
                meta_list.tokens.clone(),
                cfg.ignore,
                cfg.with,
                cfg.read_with,
                cfg.write_with,
                cfg.endian,
            );
        }
        Ok(cfg)
    }
}
