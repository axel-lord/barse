//! Utilities for parsing using dynamic endian.

use crate::{
    endian::{Big, Little, Native},
    Barse,
};

/// Endian used at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum RuntimeEndian {
    /// Use big endian.
    Big,

    /// Use little endian.
    Little,

    /// Use platform native endian.
    #[default]
    Native,
}

impl From<Big> for RuntimeEndian {
    #[inline(always)]
    fn from(_value: Big) -> Self {
        Self::Big
    }
}

impl From<Little> for RuntimeEndian {
    #[inline(always)]
    fn from(_value: Little) -> Self {
        Self::Little
    }
}

impl From<Native> for RuntimeEndian {
    #[inline(always)]
    fn from(_value: Native) -> Self {
        Self::Native
    }
}

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
    type ReadWith = RuntimeEndian;

    type WriteWith = RuntimeEndian;

    #[inline(always)]
    fn read<_E, B>(from: &mut B, with: Self::ReadWith) -> Result<Self, crate::Error<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSource,
    {
        match with {
            RuntimeEndian::Big => T::read::<Big, B>(from, ()),
            RuntimeEndian::Little => T::read::<Little, B>(from, ()),
            RuntimeEndian::Native => T::read::<Native, B>(from, ()),
        }
        .map(Self::new)
    }

    #[inline(always)]
    fn write<_E, B>(&self, to: &mut B, with: Self::WriteWith) -> Result<(), crate::Error<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSink,
    {
        match with {
            RuntimeEndian::Big => T::write::<Big, B>(&self.0, to, ()),
            RuntimeEndian::Little => T::write::<Little, B>(&self.0, to, ()),
            RuntimeEndian::Native => T::write::<Native, B>(&self.0, to, ()),
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
    type ReadWith = (RuntimeEndian, T::ReadWith);

    type WriteWith = (RuntimeEndian, T::WriteWith);

    #[inline(always)]
    fn read<_E, B>(from: &mut B, (e, with): Self::ReadWith) -> Result<Self, crate::Error<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSource,
    {
        match e {
            RuntimeEndian::Big => T::read::<Big, B>(from, with),
            RuntimeEndian::Little => T::read::<Little, B>(from, with),
            RuntimeEndian::Native => T::read::<Native, B>(from, with),
        }
        .map(Self::new)
    }

    #[inline(always)]
    fn write<_E, B>(
        &self,
        to: &mut B,
        (e, with): Self::WriteWith,
    ) -> Result<(), crate::Error<B::Err>>
    where
        _E: crate::Endian,
        B: crate::ByteSink,
    {
        match e {
            RuntimeEndian::Big => T::write::<Big, B>(&self.0, to, with),
            RuntimeEndian::Little => T::write::<Little, B>(&self.0, to, with),
            RuntimeEndian::Native => T::write::<Native, B>(&self.0, to, with),
        }
    }
}
