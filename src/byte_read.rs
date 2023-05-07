use std::{any::Any, io::Cursor};

use crate::Endian;

pub trait ByteRead<'a> {
    fn read_ref(&mut self, count: usize) -> Option<&'a [u8]>;

    fn read<const COUNT: usize>(&mut self) -> Option<[u8; COUNT]> {
        self.read_ref(COUNT)?.try_into().ok()
    }

    fn remaining(&mut self) -> Option<&'a [u8]>;

    fn endian(&self) -> Endian {
        Endian::Little
    }

    fn flags<T>(&self) -> Option<&'a T>
    where
        T: Any,
    {
        None
    }
}

impl<'a, B> ByteRead<'a> for &mut B
where
    B: ByteRead<'a>,
{
    fn read<const COUNT: usize>(&mut self) -> Option<[u8; COUNT]> {
        (*self).read()
    }

    fn endian(&self) -> Endian {
        (**self).endian()
    }

    fn read_ref(&mut self, count: usize) -> Option<&'a [u8]> {
        (*self).read_ref(count)
    }

    fn remaining(&mut self) -> Option<&'a [u8]> {
        (*self).remaining()
    }
}

impl<'a> ByteRead<'a> for Cursor<&'a [u8]> {
    fn read_ref(&mut self, count: usize) -> Option<&'a [u8]> {
        let start: usize = self.position().try_into().ok()?;
        let end = start.checked_add(count)?;
        let range = start..end;

        // Make sure the slicing is possible
        self.get_ref().as_ref().get(range.clone())?;

        // Update position performed here to avoid mutable borrow after immutable borrow.
        self.set_position(end.try_into().ok()?);

        self.get_ref().get(range)
    }

    fn remaining(&mut self) -> Option<&'a [u8]> {
        let start: usize = self.position().try_into().ok()?;
        let end = self.get_ref().as_ref().len();
        let range = start..end;

        // Make sure the slicing is possible
        self.get_ref().as_ref().get(range.clone())?;

        // Update position performed here to avoid mutable borrow after immutable borrow.
        self.set_position(end.try_into().ok()?);

        self.get_ref().get(range)
    }
}
