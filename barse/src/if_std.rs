//! Trait implementations and types used with std feature.

use ::std::io::{self, Read, Write};

use crate::{ByteSink, ByteSource};

/// Extension trait used to convert [Read] implementors to [ByteSource].
pub trait AsByteSource {
    /// Get a [ByteSource].
    fn as_byte_source(&mut self) -> impl '_ + ByteSource;
}

/// Extension trait used to convert [Write] implementors to [ByteSink].
pub trait AsByteSink {
    /// Get a [ByteSink].
    fn as_byte_sink(&mut self) -> impl '_ + ByteSink;
}

impl<T> AsByteSource for T
where
    T: Read,
{
    #[inline(always)]
    fn as_byte_source(&mut self) -> impl '_ + ByteSource {
        /// [Read] wrapper.
        #[derive(Debug)]
        struct Reader<R>(R);

        impl<R> ByteSource for Reader<R>
        where
            R: Read,
        {
            type Err = io::Error;

            #[inline(always)]
            fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
                self.0.read_exact(buf)
            }
        }

        Reader(self)
    }
}

impl<T> AsByteSink for T
where
    T: Write,
{
    #[inline(always)]
    fn as_byte_sink(&mut self) -> impl '_ + ByteSink {
        /// [Write] wrapper.
        #[derive(Debug)]
        struct Writer<W>(W);

        impl<W> ByteSink for Writer<W>
        where
            W: Write,
        {
            type Err = io::Error;

            #[inline(always)]
            fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
                self.0.write_all(buf)
            }
        }

        Writer(self)
    }
}
