//! [ByteSink] trait.

use crate::{
    endian::{Big, Little, Native},
    error::Error,
    Barse, Endian,
};

/// Sink for writing of bytes.
pub trait ByteSink: Sized {
    /// Error reported by sink.
    type Err;

    /// Try to write buf to sink.
    ///
    /// # Errors
    /// If bytes cannot be written or sink otherwise fails.
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err>;

    /// Write an array of bytes.
    ///
    /// # Errors
    /// If bytes cannot be written or sink otherwise fails.
    #[inline(always)]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        self.write_slice(&bytes)
    }

    /// Write a value implementing [Barse] using given endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    #[inline(always)]
    fn write<T: Barse, E: Endian>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        T::write::<E, Self>(value, self)
    }

    /// Write a value implementing [Barse] using little endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    #[inline(always)]
    fn write_le<T: Barse>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Self::write::<T, Little>(self, value)
    }

    /// Write a value implementing [Barse] using big endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    #[inline(always)]
    fn write_be<T: Barse>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Self::write::<T, Big>(self, value)
    }

    /// Write a value implementing [Barse] using native endian.
    ///
    /// # Errors
    /// If source or implementation errors.
    #[inline(always)]
    fn write_ne<T: Barse>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Self::write::<T, Native>(self, value)
    }
}

impl<Sink> ByteSink for &mut Sink
where
    Sink: ByteSink,
{
    type Err = Sink::Err;

    #[inline(always)]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        Sink::write_slice(self, buf)
    }

    #[inline(always)]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        Sink::write_array(self, bytes)
    }

    #[inline(always)]
    fn write<T: Barse, E: Endian>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Sink::write::<T, E>(self, value)
    }

    #[inline(always)]
    fn write_le<T: Barse>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Sink::write_le(self, value)
    }

    #[inline(always)]
    fn write_be<T: Barse>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Sink::write_be(self, value)
    }

    #[inline(always)]
    fn write_ne<T: Barse>(&mut self, value: &T) -> Result<(), Error<Self::Err>> {
        Sink::write_ne(self, value)
    }
}
