//! [Barse] trait and implementations.

use crate::{ByteSink, ByteSource, Endian, Error};

/// Trait to serialize and deserialize from binary data.
pub trait Barse: Sized {
    /// Read an instnce from source with given endianess.
    ///
    /// # Errors
    /// If Soure or implementation errors.
    fn read<E, B>(from: &mut B) -> Result<Self, Error<B::Err>>
    where
        E: Endian,
        B: ByteSource;

    /// Write an instance to a sink with given endianess.
    ///
    /// # Errors
    /// If Sink or implementation errors.
    fn write<E, B>(&self, to: &mut B) -> Result<(), Error<B::Err>>
    where
        E: Endian,
        B: ByteSink;
}

integer_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

/// Implement Barse trait for integers.
macro_rules! integer_impl {
    ($($ty:ty),*) => {
        $(
        paste::paste! {
        impl Barse for $ty {

            fn read<E, B>(from: &mut B) -> Result<Self, Error<B::Err>>
            where
                E: Endian,
                B: ByteSource,
            {
                    Ok(E :: [< $ty _from_bytes >](from.read()?))
            }

            fn write<E, B>(&self, to: &mut B) -> Result<(), Error<B::Err>>
            where
                E: Endian,
                B: ByteSink
            {
                Ok(to.write(E :: [< $ty _to_bytes >](*self))?)
            }
        }
        }
        )*
    };
}
use integer_impl;
