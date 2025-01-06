//! Trait implementations for alloc types.

use ::core::convert::Infallible;

use alloc::{borrow::Cow, boxed::Box};

use crate::{Barse, ByteSink, ByteSource};

extern crate alloc;

impl ByteSink for alloc::vec::Vec<u8> {
    type Err = Infallible;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

impl<Src> ByteSource for Box<Src>
where
    Src: ByteSource,
{
    type Err = Src::Err;

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        (**self).read_slice(buf)
    }
}

impl<Sink> ByteSink for Box<Sink>
where
    Sink: ByteSink,
{
    type Err = Sink::Err;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        (**self).write_slice(buf)
    }
}

impl<T> Barse for Box<T>
where
    T: Barse,
{
    type ReadWith = T::ReadWith;

    type WriteWith = T::WriteWith;

    #[inline]
    fn read_with<E, B>(
        from: &mut B,
        with: Self::ReadWith,
    ) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: ByteSource,
    {
        T::read_with::<E, _>(from, with).map(Box::new)
    }

    #[inline]
    fn write_with<E, B>(
        &self,
        to: &mut B,
        with: Self::WriteWith,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: ByteSink,
    {
        T::write_with::<E, _>(self, to, with)
    }
}

impl<'a, T> Barse for Cow<'a, T>
where
    T: ToOwned + Barse + 'a,
    T::Owned: Barse<ReadWith = T::ReadWith, WriteWith = T::WriteWith>,
{
    type ReadWith = <T::Owned as Barse>::ReadWith;

    type WriteWith = <T::Owned as Barse>::WriteWith;

    #[inline]
    fn read_with<E, B>(
        from: &mut B,
        with: Self::ReadWith,
    ) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: ByteSource,
    {
        T::Owned::read_with::<E, _>(from, with).map(Cow::Owned)
    }

    #[inline]
    fn write_with<E, B>(
        &self,
        to: &mut B,
        with: Self::WriteWith,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: ByteSink,
    {
        T::write_with::<E, _>(self, to, with)
    }
}
