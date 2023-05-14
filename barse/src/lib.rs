//! Library for parsing binary data.

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::disallowed_types,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
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

// Utility
pub use endian::Endian;
pub use error::Error;

// Traits
pub use byte_read::ByteRead;
pub use from_byte_reader::FromByteReader;
pub use from_reader::{
    flag_conditional::Condition, from_reader_slice::ByteSizeQuery, sized_vec::VecLenQuery,
};

// ByteRead

#[cfg(feature = "derive")]
pub use barse_derive::FromByteReader;

/// Result type in use by crate.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

// Utility
mod endian;
mod error;

// Traits
mod byte_read;
mod from_byte_reader;

// FromByteReader
pub mod from_reader;

// ByteRead
pub mod reader;

pub mod prelude {
    //! Prelude module with all traits in use by crate.
    pub use super::{ByteRead, ByteSizeQuery, Condition, FromByteReader, VecLenQuery};
}

#[cfg(feature = "derive")]
pub mod attribute {
    //! Attribute macros.
    pub use barse_derive::{byte_size_query, condition, vec_len_query};
}
