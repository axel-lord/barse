//! Items requiring stdandard library.

use ::std::io::{Cursor, Read, Write};

use crate::{ByteSink, ByteSource};

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
