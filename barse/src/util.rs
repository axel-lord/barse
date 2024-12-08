//! Helper types implementing [Barse][crate::Barse] for common usages.

pub use self::{
    byte_array::ByteArray,
    padding::Padding,
    slice_sink::{SliceSink, SliceSinkFull},
    slice_source::{SliceSrc, SliceSrcEmpty},
    use_endian::UseEndian,
};

mod byte_array;

mod use_endian;

mod padding;

mod slice_source;

mod slice_sink;

#[cfg(feature = "zerocopy")]
pub mod zerocopy;

#[cfg(feature = "bytemuck")]
pub mod bytemuck;
