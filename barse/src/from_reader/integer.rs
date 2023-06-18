use paste::paste;

use crate::{endian::Endian, error::Error, ByteRead, FromByteReader, Result};

from_byte_integer_reader_impl! {
    u8,
    i8,
    u16,
    i16,
    u32,
    i32,
    u64,
    i64,
    u128,
    i128,
}

macro_rules! from_byte_integer_reader_impl {
    ($($ty:ty),* $(,)?) => {
        paste! {
        $(
        impl<'input> FromByteReader<'input> for $ty {
            type Err = Error;
            fn from_byte_reader<R, E>(mut reader: R) -> Result<Self>
            where
                R: ByteRead<'input>,
                E: Endian,
            {
                Ok(E:: [<parse_ $ty>](reader.read()?))
            }
        }
        )*
        }
    };
}
use from_byte_integer_reader_impl;
