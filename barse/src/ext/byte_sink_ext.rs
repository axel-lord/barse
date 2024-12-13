//! Byte sink extension trait.

use crate::{
    endian::{Big, Little, Native},
    Barse, ByteSink, Endian, Error,
};

/// Extension to [ByteSink] adding convenient functions.
pub trait ByteSinkExt: ByteSink {
    /// Write a value implementing [Barse] using given endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn write<T: Barse<WriteWith = ()>, E: Endian>(
        &mut self,
        value: &T,
    ) -> Result<(), Error<Self::Err>>;

    /// Write a value implementing [Barse] using given endian and aditional value.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn write_with<T: Barse, E: Endian>(
        &mut self,
        value: &T,
        with: T::WriteWith,
    ) -> Result<(), Error<Self::Err>>;

    /// Write a value implementing [Barse] using little endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    fn write_le<T: Barse<WriteWith = ()>>(&mut self, value: &T) -> Result<(), Error<Self::Err>>;

    /// Write a value implementing [Barse] using big endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    fn write_be<T: Barse<WriteWith = ()>>(&mut self, value: &T) -> Result<(), Error<Self::Err>>;

    /// Write a value implementing [Barse] using native endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    fn write_ne<T: Barse<WriteWith = ()>>(&mut self, value: &T) -> Result<(), Error<Self::Err>>;
}

impl<S: ByteSink> ByteSinkExt for S {
    #[inline(always)]
    fn write<T: Barse<WriteWith = ()>, E: Endian>(
        &mut self,
        value: &T,
    ) -> Result<(), Error<Self::Err>> {
        T::write::<E, Self>(value, self, ())
    }

    #[inline(always)]
    fn write_le<T: Barse<WriteWith = ()>>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Self::write::<T, Little>(self, value)
    }

    #[inline(always)]
    fn write_be<T: Barse<WriteWith = ()>>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Self::write::<T, Big>(self, value)
    }

    #[inline(always)]
    fn write_ne<T: Barse<WriteWith = ()>>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Self::write::<T, Native>(self, value)
    }

    fn write_with<T: Barse, E: Endian>(
        &mut self,
        value: &T,
        with: T::WriteWith,
    ) -> Result<(), Error<Self::Err>> {
        T::write::<E, Self>(value, self, with)
    }
}
