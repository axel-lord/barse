use std::{
    any,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{error::Error, ByteRead, Endian, Result};

/// Dynamic version of [`crate::ByteRead`][ByteRead] that only uses trait features compatible
/// with dynamic dispatch.
trait DynByteRead<'input> {
    /// Read given amount of bytes.
    ///
    /// # Errors
    /// If wrapped reader errors.
    fn dyn_read_ref(&mut self, count: usize) -> Result<&'input [u8]>;

    /// Return a reference to all bytes.
    ///
    /// # Errors
    /// If wrapped reader errors.
    fn dyn_all(&self) -> Result<&'input [u8]>;

    /// Return remaining bytes.
    ///
    /// # Errors
    /// If wrapped reader errors.
    fn dyn_remaining(&mut self) -> Result<&'input [u8]>;

    /// Return endian of wrapped reader.
    fn dyn_endian(&self) -> Endian;

    /// Return a [`DynByteRead`] box starting at specified position.
    ///
    /// # Errors
    /// If wrapped reader errors.
    fn dyn_at(
        &self,
        location: usize,
    ) -> Result<(Box<dyn DynByteRead<'input> + 'input>, &'static str)>;

    fn dyn_get_flag(&self, id: any::TypeId) -> Option<&dyn any::Any>;
}

/// Wrapper for any [`crate::ByteRead`][ByteRead] that does not use generics.
pub struct DynamicByteReader<'input>(ReaderEnum<'input>, &'static str);

enum ReaderEnum<'input> {
    Owned(Box<dyn DynByteRead<'input> + 'input>),
    Borrowed(&'input mut dyn DynByteRead<'input>),
}

impl<'input> Deref for ReaderEnum<'input> {
    type Target = dyn DynByteRead<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        match self {
            ReaderEnum::Owned(br) => br.as_ref(),
            ReaderEnum::Borrowed(br) => *br,
        }
    }
}

impl<'input> DerefMut for ReaderEnum<'input> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ReaderEnum::Owned(br) => br.as_mut(),
            ReaderEnum::Borrowed(br) => *br,
        }
    }
}

impl<'input> DynamicByteReader<'input> {
    /// Construct a new instance of self without any forwarding of debug info.
    pub fn from_reader<R>(reader: R) -> Self
    where
        R: ByteRead<'input> + 'input,
    {
        Self(
            ReaderEnum::Owned(Box::new(reader)),
            std::any::type_name::<R>(),
        )
    }

    /// Create a new instance borrowing a reader.
    pub fn borrow_reader<R>(reader: &'input mut R) -> Self
    where
        R: ByteRead<'input> + 'input,
    {
        Self(
            ReaderEnum::Borrowed(reader),
            std::any::type_name::<&mut R>(),
        )
    }
}

impl<'input> Debug for DynamicByteReader<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DynamicByteReader").field(&self.1).finish()
    }
}

impl<'input, R> From<Box<R>> for DynamicByteReader<'input>
where
    R: ByteRead<'input> + 'input,
{
    fn from(value: Box<R>) -> Self {
        Self(ReaderEnum::Owned(value), any::type_name::<R>())
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input> ByteRead<'input> for DynamicByteReader<'input> {
    type AtByteRead = Self;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        self.0.dyn_read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        self.0.dyn_remaining()
    }

    fn all(&self) -> Result<&'input [u8]> {
        self.0.dyn_all()
    }

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        Ok(self.read_ref(COUNT)?.try_into()?)
    }

    fn endian(&self) -> Endian {
        self.0.dyn_endian()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        let (read, name) = self.0.dyn_at(location)?;
        Ok(Self(ReaderEnum::Owned(read), name))
    }

    fn get_flag(&self, id: any::TypeId) -> Option<&dyn any::Any> {
        self.dyn_get_flag(id)
    }

    fn into_dynamic(self) -> DynamicByteReader<'input>
    where
        Self: Sized + 'input,
    {
        self
    }

    fn flag<T>(&self) -> Result<&T>
    where
        T: any::Any,
    {
        self.get_flag(any::TypeId::of::<T>())
            .and_then(<(dyn std::any::Any + 'static)>::downcast_ref)
            .ok_or_else(Error::flag_not_found::<T>)
    }
}

impl<'input, R> DynByteRead<'input> for R
where
    R: ByteRead<'input> + 'input,
{
    fn dyn_read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        self.read_ref(count)
    }

    fn dyn_all(&self) -> Result<&'input [u8]> {
        self.all()
    }

    fn dyn_remaining(&mut self) -> Result<&'input [u8]> {
        self.remaining()
    }

    fn dyn_endian(&self) -> Endian {
        self.endian()
    }

    fn dyn_at(
        &self,
        location: usize,
    ) -> Result<(Box<dyn DynByteRead<'input> + 'input>, &'static str)> {
        let name = any::type_name::<Self>();
        let reader = self.at(location)?;

        Ok((Box::new(reader), name))
    }

    fn dyn_get_flag(&self, id: any::TypeId) -> Option<&dyn any::Any> {
        self.get_flag(id)
    }
}
