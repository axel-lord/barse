//! Helper types implementing [Barse][crate::Barse] for common usages.

pub use self::{
    byte_array::ByteArray,
    fixed_size::FixedSize,
    padding::Padding,
    slice_sink::{SliceSink, SliceSinkFull},
    slice_source::{SliceSrc, SliceSrcEmpty},
    use_endian::UseEndian,
    with_endian::WithEndian,
};

mod byte_array;

mod use_endian;

mod padding;

mod slice_source;

mod slice_sink;

mod with_endian;

mod fixed_size;
