//! Items requiring stdandard library.

use ::std::io::{Cursor, Read, Write};

use crate::{
    endian::Native,
    ext::{AsByteSink, AsByteSource},
    Barse, ByteSink, ByteSource,
};

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

#[cfg(feature = "ext")]
/// Write a value using native endian to a [Write] implementor.
///
/// # Errors
/// If [Barse] implementation errors.
/// Or if the bytes resulting from it cannot be written.
pub fn write_value<T, W>(value: &T, mut to: W) -> ::std::io::Result<()>
where
    T: Barse<WriteWith = ()>,
    W: ::std::io::Write,
{
    use crate::WrappedErr;

    T::write_with::<Native, _>(value, &mut to.as_byte_sink(), ()).map_err(WrappedErr::merge_into)
}

#[cfg(feature = "ext")]
/// Read a value using native endian from a [Read] implementor.
///
/// # Errors
/// If [Barse] implementation errors.
/// Or if the bytes needed cannot be read.
pub fn read_value<T, R>(mut from: R) -> ::std::io::Result<T>
where
    T: Barse<ReadWith = ()>,
    R: ::std::io::Read,
{
    use crate::WrappedErr;

    T::read_with::<Native, _>(&mut from.as_byte_source(), ()).map_err(WrappedErr::merge_into)
}
