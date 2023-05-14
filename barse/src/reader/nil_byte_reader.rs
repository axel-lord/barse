use std::any::{Any, TypeId};

use crate::{ByteRead, Result};

/// A reader that cannot be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NilByteReader {}

impl<'input> ByteRead<'input> for NilByteReader {
    type AtByteRead = NilByteReader;

    fn read_ref(&mut self, _count: usize) -> Result<&'input [u8]> {
        unreachable!("NilReaders should never exist")
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        unreachable!("NilReaders should never exist")
    }

    fn all(&self) -> Result<&'input [u8]> {
        unreachable!("NilReaders should never exist")
    }

    fn get_flag(&self, _id: TypeId) -> Option<&dyn Any> {
        unreachable!("NilReaders should never exist")
    }
}
