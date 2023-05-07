use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput};

pub fn impl_from_byte_reader(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let Data::Struct(data_struct) = &ast.data else {
        panic!("FromByteReader can only be derived for structs");
    };

    let reader = Ident::new("__reader__", Span::call_site());

    let body: TokenStream = match data_struct.fields {
        syn::Fields::Named(ref fields) => {
            let field_init = fields.named.iter().map(|field| {
                let ty = &field.ty;
                quote! {
                    <#ty as ::parse_common::FromByteReader>::from_byte_reader(&mut #reader)?
                }
            });

            let field_names = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().unwrap());

            quote! {
                Ok(Self{#(#field_names: #field_init),*} )
            }
        }
        syn::Fields::Unnamed(ref fields) => {
            let field_init = fields.unnamed.iter().map(|field| {
                let ty = &field.ty;
                quote! {
                    <#ty as ::parse_common::FromByteReader>::from_byte_reader(&mut #reader)?
                }
            });

            quote! {
                Ok(Self(#(#field_init),*))
            }
        }
        syn::Fields::Unit => quote! {Ok(Self)},
    };

    let stripped_generics = ast.generics.lt_token.is_some().then(|| {
        let stripped_generics = ast.generics.params.iter().map(|param| match param {
            syn::GenericParam::Lifetime(lt) => lt.lifetime.to_token_stream(),
            syn::GenericParam::Type(ty) => ty.ident.to_token_stream(),
            syn::GenericParam::Const(co) => co.ident.to_token_stream(),
        });

        quote! {
            <#(#stripped_generics),*>
        }
    });

    let generics = ast
        .generics
        .lt_token
        .is_some()
        .then(|| ast.generics.params.iter().map(|param| quote! {#param}))
        .into_iter()
        .flatten();

    let where_clause = ast.generics.where_clause.as_ref();

    quote! {
        impl <'__input #(,#generics)*> FromByteReader<'__input> for #name #stripped_generics #where_clause {
            fn from_byte_reader<R>(mut #reader: R) -> ::parse_common::Result<Self>
            where
                R: ::parse_common::ByteRead<'__input>,
            {
                #body
            }
        }
    }
}
