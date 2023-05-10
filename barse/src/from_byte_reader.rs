use std::marker::PhantomData;

use crate::{ByteRead, Endian, Padding, Result};

/// Trait for types that can be parsed from a [`ByteRead`].
pub trait FromByteReader<'input>: Sized {
    /// Read the Self from a [`ByteRead`].
    ///
    /// # Errors
    /// If the implementor needs to.
    fn from_byte_reader<R>(reader: R) -> Result<Self>
    where
        R: ByteRead<'input>;
}

impl<'input, const COUNT: usize> FromByteReader<'input> for [u8; COUNT] {
    fn from_byte_reader<R>(mut reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        reader.read::<COUNT>()
    }
}

impl<'input, const SIZE: usize> FromByteReader<'input> for Padding<SIZE> {
    fn from_byte_reader<R>(mut reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        reader.read::<SIZE>()?;
        Ok(Self)
    }
}

impl<'input, T> FromByteReader<'input> for PhantomData<T> {
    fn from_byte_reader<R>(_reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        Ok(PhantomData::default())
    }
}

impl<'input> FromByteReader<'input> for Vec<u8> {
    fn from_byte_reader<R>(mut reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        reader.remaining().map(Vec::from)
    }
}

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