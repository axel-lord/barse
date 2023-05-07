use std::{any::Any, io::Cursor};

use crate::{error::Error, Endian, Result};

pub trait ByteRead<'input> {
    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]>;

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        Ok(self.read_ref(COUNT)?.try_into()?)
    }

    fn remaining(&mut self) -> Result<&'input [u8]>;

    fn endian(&self) -> Endian {
        Endian::Little
    }

    fn flags<T>(&self) -> Result<&'input T>
    where
        T: Any,
    {
        Err(Error::flag_not_found::<T>())
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, B> ByteRead<'input> for &mut B
where
    B: ByteRead<'input>,
{
    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        (*self).read()
    }

    fn endian(&self) -> Endian {
        (**self).endian()
    }

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        (*self).read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        (*self).remaining()
    }

    fn flags<T>(&self) -> Result<&'input T>
    where
        T: Any,
    {
        (**self).flags()
    }
}

impl<'input> ByteRead<'input> for Cursor<&'input [u8]> {
    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        let start: usize = self.position().try_into()?;
        let end = start.checked_add(count).ok_or(Error::CheckedOperation)?;
        let range = start..end;

        // Make sure the slicing is possible
        self.get_ref()
            .as_ref()
            .get(range.clone())
            .ok_or(Error::SliceFailure)?;

        // Update position performed here to avoid mutable borrow after immutable borrow.
        self.set_position(end.try_into()?);

        self.get_ref().get(range).ok_or(Error::SliceFailure)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        let start: usize = self.position().try_into()?;
        let end = self.get_ref().as_ref().len();
        let range = start..end;

        // Make sure the slicing is possible
        self.get_ref()
            .as_ref()
            .get(range.clone())
            .ok_or(Error::SliceFailure)?;

        // Update position performed here to avoid mutable borrow after immutable borrow.
        self.set_position(end.try_into()?);

        self.get_ref().get(range).ok_or(Error::SliceFailure)
    }
}
