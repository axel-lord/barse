//! Trait implementations for alloc types.

use ::core::convert::Infallible;

use crate::{ByteSink, ByteSource};

extern crate alloc;

impl ByteSink for alloc::vec::Vec<u8> {
    type Err = Infallible;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

impl<Src> ByteSource for alloc::boxed::Box<Src>
where
    Src: ByteSource,
{
    type Err = Src::Err;

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        (**self).read_slice(buf)
    }
}

impl<Sink> ByteSink for alloc::boxed::Box<Sink>
where
    Sink: ByteSink,
{
    type Err = Sink::Err;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        (**self).write_slice(buf)
    }
}
