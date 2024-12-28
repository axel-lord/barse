//! [Barse] trait and implementations.

use ::core::marker::PhantomData;

use crate::{ByteSink, ByteSource, Endian, WrappedErr};

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
    fn read_with<E, B>(from: &mut B, with: Self::ReadWith) -> Result<Self, WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource;

    /// Write an instance to a sink with given endianess.
    ///
    /// # Errors
    /// If Sink or implementation errors.
    fn write_with<E, B>(&self, to: &mut B, with: Self::WriteWith) -> Result<(), WrappedErr<B::Err>>
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

    fn read_with<E, B>(from: &mut B, with: Self::ReadWith) -> Result<Self, WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        let mut values = [const { None }; N];
        for value in values.iter_mut() {
            *value = Some(T::read_with::<E, B>(from, with.clone())?);
        }
        Ok(values.map(|value| value.expect("all values should be some")))
    }

    fn write_with<E, B>(&self, to: &mut B, with: Self::WriteWith) -> Result<(), WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        for value in self {
            T::write_with::<E, B>(value, to, with.clone())?;
        }
        Ok(())
    }
}

impl Barse for () {
    type ReadWith = ();

    type WriteWith = ();

    fn read_with<E, B>(_from: &mut B, _with: Self::ReadWith) -> Result<Self, WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        Ok(())
    }

    fn write_with<E, B>(
        &self,
        _to: &mut B,
        _with: Self::WriteWith,
    ) -> Result<(), WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        Ok(())
    }
}

impl<T> Barse for PhantomData<T> {
    type ReadWith = ();

    type WriteWith = ();

    fn read_with<E, B>(_from: &mut B, _with: Self::ReadWith) -> Result<Self, WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSource,
    {
        Ok(PhantomData)
    }

    fn write_with<E, B>(
        &self,
        _to: &mut B,
        _with: Self::WriteWith,
    ) -> Result<(), WrappedErr<B::Err>>
    where
        E: Endian,
        B: ByteSink,
    {
        Ok(())
    }
}

/// Implement Barse trait for integers.
macro_rules! integer_impl {
    ($($ty:ty),*) => {
        $(
        impl crate::sealed::ToFromEndian for $ty {
            type Bytes = [u8; size_of::<$ty>()];

            #[inline]
            fn to_native(self) -> Self::Bytes { <$ty>::to_ne_bytes(self) }
            #[inline]
            fn to_big(self) -> Self::Bytes { <$ty>::to_be_bytes(self) }
            #[inline]
            fn to_little(self) -> Self::Bytes { <$ty>::to_le_bytes(self) }
            #[inline]
            fn from_native(bytes: Self::Bytes) -> Self { <$ty>::from_ne_bytes(bytes) }
            #[inline]
            fn from_big(bytes: Self::Bytes) -> Self { <$ty>::from_be_bytes(bytes) }
            #[inline]
            fn from_little(bytes: Self::Bytes) -> Self { <$ty>::from_le_bytes(bytes) }
        }
        impl Barse for $ty {
            type ReadWith = ();
            type WriteWith = ();
            #[inline]
            fn read_with<E, B>(from: &mut B, _with: ()) -> Result<Self, WrappedErr<B::Err>>
            where
                E: Endian,
                B: ByteSource,
            {
                Ok(E::read::<Self>(from.read_array()?))
            }

            #[inline]
            fn write_with<E, B>(&self, to: &mut B, _with: ()) -> Result<(), WrappedErr<B::Err>>
            where
                E: Endian,
                B: ByteSink
            {
                Ok(to.write_array(E::write::<Self>(*self))?)
            }
        }
        )*
    };
}
use integer_impl;
