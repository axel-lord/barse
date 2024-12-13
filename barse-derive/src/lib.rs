use ::proc_macro::TokenStream;

/// Derive barse for a struct or enum.
#[proc_macro_derive(Barse, attributes(barse))]
pub fn derive_barse(item: TokenStream) -> TokenStream {
    ::barse_derive_impl::derive_barse(item.into()).into()
}
