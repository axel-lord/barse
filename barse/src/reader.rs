//! Implementations of [`crate::ByteRead`][ByteRead].

#![allow(clippy::module_name_repetitions)]

pub use const_endian_byte_reader::ConstEndianByteReader;
pub use cursor::Cursor;
pub use endian_byte_reader::EndianByteReader;

mod const_endian_byte_reader;
mod cursor;
mod endian_byte_reader;

mod impl_box;
mod impl_mut_ref;
