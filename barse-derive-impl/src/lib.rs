//! Implementation of procedural macros for barse.

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

use std::fmt;

use proc_macro2::{Ident, TokenStream};
use quote::format_ident;

mod from_byte_reader;

/// Derive a `FromByteReader` implementation.
#[must_use]
pub fn derive_from_byte_reader(item: TokenStream) -> TokenStream {
    let ast = parse_as!(item as syn::DeriveInput);
    from_byte_reader::impl_trait(&ast).unwrap_or_else(syn::Error::into_compile_error)
}

fn dyn_mangle(ident: &Ident) -> Ident {
    format_ident!("__dyn_barse_derive_i{ident}")
}

fn dyn_mangle_display<D>(disp: D) -> Ident
where
    D: fmt::Display,
{
    format_ident!("__dyn_disp_barse_derive_i{disp}")
}

fn static_mangle(ident: &str) -> Ident {
    format_ident!("__static_barse_derive_i{ident}")
}

macro_rules! parse_as {
    ($e:path as $ty:ty) => {
        match syn::parse2::<$ty>($e) {
            Err(err) => return err.into_compile_error(),
            Ok(val) => val,
        }
    };
}
use parse_as;
