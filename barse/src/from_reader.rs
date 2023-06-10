//! Implementations of [`crate::FromByteReader`][FromByteReader].

pub use padding::Padding;
pub use remaining::Remaining;

pub(super) mod padding;

mod def;
mod integer;
mod option;
mod phantom_data;
mod remaining;
mod tuple;
mod u8_array;
mod u8_slice;
mod value;
mod vec;
mod with_fn;
