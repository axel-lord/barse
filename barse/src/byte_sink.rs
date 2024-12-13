//! [ByteSink] trait.

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
}
