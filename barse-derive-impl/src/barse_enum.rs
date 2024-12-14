//! Implementation of derive for enums.

use ::proc_macro2::TokenStream;
use ::syn::ItemEnum;

/// Derive barse for an enum.
///
/// # Errors
/// Should derive not be possible.
pub fn derive_barse_enum(_item: ItemEnum) -> Result<TokenStream, ::syn::Error> {
    todo!()
}
