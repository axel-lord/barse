//! Trait implementations for alloc types.

use ::core::convert::Infallible;

use crate::ByteSink;

extern crate alloc;

impl ByteSink for alloc::vec::Vec<u8> {
    type Err = Infallible;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

