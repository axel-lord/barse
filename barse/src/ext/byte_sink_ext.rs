//! Byte sink extension trait.

use crate::{
    endian::{Big, Little, Native},
    Barse, ByteSink, Endian, WrappedErr,
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
    ) -> Result<(), WrappedErr<Self::Err>>;

    /// Write a value implementing [Barse] using given endian and aditional value.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn write_with<T: Barse, E: Endian>(
        &mut self,
        value: &T,
        with: T::WriteWith,
    ) -> Result<(), WrappedErr<Self::Err>>;

    /// Write a value implementing [Barse] using little endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    fn write_le<T: Barse<WriteWith = ()>>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>>;

    /// Write a value implementing [Barse] using big endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    fn write_be<T: Barse<WriteWith = ()>>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>>;

    /// Write a value implementing [Barse] using native endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    fn write_ne<T: Barse<WriteWith = ()>>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>>;
}

impl<S: ByteSink> ByteSinkExt for S {
    #[inline]
    fn write<T: Barse<WriteWith = ()>, E: Endian>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>> {
        T::write_with::<E, Self>(value, self, ())
    }

    #[inline]
    fn write_le<T: Barse<WriteWith = ()>>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>> {
        Self::write::<T, Little>(self, value)
    }

    #[inline]
    fn write_be<T: Barse<WriteWith = ()>>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>> {
        Self::write::<T, Big>(self, value)
    }

    #[inline]
    fn write_ne<T: Barse<WriteWith = ()>>(
        &mut self,
        value: &T,
    ) -> Result<(), WrappedErr<Self::Err>> {
        Self::write::<T, Native>(self, value)
    }

    fn write_with<T: Barse, E: Endian>(
        &mut self,
        value: &T,
        with: T::WriteWith,
    ) -> Result<(), WrappedErr<Self::Err>> {
        T::write_with::<E, Self>(value, self, with)
    }
}
