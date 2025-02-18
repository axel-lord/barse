//! Byte source extension trait.

use crate::{
    endian::{Big, Little, Native},
    Barse, ByteSource, Endian, WrappedErr,
};

/// Extension to [ByteSource] adding som convenient functions.
pub trait ByteSourceExt: ByteSource {
    /// Read a value implementing [Barse] using given endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read<T: Barse<ReadWith = ()>, E: Endian>(&mut self) -> Result<T, WrappedErr<Self::Err>>;

    /// Read a value implementing [Barse] using given endian and additional value.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_with<T: Barse, E: Endian>(
        &mut self,
        with: T::ReadWith,
    ) -> Result<T, WrappedErr<Self::Err>>;

    /// Read a value implementing [Barse] using little endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_le<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, WrappedErr<Self::Err>>;

    /// Read a value implementing [Barse] using big endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_be<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, WrappedErr<Self::Err>>;

    /// Read a value implementing [Barse] using native endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_ne<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, WrappedErr<Self::Err>>;
}

impl<S: ByteSource> ByteSourceExt for S {
    #[inline]
    fn read<T: Barse<ReadWith = ()>, E: Endian>(&mut self) -> Result<T, WrappedErr<Self::Err>> {
        T::read_with::<E, Self>(self, ())
    }

    #[inline]
    fn read_le<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, WrappedErr<Self::Err>> {
        Self::read::<T, Little>(self)
    }

    #[inline]
    fn read_be<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, WrappedErr<Self::Err>> {
        Self::read::<T, Big>(self)
    }

    #[inline]
    fn read_ne<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, WrappedErr<Self::Err>> {
        Self::read::<T, Native>(self)
    }

    #[inline]
    fn read_with<T: Barse, E: Endian>(
        &mut self,
        with: T::ReadWith,
    ) -> Result<T, WrappedErr<Self::Err>> {
        T::read_with::<E, Self>(self, with)
    }
}
