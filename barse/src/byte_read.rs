use std::{
    any::{self, type_name, Any},
    io::Cursor,
};

use crate::{error::Error, DynamicByteReader, Endian, Result};

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
        self.get_flag(any::TypeId::of::<T>())
            .and_then(<(dyn std::any::Any + 'static)>::downcast_ref)
            .ok_or_else(Error::flag_not_found::<T>)
    }

    /// Get a flag as an any from the reader, read using a [`any::TypeId`].
    fn get_flag(&self, id: any::TypeId) -> Option<&dyn any::Any>;

    /// Convert self into a [`crate::DynamicByteReader`][DynamicByteReader] it still implements
    /// [`ByteRead`] but has the same type regardless of the [`ByteRead`] that was converted, however
    fn into_dynamic(self) -> DynamicByteReader<'input>
    where
        Self: Sized + 'input,
    {
        DynamicByteReader::from_reader(self)
    }
}

/// A reader that cannot be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NilReader {}

impl<'input> ByteRead<'input> for NilReader {
    type AtByteRead = NilReader;

    fn read_ref(&mut self, _count: usize) -> Result<&'input [u8]> {
        unreachable!("NilReaders should never exist")
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        unreachable!("NilReaders should never exist")
    }

    fn all(&self) -> Result<&'input [u8]> {
        unreachable!("NilReaders should never exist")
    }

    fn get_flag(&self, _id: any::TypeId) -> Option<&dyn any::Any> {
        unreachable!("NilReaders should never exist")
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

    fn flag<T>(&self) -> Result<&T>
    where
        T: Any,
    {
        (**self).flag()
    }

    fn all(&self) -> Result<&'input [u8]> {
        (**self).all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        (**self).at(location)
    }

    fn into_dynamic(self) -> DynamicByteReader<'input>
    where
        Self: Sized + 'input,
    {
        DynamicByteReader::borrow_reader(self)
    }

    fn get_flag(&self, id: any::TypeId) -> Option<&dyn any::Any> {
        (**self).get_flag(id)
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

    fn get_flag(&self, _id: any::TypeId) -> Option<&dyn any::Any> {
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

#[deny(clippy::missing_trait_methods)]
impl<'input, R> ByteRead<'input> for Box<R>
where
    R: ByteRead<'input>,
{
    type AtByteRead = R::AtByteRead;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        (**self).read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        (**self).remaining()
    }

    fn all(&self) -> Result<&'input [u8]> {
        (**self).all()
    }

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        (**self).read()
    }

    fn endian(&self) -> Endian {
        (**self).endian()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        (**self).at(location)
    }

    fn flag<T>(&self) -> Result<&T>
    where
        T: Any,
    {
        (**self).flag()
    }

    fn get_flag(&self, id: any::TypeId) -> Option<&dyn any::Any> {
        (**self).get_flag(id)
    }

    fn into_dynamic(self) -> DynamicByteReader<'input>
    where
        Self: Sized + 'input,
    {
        self.into()
    }
}
