//! [ByteSource] trait.

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
    #[inline]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        let mut bytes = [0u8; N];
        self.read_slice(&mut bytes)?;
        Ok(bytes)
    }

    /// Read a single byte.
    ///
    /// # Errors
    /// If the byte cannot be read from source.
    #[inline]
    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        let mut byte = 0u8;
        self.read_slice(::core::array::from_mut(&mut byte))?;
        Ok(byte)
    }

    /// Skip bytes, as if they have been read.
    ///
    /// # Errors
    /// If bytes cannot be skipped/read.
    #[inline]
    fn skip(&mut self, count: usize) -> Result<(), Self::Err> {
        for _ in 0..count {
            _ = self.read_byte()?;
        }
        Ok(())
    }

    /// Skip an amount of bytes known at compile time.
    ///
    /// # Errors
    /// If bytes cannot be skipped/read.
    #[inline]
    fn skip_n<const N: usize>(&mut self) -> Result<(), Self::Err> {
        self.skip(N)
    }

    /// Get remaining bytes that may be read, if known.
    ///
    /// It may be possible to read more or fewer bytes than returned, but it should still be
    /// treated as valid to error if too small.
    fn remaining(&self) -> Option<usize> {
        None
    }
}

impl<Src> ByteSource for &mut Src
where
    Src: ByteSource,
{
    type Err = Src::Err;

    #[inline]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        Src::read_slice(self, buf)
    }

    #[inline]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        Src::read_array(self)
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        Src::read_byte(self)
    }

    #[inline]
    fn skip(&mut self, count: usize) -> Result<(), Self::Err> {
        Src::skip(self, count)
    }

    #[inline]
    fn skip_n<const N: usize>(&mut self) -> Result<(), Self::Err> {
        Src::skip_n::<N>(self)
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Src::remaining(self)
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
impl<Src> ByteSource for alloc::boxed::Box<Src>
where
    Src: ByteSource,
{
    type Err = Src::Err;

    #[inline]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        Src::read_slice(self, buf)
    }

    #[inline]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        Src::read_array(self)
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        Src::read_byte(self)
    }

    #[inline]
    fn skip(&mut self, count: usize) -> Result<(), Self::Err> {
        Src::skip(self, count)
    }

    #[inline]
    fn skip_n<const N: usize>(&mut self) -> Result<(), Self::Err> {
        Src::skip_n::<N>(self)
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Src::remaining(self)
    }
}
