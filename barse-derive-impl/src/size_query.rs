use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::ItemFn;

pub fn generate_impl(name: &Ident, body: &ItemFn) -> Result<TokenStream, TokenStream> {
    let (fn_name, ty) = super::fn_name_and_type(body)?;

    Ok(quote! {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub struct #name;
        impl ::barse::ByteSizeQuery for #name {
            type Flag = #ty;
            fn size(flag: &Self::Flag) -> usize {
                #fn_name(flag)
            }
        }
    })
}
