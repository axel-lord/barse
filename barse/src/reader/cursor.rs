use crate::{ByteRead, Error, Result};

/// Byte reader for u8 slices, like [`std::io::Cursor`] but using usize for index instead of u64.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cursor<'data> {
    slice: &'data [u8],
    position: usize,
}

impl Cursor<'_> {
    /// Construct a new cursor.
    #[must_use]
    pub fn new(slice: &[u8]) -> Cursor<'_> {
        Cursor { slice, position: 0 }
    }

    /// Return the amount of bytes remaining in slice.
    ///
    /// # Errors
    /// If the position is somehow higher than the length of the slice.
    pub fn remaining_bytes(self) -> Result<usize> {
        self.slice.len().checked_sub(self.position).map_or_else(
            || {
                Err(anyhow::anyhow!(
                    "the position of a Cursor was greater \
                                than it's length something that should never happen, \
                                position {}, length {}",
                    self.position,
                    self.slice.len()
                )
                .into())
            },
            Ok,
        )
    }
}

impl<'data> From<&'data [u8]> for Cursor<'data> {
    fn from(value: &'data [u8]) -> Self {
        Cursor::new(value)
    }
}

impl<'input, 'data: 'input> ByteRead<'input> for Cursor<'data> {
    type AtByteRead = Self;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        let start = self.position;
        let end = start
            .checked_add(count)
            .ok_or(Error::ReadOverflow { start, count })?;
        let range = start..end;

        let slice = self.slice.get(range.clone());

        if let Some(slice) = slice {
            self.position = end;
            Ok(slice)
        } else {
            Err(Error::SliceFailure(range))
        }
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        self.read_ref(self.remaining_bytes()?)
    }

    fn all(&self) -> Result<&'input [u8]> {
        Ok(self.slice)
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        if location < self.slice.len() {
            Ok(Cursor {
                slice: self.slice,
                position: location,
            })
        } else {
            Err(Error::OutOfBoundsAt {
                requested: location,
                max: self.slice.len(),
            })
        }
    }
}
