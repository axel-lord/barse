//! [UseEndian] implementation.

use ::core::marker::PhantomData;

use crate::{Barse, Endian};

/// Always read/write wrapped value with given endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UseEndian<T, E>(T, PhantomData<fn() -> E>);

impl<T, E> UseEndian<T, E>
where
    E: Endian,
    T: Barse,
{
    /// Construct a new [UseEndian] from value.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self(value, PhantomData)
    }

    /// Unwrap [UseEndian] to wrapped value.
    #[inline(always)]
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
    #[inline(always)]
    fn read<_E, B>(from: &mut B) -> Result<Self, crate::Error<B::Err>>
    where
        _E: Endian,
        B: crate::ByteSource,
    {
        T::read::<E, B>(from).map(Self::new)
    }

    #[inline(always)]
    fn write<_E, B>(&self, to: &mut B) -> Result<(), crate::Error<B::Err>>
    where
        _E: Endian,
        B: crate::ByteSink,
    {
        T::write::<E, B>(&self.0, to)
    }
}
