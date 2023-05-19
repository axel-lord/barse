use crate::{error::Error, ByteRead, Endian, FromByteReader, Result};

from_byte_integer_reader_impl! {
    u8: 1,
    i8: 1,
    u16: 2,
    i16: 2,
    u32: 4,
    i32: 4,
    u64: 8,
    i64: 8,
    u128: 16,
    i128: 16,
}

macro_rules! from_byte_integer_reader_impl {
    ($($ty:ty: $si:expr),* $(,)?) => {
        $(
        impl<'input> FromByteReader<'input> for $ty {
            type Err = Error;
            fn from_byte_reader<R>(mut reader: R) -> Result<Self>
            where
                R: ByteRead<'input>,
            {
                let bytes = reader.read::<$si>()?;
                Ok(match reader.endian() {
                    Endian::Little => Self::from_le_bytes(bytes),
                    Endian::Big => Self::from_be_bytes(bytes),
                })
            }
        }
        )*
    };
}
use from_byte_integer_reader_impl;
