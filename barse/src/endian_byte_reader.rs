use std::ops::{Deref, DerefMut};

use crate::{ByteRead, DynamicByteReader, Endian, Result};

/// [`ByteRead`] wrapper using given endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndianByteReader<R>(R, Endian);

impl<R> EndianByteReader<R> {
    /// Construct a new [`EndianByteReader`] wrapping given reader and using passed endian.
    pub fn new<'input>(reader: R, endian: Endian) -> Self
    where
        R: ByteRead<'input>,
    {
        Self(reader, endian)
    }

    /// Set the endian in use by this reader.
    pub fn set_endian(&mut self, endian: Endian) {
        self.1 = endian;
    }

    /// Consume self returning the wrapped value.
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R> Deref for EndianByteReader<R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R> DerefMut for EndianByteReader<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R> AsRef<R> for EndianByteReader<R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, R> ByteRead<'input> for EndianByteReader<R>
where
    R: ByteRead<'input>,
{
    type AtByteRead = EndianByteReader<R::AtByteRead>;

    fn read_ref(&mut self, count: usize) -> Result<&'input [u8]> {
        self.0.read_ref(count)
    }

    fn remaining(&mut self) -> Result<&'input [u8]> {
        self.0.remaining()
    }

    fn read<const COUNT: usize>(&mut self) -> Result<[u8; COUNT]> {
        self.0.read::<COUNT>()
    }

    fn endian(&self) -> Endian {
        self.1
    }

    fn flag<T>(&self) -> Result<&T>
    where
        T: std::any::Any,
    {
        self.0.flag()
    }

    fn all(&self) -> Result<&'input [u8]> {
        self.0.all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        Ok(EndianByteReader(self.0.at(location)?, self.1))
    }

    fn get_flag(&self, id: std::any::TypeId) -> Option<&dyn std::any::Any> {
        self.0.get_flag(id)
    }

    fn into_dynamic(self) -> DynamicByteReader<'input>
    where
        Self: Sized + 'input,
    {
        DynamicByteReader::from_reader(self)
    }
}
