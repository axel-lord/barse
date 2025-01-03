//! [UseEndian] implementation.

use ::core::marker::PhantomData;

use crate::{Barse, Endian};

/// Always read/write wrapped value with given endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct UseEndian<T, E>(T, PhantomData<fn() -> E>);

impl<T, E> UseEndian<T, E>
where
    E: Endian,
    T: Barse,
{
    /// Construct a new [UseEndian] from value.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self(value, PhantomData)
    }

    /// Unwrap [UseEndian] to wrapped value.
    #[inline]
    pub fn into_inner(self) -> T {
        let Self(value, _) = self;
        value
    }
}

impl<T, E> Barse for UseEndian<T, E>
where
    T: Barse,
    E: Endian,
{
    type ReadWith = T::ReadWith;
    type WriteWith = T::WriteWith;

    #[inline]
    fn read_with<_E, B>(
        from: &mut B,
        with: Self::ReadWith,
    ) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        _E: Endian,
        B: crate::ByteSource,
    {
        T::read_with::<E, B>(from, with).map(Self::new)
    }

    #[inline]
    fn write_with<_E, B>(
        &self,
        to: &mut B,
        with: Self::WriteWith,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        _E: Endian,
        B: crate::ByteSink,
    {
        T::write_with::<E, B>(&self.0, to, with)
    }
}
