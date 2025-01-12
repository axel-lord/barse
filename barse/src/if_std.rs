//! Items requiring stdandard library.

use ::std::io::{Cursor, Read, Write};

use crate::{ByteSink, ByteSource, WrappedErr};

pub use self::{read_source::ReadSource, write_sink::WriteSink};

mod read_source;

mod write_sink;

impl<A> ByteSource for Cursor<A>
where
    Cursor<A>: Read,
{
    type Err = ::std::io::Error;

    #[inline]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        self.read_exact(buf)
    }
}

impl<A> ByteSink for Cursor<A>
where
    Cursor<A>: Write,
{
    type Err = ::std::io::Error;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.write_all(buf)
    }
}

/// Write a value using native endian to a [Write] implementor.
///
/// # Errors
/// If [Barse][crate::Barse] implementation errors.
/// Or if the bytes resulting from it cannot be written.
pub fn write_value<T>(value: &T, to: impl Write) -> ::std::io::Result<()>
where
    T: crate::Barse<WriteWith = ()>,
{
    T::write_with::<crate::endian::Native, _>(value, &mut WriteSink::new(to), ())
        .map_err(WrappedErr::merge_into)
}

/// Read a value using native endian from a [Read] implementor.
///
/// # Errors
/// If [Barse][crate::Barse] implementation errors.
/// Or if the bytes needed cannot be read.
pub fn read_value<T>(from: impl Read) -> ::std::io::Result<T>
where
    T: crate::Barse<ReadWith = ()>,
{
    T::read_with::<crate::endian::Native, _>(&mut ReadSource::new(from), ())
        .map_err(WrappedErr::merge_into)
}
