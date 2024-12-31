#![doc = include_str!("../README.md")]

use ::proc_macro2::TokenStream;
use ::quote::ToTokens;
use ::syn::{spanned::Spanned, DataEnum, DataStruct, DeriveInput, ItemEnum, ItemStruct};

use crate::{barse_enum::derive_barse_enum, barse_struct::derive_barse_struct};

mod kw {
    //! Custom keywords.

    use ::syn::custom_keyword;

    custom_keyword!(barse);
    custom_keyword!(barse_path);
    custom_keyword!(with);
    custom_keyword!(read_with);
    custom_keyword!(write_with);
    custom_keyword!(field_prefix);
    custom_keyword!(endian);
    custom_keyword!(ignore);
    custom_keyword!(bytes);
    custom_keyword!(read_bytes);
    custom_keyword!(write_bytes);
    custom_keyword!(read_as);
    custom_keyword!(write_as);
    custom_keyword!(discriminant);
}

mod barse_enum;

mod barse_struct;

mod opt;

mod result_aggregate;

pub mod barse_field;

/// Derive barse for a struct or enum.
pub fn derive_barse(item: TokenStream) -> TokenStream {
    ::syn::parse2(item)
        .and_then(derive_barse_impl)
        .unwrap_or_else(::syn::Error::into_compile_error)
}

/// Create an expression from something that may be turned into a path.
fn path_expr(path: impl Into<::syn::Path>) -> ::syn::Expr {
    ::syn::ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path.into(),
    }
    .into()
}

/// ToTokens implementor that may be one of two types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<A, B> {
    /// First type.
    A(A),

    /// Second type.
    B(B),
}

impl<A, B> Either<A, B> {
    /// Swap order of A and B values.
    pub fn swapped(self) -> Either<B, A> {
        match self {
            Either::A(a) => Either::B(a),
            Either::B(b) => Either::A(b),
        }
    }

    /// Map A value.
    pub fn map_a<F, T>(self, map: F) -> Either<T, B>
    where
        F: FnOnce(A) -> T,
    {
        match self {
            Either::A(a) => Either::A(map(a)),
            Either::B(b) => Either::B(b),
        }
    }

    /// Map B value.
    pub fn map_b<F, T>(self, map: F) -> Either<A, T>
    where
        F: FnOnce(B) -> T,
    {
        self.swapped().map_a(map).swapped()
    }
}

impl<A, B> ToTokens for Either<A, B>
where
    A: ToTokens,
    B: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Either::A(a) => a.to_tokens(tokens),
            Either::B(b) => b.to_tokens(tokens),
        }
    }
}

/// Split up derive input based on kind of type.
///
/// # Errors
/// Should derive not be possible.
fn derive_barse_impl(derive_input: DeriveInput) -> Result<TokenStream, ::syn::Error> {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = derive_input;
    match data {
        ::syn::Data::Struct(DataStruct {
            struct_token,
            fields,
            semi_token,
        }) => derive_barse_struct(ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
            semi_token,
        }),
        ::syn::Data::Enum(DataEnum {
            enum_token,
            brace_token,
            variants,
        }) => derive_barse_enum(ItemEnum {
            attrs,
            vis,
            enum_token,
            ident,
            generics,
            brace_token,
            variants,
        }),
        ::syn::Data::Union(data_union) => Err(::syn::Error::new(
            data_union.union_token.span(),
            "Barse trait cannot be derived for union types",
        )),
    }
}
