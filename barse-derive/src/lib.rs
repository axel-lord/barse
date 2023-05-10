//! Procedural macros for barse.

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::disallowed_types,
    clippy::indexing_slicing,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::default_numeric_fallback,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::float_equality_without_abs,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::impl_trait_in_params,
    clippy::mixed_read_write_in_expression,
    clippy::mod_module_files,
    clippy::multiple_unsafe_ops_per_block,
    clippy::undocumented_unsafe_blocks,
    clippy::partial_pub_fields,
    clippy::panic,
    clippy::semicolon_if_nothing_returned,
    clippy::semicolon_inside_block,
    clippy::str_to_string,
    clippy::todo,
    clippy::try_err,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::fallible_impl_from,
    clippy::future_not_send,
    clippy::option_if_let_else,
    clippy::or_fun_call,
    clippy::path_buf_push_overwrite,
    clippy::redundant_pub_crate,
    clippy::redundant_allocation,
    clippy::significant_drop_tightening,
    clippy::useless_let_if_seq,
    rustdoc::all,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, FnArg, ItemFn, Type, TypeReference};

mod condition;
mod from_byte_reader;
mod size_query;
mod vec_len_query;

fn simplify_result<T>(res: Result<T, T>) -> T {
    match res {
        Ok(t) | Err(t) => t,
    }
}

fn dyn_mangle(ident: &Ident) -> Ident {
    format_ident!("__dyn_barse_derive_{ident}")
}

fn static_mangle(ident: &str) -> Ident {
    format_ident!("__static_barse_derive_{ident}")
}

/// Derive a `FromByteReader` implementation.
#[proc_macro_derive(FromByteReader, attributes(barse))]
pub fn derive_from_byte_reader(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    simplify_result(from_byte_reader::impl_trait(&ast)).into()
}

/// Create a `ByteSizeQuery` implementor from a function.
#[proc_macro_attribute]
pub fn byte_size_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as Ident);
    let body = parse_macro_input!(item as ItemFn);

    let gen_trait = simplify_result(size_query::generate_impl(&name, &body));

    quote! {
        #body
        #gen_trait
    }
    .into()
}

/// Create a Condition implementor from a function.
#[proc_macro_attribute]
pub fn condition(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as Ident);
    let body = parse_macro_input!(item as ItemFn);

    let gen_trait = simplify_result(condition::generate_impl(&name, &body));

    quote! {
        #body
        #gen_trait
    }
    .into()
}

/// Create a `VecLenQuery` implementor from a function.
#[proc_macro_attribute]
pub fn vec_len_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = parse_macro_input!(attr as Ident);
    let body = parse_macro_input!(item as ItemFn);

    let gen_trait = simplify_result(vec_len_query::generate_impl(&name, &body));

    quote! {
        #body
        #gen_trait
    }
    .into()
}

fn fn_name_and_type(body: &ItemFn) -> Result<(&Ident, &Type), proc_macro2::TokenStream> {
    let fn_name = &body.sig.ident;

    if !body.sig.generics.params.is_empty() {
        let span = body.sig.generics.span();
        return Err(quote_spanned! {
            span=> compile_error!("annotated function can not have generic params or lifetimes")
        });
    }

    if body.sig.inputs.len() != 1 {
        let span = body.sig.inputs.span();
        return Err(quote_spanned! {
            span=> compile_error!("annotated function should have one and only have one parameter")
        });
    }

    let Some(FnArg::Typed(flag_param)) = &body.sig.inputs.first() else {
        let span = body.sig.inputs.span();
        return Err(quote_spanned!{
            span=> compile_error!("annotated function should have a non-self parameter")
        });
    };

    let Type::Reference(TypeReference {
        lifetime: None,
        mutability: None,
        elem: ty,
        ..
    }) = &*flag_param.ty else {
        let span = flag_param.span();
        return Err(quote_spanned!{
            span=> compile_error!("annotaded function should have it's param be a immutable reference with no specified lifetime")
        });
    };

    Ok((fn_name, ty))
}
