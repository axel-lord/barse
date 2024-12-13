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
}
