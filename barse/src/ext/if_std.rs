//! Trait implementations and types used with std feature.

use ::std::io::{self, Read, Write};

use crate::{ByteSink, ByteSource};

/// Extension trait used to convert [Read] implementors to [ByteSource].
pub trait AsByteSource {
    /// Get a [ByteSource].
    fn as_byte_source(&mut self) -> impl '_ + ByteSource<Err = ::std::io::Error>;
}

/// Extension trait used to convert [Write] implementors to [ByteSink].
pub trait AsByteSink {
    /// Get a [ByteSink].
    fn as_byte_sink(&mut self) -> impl '_ + ByteSink<Err = ::std::io::Error>;
}

/// [Read] wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Reader<R>(R);

impl<R> ByteSource for Reader<R>
where
    R: Read,
{
    type Err = io::Error;

    #[inline]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        self.0.read_exact(buf)
    }
}

impl<T> AsByteSource for T
where
    T: Read,
{
    #[inline]
    fn as_byte_source(&mut self) -> impl '_ + ByteSource<Err = ::std::io::Error> {
        Reader(self)
    }
}

/// [Write] wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct Writer<W>(W);

impl<W> ByteSink for Writer<W>
where
    W: Write,
{
    type Err = io::Error;

    #[inline]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.0.write_all(buf)
    }
}

impl<T> AsByteSink for T
where
    T: Write,
{
    #[inline]
    fn as_byte_sink(&mut self) -> impl '_ + ByteSink<Err = ::std::io::Error> {
        Writer(self)
    }
}
