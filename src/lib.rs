pub use byte_read::{ByteRead, NilReader};
pub use endian::Endian;
pub use endian_byte_reader::EndianByteReader;
pub use flag_byte_reader::FlagByteReader;
pub use flag_conditional::{Condition, FlagConditional};
pub use from_byte_reader::FromByteReader;
pub use from_reader_slice::{ByteSizeQuery, FromReaderSlice};
pub use padding::Padding;
pub use sized_vec::{FromReaderVec, VecLenQuery};

pub type Result<T> = std::result::Result<T, error::Error>;

#[cfg(feature = "derive")]
pub use parse_derive::FromByteReader;

mod byte_read;
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
    pub use super::{ByteRead, ByteSizeQuery, Condition, FromByteReader, VecLenQuery};
}
