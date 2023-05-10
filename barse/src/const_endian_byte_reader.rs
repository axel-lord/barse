use std::ops::{Deref, DerefMut};

use crate::{ByteRead, Endian, Result};

/// [`ByteRead`] wrapper using the given endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstEndianByteReader<R, const LITTLE: bool>(R);

impl<R, const LITTLE: bool> ConstEndianByteReader<R, LITTLE> {
    /// Construct a new [`ConstEndianByteReader`] wrapping given reader.
    pub fn new<'input>(reader: R) -> Self
    where
        R: ByteRead<'input>,
    {
        Self(reader)
    }

    /// Consume self returning the wrapped value.
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R, const LITTLE: bool> Deref for ConstEndianByteReader<R, LITTLE> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R, const LITTLE: bool> DerefMut for ConstEndianByteReader<R, LITTLE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R, const LITTLE: bool> AsRef<R> for ConstEndianByteReader<R, LITTLE> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, R, const LITTLE: bool> ByteRead<'input> for ConstEndianByteReader<R, LITTLE>
where
    R: ByteRead<'input>,
{
    type AtByteRead = ConstEndianByteReader<R::AtByteRead, LITTLE>;

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
        if LITTLE {
            Endian::Little
        } else {
            Endian::Big
        }
    }

    fn flags<T>(&self) -> Result<&T>
    where
        T: std::any::Any,
    {
        self.0.flags()
    }

    fn all(&self) -> Result<&'input [u8]> {
        self.0.all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        Ok(ConstEndianByteReader(self.0.at(location)?))
    }
}
