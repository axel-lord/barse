//! Utilities for parsing using dynamic endian.

use crate::{
    endian::{Big, Little, Native, Runtime},
    Barse,
};

/// Wrap a type such that it's [Barse] implementation uses dynamic endianess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct WithEndian<T>(T);

impl<T> WithEndian<T>
where
    T: Barse<ReadWith = (), WriteWith = ()>,
{
    /// Construct a new [WithEndian] from value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwrap [WithEndian] to wrapped value.
    pub fn into_inner(self) -> T {
        let Self(value) = self;
        value
    }
}

impl<T: Barse<ReadWith = (), WriteWith = ()>> Barse for WithEndian<T> {
    type ReadWith = Runtime;

    type WriteWith = Runtime;

    #[inline]
    fn read_with<_E, B>(
        from: &mut B,
        with: Self::ReadWith,
    ) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSource,
    {
        match with {
            Runtime::Big => T::read_with::<Big, B>(from, ()),
            Runtime::Little => T::read_with::<Little, B>(from, ()),
            Runtime::Native => T::read_with::<Native, B>(from, ()),
        }
        .map(Self::new)
    }

    #[inline]
    fn write_with<_E, B>(
        &self,
        to: &mut B,
        with: Self::WriteWith,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSink,
    {
        match with {
            Runtime::Big => T::write_with::<Big, B>(&self.0, to, ()),
            Runtime::Little => T::write_with::<Little, B>(&self.0, to, ()),
            Runtime::Native => T::write_with::<Native, B>(&self.0, to, ()),
        }
    }
}

/// Wrap a type such that it's [Barse] implementation uses dynamic endianess, and passes any
/// aditional data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct WithEndianWith<T>(T);

impl<T> WithEndianWith<T>
where
    T: Barse,
{
    /// Construct a new [WithEndian] from value.
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwrap [WithEndian] to wrapped value.
    pub fn into_inner(self) -> T {
        let Self(value) = self;
        value
    }
}

impl<T: Barse> Barse for WithEndianWith<T> {
    type ReadWith = (Runtime, T::ReadWith);

    type WriteWith = (Runtime, T::WriteWith);

    #[inline]
    fn read_with<_E, B>(
        from: &mut B,
        (e, with): Self::ReadWith,
    ) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSource,
    {
        match e {
            Runtime::Big => T::read_with::<Big, B>(from, with),
            Runtime::Little => T::read_with::<Little, B>(from, with),
            Runtime::Native => T::read_with::<Native, B>(from, with),
        }
        .map(Self::new)
    }

    #[inline]
    fn write_with<_E, B>(
        &self,
        to: &mut B,
        (e, with): Self::WriteWith,
    ) -> Result<(), crate::WrappedErr<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSink,
    {
        match e {
            Runtime::Big => T::write_with::<Big, B>(&self.0, to, with),
            Runtime::Little => T::write_with::<Little, B>(&self.0, to, with),
            Runtime::Native => T::write_with::<Native, B>(&self.0, to, with),
        }
    }
}
