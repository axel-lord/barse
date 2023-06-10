use std::io::Cursor;

use crate::{ByteRead, Error, Result};

impl<'input> ByteRead<'input> for Cursor<&'input [u8]> {
    type AtByteRead = Self;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        let start: usize = self.position().try_into().map_err(anyhow::Error::from)?;
        let end = start
            .checked_add(count)
            .ok_or(Error::ReadOverflow { start, count })?;
        let range = start..end;

        // Make sure the slicing is possible
        self.get_ref()
            .as_ref()
            .get(range.clone())
            .ok_or(Error::SliceFailure(range.clone()))?;

        // Update position performed here to avoid mutable borrow after immutable borrow.
        self.set_position(end.try_into().map_err(anyhow::Error::from)?);

        self.get_ref()
            .get(range.clone())
            .ok_or(Error::SliceFailure(range))
    }

    fn all(&self) -> Result<&'input [u8]> {
        Ok(self.get_ref())
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        let mut cursor = Cursor::new(*self.get_ref());
        cursor.set_position(location.try_into().map_err(anyhow::Error::from)?);

        Ok(cursor)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        let start: usize = self.position().try_into().map_err(anyhow::Error::from)?;
        let end = self.get_ref().as_ref().len();
        let range = start..end;

        // Make sure the slicing is possible
        self.get_ref()
            .as_ref()
            .get(range.clone())
            .ok_or(Error::SliceFailure(range.clone()))?;

        // Update position performed here to avoid mutable borrow after immutable borrow.
        self.set_position(end.try_into().map_err(anyhow::Error::from)?);

        self.get_ref()
            .get(range.clone())
            .ok_or(Error::SliceFailure(range))
    }
}
