use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Type, TypeReference};

#[proc_macro_derive(FromByteReader)]
pub fn derive_from_byte_reader(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    from_byte_reader::impl_from_byte_reader(&ast).into()
}

#[proc_macro_attribute]
pub fn size_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as Ident);
    let body = parse_macro_input!(item as ItemFn);

    let gen_trait = size_query::generate_impl(&name, &body);

    quote! {
        #body
        #gen_trait
    }
    .into()
}

#[proc_macro_attribute]
pub fn condition(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as Ident);
    let body = parse_macro_input!(item as ItemFn);

    let gen_trait = condition::generate_impl(&name, &body);

    quote! {
        #body
        #gen_trait
    }
    .into()
}

fn fn_name_and_type(body: &ItemFn) -> (&Ident, &Type) {
    let fn_name = &body.sig.ident;

    assert!(
        body.sig.generics.params.is_empty(),
        "annotated function can not be generic"
    );

    assert!(
        body.sig.inputs.len() == 1,
        "annotated function should only have one parameter"
    );

    let Some(FnArg::Typed(flag_param)) = &body.sig.inputs.first() else {
        panic!("annotated function should have a non-self parameter");
    };

    let Type::Reference(TypeReference {
        lifetime: None,
        mutability: None,
        elem: ty,
        ..
    }) = &*flag_param.ty else {
        panic!("annotaded function should have it's \
               param be a immutable reference with no \
               specified lifetime")
    };

    (fn_name, ty)
}

mod condition;
mod from_byte_reader;
mod size_query;
