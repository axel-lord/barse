use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::ItemFn;

pub fn generate_impl(name: &Ident, body: &ItemFn) -> TokenStream {
    let (fn_name, ty) = super::fn_name_and_type(body);

    quote! {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
        pub struct #name;
        impl ::barse::Condition for #name {
            type Flag = #ty;
            fn verify(flag: &Self::Flag) -> bool {
                #fn_name(flag)
            }
        }

    }
}
