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

/// Derive a `FromByteReader` implementation.
#[proc_macro_derive(FromByteReader, attributes(barse))]
pub fn derive_from_byte_reader(item: TokenStream) -> TokenStream {
    barse_derive_impl::derive_from_byte_reader(item.into()).into()
}
