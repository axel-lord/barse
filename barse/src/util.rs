//! Helper types implementing [Barse][crate::Barse] for common usages.

pub use self::{
    byte_array::ByteArray, fixed_size::FixedSize, padding::Padding, slice_sink::SliceSink,
    slice_source::SliceSrc, use_endian::UseEndian,
};

#[cfg(feature = "alloc")]
pub use self::byte_array::boxed_byte_array::ByteBox;

pub mod error;

mod byte_array;

mod use_endian;

mod padding;

mod slice_source;

mod slice_sink;

mod fixed_size;
