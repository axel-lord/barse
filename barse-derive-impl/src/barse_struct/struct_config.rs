//! [StructConfig] impl.

use ::syn::Attribute;

use crate::opt;

/// Struct-wide configuration of barse, determined by attributes.
#[derive(Debug, Default)]
pub struct StructConfig {
    /// Manual where clause.
    pub where_clause: Option<opt::CustomWhere>,

    /// Path to Barse trait.
    pub barse_path: Option<opt::BarsePath>,

    /// With pattern.
    pub with: Option<opt::With>,

    /// ReadWith pattern.
    pub read_with: Option<opt::ReadWith>,

    /// WriteWith pattern.
    pub write_with: Option<opt::WriteWith>,

    /// field_prefix.
    pub field_prefix: Option<opt::FieldPrefix>,

    /// endian override.
    pub endian: Option<opt::Endian>,
}

impl StructConfig {
    /// Get config from struct attributes.
    ///
    /// # Errors
    /// If any invalid barse attributes are encountered.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        let mut cfg = StructConfig::default();
        opt::parse_attrs(attrs, |tokens| {
            opt::parse_opts!(
                tokens,
                cfg.where_clause,
                cfg.barse_path,
                cfg.with,
                cfg.read_with,
                cfg.write_with,
                cfg.field_prefix,
                cfg.endian
            );
            Ok(())
        })?;
        Ok(cfg)
    }
}
