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

pub use byte_read::{ByteRead, NilReader};
pub use const_endian_byte_reader::ConstEndianByteReader;
pub use endian::Endian;
pub use endian_byte_reader::EndianByteReader;
pub use flag_byte_reader::FlagByteReader;
pub use flag_conditional::{Condition, FlagConditional};
pub use from_byte_reader::FromByteReader;
pub use from_reader_slice::{ByteSizeQuery, FromReaderSlice};
pub use padding::Padding;
pub use sized_vec::{FromReaderVec, VecLenQuery};

/// Result type in use by crate.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[cfg(feature = "derive")]
pub use barse_derive::FromByteReader;

mod byte_read;
mod const_endian_byte_reader;
mod endian;
mod endian_byte_reader;
mod flag_byte_reader;
mod flag_conditional;
mod from_byte_reader;
mod from_reader_slice;
mod padding;
mod sized_vec;

pub mod error;

pub mod prelude {
    //! Prelude module with all traits in use by crate.
    pub use super::{ByteRead, ByteSizeQuery, Condition, FromByteReader, VecLenQuery};
}

#[cfg(feature = "derive")]
pub mod attribute {
    //! Attribute macros.
    pub use barse_derive::{condition, len_query, size_query};
}
