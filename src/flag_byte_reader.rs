use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

use crate::{error::Error, ByteRead, Endian, Result};

/// [`ByteRead`] wrapper that can give a set of flags.
#[derive(Debug, Clone, Copy)]
pub struct FlagByteReader<'flags, R, const SIZE: usize>(R, [(TypeId, &'flags dyn Any); SIZE]);

impl<'flags, R, const SIZE: usize> FlagByteReader<'flags, R, SIZE> {
    /// Wrap a [`ByteRead`] yo use the given flags and fall back on it's own if any.
    pub fn new(reader: R, flags: [&'flags dyn Any; SIZE]) -> Self
    where
        R: ByteRead<'flags>,
    {
        Self(reader, flags.map(|flag| (flag.type_id(), flag)))
    }

    /// All flags set on this [`FlagByteReader`].
    pub fn all_flags(&self) -> [&'flags dyn Any; SIZE] {
        self.1.map(|flag| flag.1)
    }

    /// Consume self and return the wrapped [`ByteRead`].
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<'flags, R, const SIZE: usize> Deref for FlagByteReader<'flags, R, SIZE> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'flags, R, const SIZE: usize> DerefMut for FlagByteReader<'flags, R, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'flags, R, const SIZE: usize> AsRef<R> for FlagByteReader<'flags, R, SIZE> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, 'flags, R, const SIZE: usize> ByteRead<'input> for FlagByteReader<'flags, R, SIZE>
where
    R: ByteRead<'input>,
    'flags: 'input,
{
    type AtByteRead = FlagByteReader<'input, R::AtByteRead, SIZE>;
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
        self.0.endian()
    }

    fn flags<T>(&self) -> Result<&T>
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();
        for (id, val) in &self.1 {
            if type_id == *id {
                return val
                    .downcast_ref::<T>()
                    .ok_or_else(Error::flag_not_found::<T>);
            }
        }

        // In case the flag was not found the wrapped reader is queried.
        self.0.flags::<T>()
    }

    fn all(&self) -> Result<&'input [u8]> {
        self.0.all()
    }

    fn at(&self, location: usize) -> Result<Self::AtByteRead> {
        Ok(FlagByteReader(self.0.at(location)?, self.1))
    }
}
