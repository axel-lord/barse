use crate::{Endian, Result};

/// Trait for types that read bytes.
pub trait ByteRead<'input> {
    /// Type returned by [`Self::at`], must implement [`ByteRead`] itself.
    type AtByteRead: ByteRead<'input>;

    /// Type returned by [`Self::by_ref`], must implement [`ByteRead`] itself.
    type ByRefByteRead<'s>: 's + ByteRead<'input>
    where
        Self: 's;

    /// A reference to a runtime specified amount of bytes.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]>;

    /// Array of bytes with it's size specified at compile time.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        Ok(self
            .read_ref(COUNT)?
            .try_into()
            .map_err(anyhow::Error::from)?)
    }

    /// A reference to all remaining data.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn remaining(&mut self) -> Result<&'input [u8]>;

    /// The endianess of the reader.
    fn endian(&self) -> Endian {
        Endian::Little
    }

    /// All data managed by reader as a slice.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn all(&self) -> Result<&'input [u8]>;

    /// Get a reference to self, often just a mut ref.
    fn by_ref(&mut self) -> Self::ByRefByteRead<'_>;

    /// A new reader starting at the given position of this reader.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn at(&self, location: usize) -> Result<Self::AtByteRead>;
}
