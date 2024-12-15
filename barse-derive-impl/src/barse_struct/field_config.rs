//! [FieldConfig] impl.

use ::quote::ToTokens;
use ::syn::Attribute;

/// Field configuration determined by attribute.
#[derive(Debug)]
pub struct FieldConfig {}

impl FieldConfig {
    /// Get config from field attributes
    ///
    /// # Errors
    /// If any invalid field attributes are encountered.
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Self, ::syn::Error> {
        for attr in attrs {
            if !attr.path().is_ident("barse") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                Err(meta.error(format!(
                    "attribute '{}' is unkown/does not apply to struct fields",
                    meta.path.to_token_stream()
                )))
            })?;
        }
        Ok(Self {})
    }
}
