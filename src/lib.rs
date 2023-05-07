use proc_macro::TokenStream;

#[proc_macro_derive(FromByteReader)]
pub fn derive_from_byte_reader(item: TokenStream) -> TokenStream {
    let ast = syn::parse(item).unwrap();
    from_byte_reader::impl_from_byte_reader(&ast).into()
}

mod from_byte_reader;
