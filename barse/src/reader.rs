//! Implementations of [`crate::ByteRead`][ByteRead].

pub use const_endian_byte_reader::ConstEndianByteReader;
pub use dynamic_byte_reader::DynamicByteReader;
pub use endian_byte_reader::EndianByteReader;
pub use flag_byte_reader::FlagByteReader;
pub use nil_byte_reader::NilByteReader;

mod const_endian_byte_reader;
mod dynamic_byte_reader;
mod endian_byte_reader;
mod flag_byte_reader;
mod nil_byte_reader;

mod impl_box;
mod impl_cursor;
mod impl_mut_ref;
