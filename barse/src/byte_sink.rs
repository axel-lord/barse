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
    #[inline]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        self.write_slice(&bytes)
    }

    /// Write a single byte.
    ///
    /// # Errors
    /// If the byte cannot be written to sink.
    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Err> {
        self.write_slice(&[byte])
    }

    /// Get remaining bytes that may be written, if known.
    ///
    /// It may be possible to write more or fewer bytes than returned, but it should still be
    /// treated as valid to error if too small.
    #[inline]
    fn remaining(&self) -> Option<usize> {
        None
    }
}

impl<Sink> ByteSink for &mut Sink
where
    Sink: ByteSink,
{
    type Err = Sink::Err;

    #[inline]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        Sink::write_slice(self, buf)
    }

    #[inline]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        Sink::write_array(self, bytes)
    }

    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Err> {
        Sink::write_byte(self, byte)
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Sink::remaining(self)
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
#[cfg(feature = "alloc")]
impl<Sink> ByteSink for alloc::boxed::Box<Sink>
where
    Sink: ByteSink,
{
    type Err = Sink::Err;

    #[inline]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        Sink::write_slice(self, buf)
    }

    #[inline]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        Sink::write_array(self, bytes)
    }

    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Err> {
        Sink::write_byte(self, byte)
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Sink::remaining(self)
    }
}
