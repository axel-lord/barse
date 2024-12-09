//! [ByteSource] trait.

use crate::{
    endian::{Big, Little, Native},
    error::Error,
    Barse, Endian,
};

/// Source of bytes for reading.
pub trait ByteSource: Sized {
    /// Error reported by source.
    type Err;

    /// Try to fill buf with bytes.
    ///
    /// # Errors
    /// If source cannot fill buffer, or otherwise fails.
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err>;

    /// Read an array of bytes.
    ///
    /// # Errors
    /// If N bytes cannot be read from source.
    #[inline(always)]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        let mut bytes = [0u8; N];
        self.read_slice(&mut bytes)?;
        Ok(bytes)
    }

    /// Read a single byte.
    ///
    /// # Errors
    /// If the byte cannot be read from source.
    #[inline(always)]
    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        let [byte] = self.read_array()?;
        Ok(byte)
    }

    /// Read a value implementing [Barse] using given endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    #[inline(always)]
    fn read<T: Barse, E: Endian>(&mut self) -> Result<T, Error<Self::Err>> {
        T::read::<E, Self>(self)
    }

    /// Read a value implementing [Barse] using little endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    #[inline(always)]
    fn read_le<T: Barse>(&mut self) -> Result<T, Error<Self::Err>> {
        Self::read::<T, Little>(self)
    }

    /// Read a value implementing [Barse] using big endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    #[inline(always)]
    fn read_be<T: Barse>(&mut self) -> Result<T, Error<Self::Err>> {
        Self::read::<T, Big>(self)
    }

    /// Read a value implementing [Barse] using native endian.
    ///
    /// # Errors
    /// If source or barse implementation errors.
    #[inline(always)]
    fn read_ne<T: Barse>(&mut self) -> Result<T, Error<Self::Err>> {
        Self::read::<T, Native>(self)
    }
}

impl<Src> ByteSource for &mut Src
where
    Src: ByteSource,
{
    type Err = Src::Err;

    #[inline(always)]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        Src::read_slice(self, buf)
    }

    #[inline(always)]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        Src::read_array(self)
    }

    #[inline(always)]
    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        Src::read_byte(self)
    }

    #[inline(always)]
    fn read<T: Barse, E: Endian>(&mut self) -> Result<T, Error<Self::Err>> {
        Src::read::<T, E>(self)
    }

    #[inline(always)]
    fn read_le<T: Barse>(&mut self) -> Result<T, Error<Self::Err>> {
        Src::read_le(self)
    }

    #[inline(always)]
    fn read_be<T: Barse>(&mut self) -> Result<T, Error<Self::Err>> {
        Src::read_be(self)
    }

    #[inline(always)]
    fn read_ne<T: Barse>(&mut self) -> Result<T, Error<Self::Err>> {
        Src::read_ne(self)
    }
}
