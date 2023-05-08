use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::ItemFn;

pub fn generate_impl(name: &Ident, body: &ItemFn) -> TokenStream {
    let (fn_name, ty) = super::fn_name_and_type(body);

    quote! {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub struct #name;
        impl ::parse_common::ByteSizeQuery for #name {
            type Flag = #ty;
            fn size(flag: &Self::Flag) -> usize {
                #fn_name(flag)
            }
        }

    }
}
