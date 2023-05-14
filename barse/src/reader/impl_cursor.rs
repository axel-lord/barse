use std::{
    any::{Any, TypeId},
    io::Cursor,
};

use crate::{ByteRead, Error, Result};

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

    fn get_flag(&self, _id: TypeId) -> Option<&dyn Any> {
        None
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
