use std::{
    any::{type_name, Any},
    io::Cursor,
};

use crate::{error::Error, Endian, Result};

pub trait ByteRead<'input> {
    type AtByteRead: ByteRead<'input>;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]>;

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        Ok(self.read_ref(COUNT)?.try_into()?)
    }

    fn remaining(&mut self) -> Result<&'input [u8]>;

    fn endian(&self) -> Endian {
        Endian::Little
    }

    fn all(&self) -> Result<&'input [u8]>;

    fn at(&self, _location: usize) -> Result<Self::AtByteRead> {
        Err(Error::AtNotSupported(type_name::<Self>().into()))
    }

    fn flags<T>(&self) -> Result<&T>
    where
        T: Any,
    {
        Err(Error::flag_not_found::<T>())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NilReader;

impl<'input> ByteRead<'input> for NilReader {
    type AtByteRead = NilReader;

    fn read_ref(&mut self, _count: usize) -> Result<&'input [u8]> {
        panic!("NilReaders should never exist")
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        panic!("NilReaders should never exist")
    }

    fn all(&self) -> Result<&'input [u8]> {
        panic!("NilReaders should never exist")
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, B> ByteRead<'input> for &mut B
where
    B: ByteRead<'input>,
{
    type AtByteRead = B::AtByteRead;

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

    fn flags<T>(&self) -> Result<&T>
    where
        T: Any,
    {
        (**self).flags()
    }

    fn all(&self) -> Result<&'input [u8]> {
        (**self).all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        (**self).at(location)
    }
}

impl<'input> ByteRead<'input> for Cursor<&'input [u8]> {
    type AtByteRead = Self;

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

    fn all(&self) -> Result<&'input [u8]> {
        Ok(self.get_ref())
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        let mut cursor = Cursor::new(*self.get_ref());
        cursor.set_position(location.try_into()?);

        Ok(cursor)
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
