//! [FieldConfig] impl.

use ::syn::{spanned::Spanned, Attribute};

use crate::opt;

/// Field configuration determined by attribute.
#[derive(Debug, Default)]
pub struct FieldConfig {
    /// Field is ignored.
    pub ignore: Option<opt::IgnoreField>,

    /// Given with value is used instead of '()'.
    pub with: Option<opt::FieldWith>,
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

            opt::parse_opts!(meta_list.tokens.clone(), cfg.ignore, cfg.with);
        }
        Ok(cfg)
    }
}
