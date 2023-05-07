pub use byte_reader::{ByteReader, Endian};
pub use from_byte_reader::FromByteReader;
pub use padding::Padding;

#[cfg(feature = "derive")]
pub use parse_derive::FromByteReader;

mod byte_reader;
mod from_byte_reader;
mod padding;

pub mod prelude {
    pub use super::{ByteReader, FromByteReader, Padding};
}
