use std::any::{type_name, Any, TypeId};

use crate::{error::Error, reader::DynamicByteReader, Endian, Result};

/// Trait for types that read bytes.
pub trait ByteRead<'input> {
    /// Type returned by [`Self::at`], must implement [`ByteRead`] itself.
    type AtByteRead: ByteRead<'input>;

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
        Ok(self.read_ref(COUNT)?.try_into()?)
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

    /// A new reader starting at the given position of this reader.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn at(&self, _location: usize) -> Result<Self::AtByteRead> {
        Err(Error::AtNotSupported(type_name::<Self>().into()))
    }

    /// Get a value of the specified type from the reader, usefull to pass along say a header.
    ///
    /// # Errors
    /// If the implementing type needs to.
    fn flag<T>(&self) -> Result<&T>
    where
        T: Any,
    {
        self.get_flag(TypeId::of::<T>())
            .and_then(<(dyn Any + 'static)>::downcast_ref)
            .ok_or_else(Error::flag_not_found::<T>)
    }

    /// Get a flag as an any from the reader, read using a [`any::TypeId`].
    fn get_flag(&self, id: TypeId) -> Option<&dyn Any>;

    /// Convert self into a [`crate::DynamicByteReader`][DynamicByteReader] it still implements
    /// [`ByteRead`] but has the same type regardless of the [`ByteRead`] that was converted, however
    fn into_dynamic(self) -> DynamicByteReader<'input>
    where
        Self: Sized + 'input,
    {
        DynamicByteReader::from_reader(self)
    }
}
