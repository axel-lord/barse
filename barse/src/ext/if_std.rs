//! Trait implementations and types used with std feature.

use ::std::io::{Read, Write};

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

impl<T> AsByteSource for T
where
    T: Read,
{
    #[inline]
    fn as_byte_source(&mut self) -> impl '_ + ByteSource<Err = ::std::io::Error> {
        crate::ReadSource::new(self)
    }
}

impl<T> AsByteSink for T
where
    T: Write,
{
    #[inline]
    fn as_byte_sink(&mut self) -> impl '_ + ByteSink<Err = ::std::io::Error> {
        crate::WriteSink::new(self)
    }
}
