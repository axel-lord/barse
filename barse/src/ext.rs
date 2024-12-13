//! Extension traits.

use crate::{
    endian::{Big, Little, Native},
    Barse, ByteSink, ByteSource, Endian, Error,
};

/// Extension to [ByteSource] adding som convenient functions.
pub trait ByteSourceExt: ByteSource {
    /// Read a value implementing [Barse] using given endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read<T: Barse<ReadWith = ()>, E: Endian>(&mut self) -> Result<T, Error<Self::Err>>;

    /// Read a value implementing [Barse] using little endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_le<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, Error<Self::Err>>;

    /// Read a value implementing [Barse] using big endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_be<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, Error<Self::Err>>;

    /// Read a value implementing [Barse] using native endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    fn read_ne<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, Error<Self::Err>>;
}

impl<S: ByteSource> ByteSourceExt for S {
    #[inline(always)]
    fn read<T: Barse<ReadWith = ()>, E: Endian>(&mut self) -> Result<T, Error<Self::Err>> {
        T::read::<E, Self>(self, ())
    }

    #[inline(always)]
    fn read_le<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, Error<Self::Err>> {
        Self::read::<T, Little>(self)
    }

    #[inline(always)]
    fn read_be<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, Error<Self::Err>> {
        Self::read::<T, Big>(self)
    }

    #[inline(always)]
    fn read_ne<T: Barse<ReadWith = ()>>(&mut self) -> Result<T, Error<Self::Err>> {
        Self::read::<T, Native>(self)
    }
}

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
}
