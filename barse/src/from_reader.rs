//! Implementations of [`crate::FromByteReader`][FromByteReader].

pub use flag_conditional::FlagConditional;
pub use from_reader_slice::ByteSlice;
pub use padding::Padding;
pub use remaining::Remaining;
pub use sized_vec::SizedVec;

pub(super) mod flag_conditional;
pub(super) mod from_reader_slice;
pub(super) mod padding;
pub(super) mod sized_vec;

mod integer;
mod option;
mod phantom_data;
mod remaining;
mod tuple;
mod u8_array;
mod vec;
mod with_fn;
