use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput, Lifetime};

pub fn mangle(ident: &Ident) -> Ident {
    format_ident!("__parse_derive_{ident}")
}

fn field_init(reader: &Ident) -> TokenStream {
    quote! {
        ::barse::FromByteReader::from_byte_reader(&mut #reader)?
    }
}

pub fn impl_from_byte_reader(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let Data::Struct(data_struct) = &ast.data else {
        panic!("FromByteReader can only be derived for structs");
    };

    let reader = mangle(&Ident::new("reader", Span::call_site()));
    let input = {
        let ident = mangle(&Ident::new("input", Span::call_site()));
        Lifetime {
            apostrophe: Span::call_site(),
            ident,
        }
    };

    let body: TokenStream = match data_struct.fields {
        syn::Fields::Named(ref fields) => {
            let field_init =
                std::iter::from_fn(|| Some(field_init(&reader))).take(fields.named.len());

            let field_names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());

            quote! {
                Ok(Self{#(#field_names: #field_init),*} )
            }
        }
        syn::Fields::Unnamed(ref fields) => {
            let field_init =
                std::iter::from_fn(|| Some(field_init(&reader))).take(fields.unnamed.len());

            quote! {
                Ok(Self(#(#field_init),*))
            }
        }
        syn::Fields::Unit => quote! {Ok(Self)},
    };

    let (_, ty_generics, where_clause) = ast.generics.split_for_impl();

    let lifetimes = ast.generics.lifetimes().collect::<Vec<_>>();
    let impl_generics = std::iter::once(if lifetimes.is_empty() {
        quote!(#input)
    } else {
        quote!(#input: #(#lifetimes),*)
    })
    .chain(
        ast.generics
            .params
            .clone()
            .into_iter()
            .map(|p| p.to_token_stream()),
    );

    quote! {
        #[automatically_derived]
        impl <#(#impl_generics),*> FromByteReader<#input> for #name #ty_generics #where_clause {
            fn from_byte_reader<R>(mut #reader: R) -> ::barse::Result<Self>
            where
                R: ::barse::ByteRead<#input>,
            {
                #body
            }
        }
    }
}
