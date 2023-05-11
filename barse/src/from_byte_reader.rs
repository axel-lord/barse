use std::{borrow::Cow, marker::PhantomData};

use crate::{error::Error, ByteRead, Endian, Padding, Result};

/// Trait for types that can be parsed from a [`ByteRead`].
pub trait FromByteReader<'input>: Sized {
    /// Error type return when parsing bytes fails.
    type Err;
    /// Read the Self from a [`ByteRead`].
    ///
    /// # Errors
    /// If the implementor needs to.
    fn from_byte_reader<R>(reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>;
}

impl<'input, const COUNT: usize> FromByteReader<'input> for [u8; COUNT] {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        reader.read::<COUNT>()
    }
}

impl<'input, const SIZE: usize> FromByteReader<'input> for Padding<SIZE> {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        reader.read::<SIZE>()?;
        Ok(Self)
    }
}

impl<'input, T> FromByteReader<'input> for PhantomData<T> {
    type Err = Error;
    fn from_byte_reader<R>(_reader: R) -> Result<Self, Self::Err>
    where
        R: ByteRead<'input>,
    {
        Ok(PhantomData::default())
    }
}

impl<'input> FromByteReader<'input> for Vec<u8> {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        reader.remaining().map(Vec::from)
    }
}

impl<'input: 'data, 'data> FromByteReader<'input> for Cow<'data, [u8]> {
    type Err = Error;
    fn from_byte_reader<R>(mut reader: R) -> Result<Self>
    where
        R: ByteRead<'input>,
    {
        reader.remaining().map(Cow::Borrowed)
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

from_byte_reader_tuple_impl_recursive! {
    A, B, C,
    D, E, F,
    G, H, I,
    J, K, L
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

macro_rules! from_byte_reader_tuple_impl {
    ($($templs:ident),+ $(,)?) => {
        impl<'input, SharedErr, $($templs),*> FromByteReader<'input> for ($($templs,)*)
        where
            $(
            $templs: FromByteReader<'input, Err = SharedErr>,
            )*
        {
            type Err = SharedErr;
            fn from_byte_reader<R>(mut reader: R) -> Result<Self, SharedErr>
            where
                R: ByteRead<'input>,
            {
                Ok((
                    $(
                    <$templs as FromByteReader>::from_byte_reader(&mut reader)?,
                    )*
                ))
            }
        }

    };
}
use from_byte_reader_tuple_impl;

macro_rules! from_byte_reader_tuple_impl_recursive {
    ($first: tt) => {
        from_byte_reader_tuple_impl! {$first}
    };
    ($first: tt, $($other: tt),+) => {
        from_byte_reader_tuple_impl! {$first, $($other),*}
        from_byte_reader_tuple_impl_recursive! {$($other),*}
    }
}
use from_byte_reader_tuple_impl_recursive;
