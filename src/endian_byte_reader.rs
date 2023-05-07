use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{ByteRead, Endian, Result};

/// ByteRead wrapper using given endian.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndianByteReader<'input, R>(R, Endian, PhantomData<&'input ()>);

impl<'input, R> EndianByteReader<'input, R> {
    pub fn new(reader: R, endian: Endian) -> Self
    where
        R: ByteRead<'input>,
    {
        Self(reader, endian, PhantomData::default())
    }

    pub fn set_endian(&mut self, endian: Endian) {
        self.1 = endian;
    }

    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<'input, R> Deref for EndianByteReader<'input, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'input, R> DerefMut for EndianByteReader<'input, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'input, R> AsRef<R> for EndianByteReader<'input, R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, R> ByteRead<'input> for EndianByteReader<'input, R>
where
    R: ByteRead<'input>,
{
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

    fn flags<T>(&self) -> Result<&'input T>
    where
        T: std::any::Any,
    {
        self.0.flags()
    }
}
