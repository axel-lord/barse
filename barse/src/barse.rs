//! [Barse] trait and implementations.

use crate::{ByteSink, ByteSource, Endian, Error};

/// Trait to serialize and deserialize from binary data.
pub trait Barse: Sized {
    /// Additional data needed to read.
    type ReadWith;

    /// Additional data needed to write.
    type WriteWith;

    /// Read an instnce from source with given endianess.
    ///
    /// # Errors
    /// If Soure or implementation errors.
    fn read<E, B>(from: &mut B, with: Self::ReadWith) -> Result<Self, Error<B::Err>>
    where
        E: Endian,
        B: ByteSource;

    /// Write an instance to a sink with given endianess.
    ///
    /// # Errors
    /// If Sink or implementation errors.
    fn write<E, B>(&self, to: &mut B, with: Self::WriteWith) -> Result<(), Error<B::Err>>
    where
        E: Endian,
        B: ByteSink;
}

integer_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl<T, ReadWith, WriteWith, const N: usize> Barse for [T; N]
where
    T: Barse<ReadWith = ReadWith, WriteWith = WriteWith>,
    ReadWith: Clone,
    WriteWith: Clone,
{
    type ReadWith = ReadWith;
    type WriteWith = WriteWith;

    fn read<E, B>(from: &mut B, with: Self::ReadWith) -> Result<Self, Error<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        let mut values = [const { None }; N];
        for value in values.iter_mut() {
            *value = Some(T::read::<E, B>(from, with.clone())?);
        }
        Ok(values.map(|value| value.expect("all values should be some")))
    }

    fn write<E, B>(&self, to: &mut B, with: Self::WriteWith) -> Result<(), Error<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        for value in self {
            T::write::<E, B>(value, to, with.clone())?;
        }
        Ok(())
    }
}

/// Implement Barse trait for integers.
macro_rules! integer_impl {
    ($($ty:ty),*) => {
        $(
        paste::paste! {
        impl Barse for $ty {
            type ReadWith = ();
            type WriteWith = ();
            #[inline(always)]
            fn read<E, B>(from: &mut B, _with: ()) -> Result<Self, Error<B::Err>>
            where
                E: Endian,
                B: ByteSource,
            {
                    Ok(E :: [< $ty _from_bytes >](from.read_array()?))
            }

            #[inline(always)]
            fn write<E, B>(&self, to: &mut B, _with: ()) -> Result<(), Error<B::Err>>
            where
                E: Endian,
                B: ByteSink
            {
                Ok(to.write_array(E :: [< $ty _to_bytes >](*self))?)
            }
        }
        }
        )*
    };
}
use integer_impl;
