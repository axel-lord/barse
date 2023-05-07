use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
};

use crate::{error::Error, ByteRead, Endian};

/// ByteRead wrapper that can give a set of flags.
pub struct FlagByteReader<'input, R, const SIZE: usize>(R, [(TypeId, &'input dyn Any); SIZE]);

impl<'input, R, const SIZE: usize> FlagByteReader<'input, R, SIZE> {
    pub fn new(reader: R, flags: [&'input dyn Any; SIZE]) -> Self
    where
        R: ByteRead<'input>,
    {
        Self(reader, flags.map(|flag| (flag.type_id(), flag)))
    }

    pub fn all_flags(&self) -> [&'input dyn Any; SIZE] {
        self.1.map(|flag| flag.1)
    }

    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<'input, R, const SIZE: usize> Deref for FlagByteReader<'input, R, SIZE> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'input, R, const SIZE: usize> DerefMut for FlagByteReader<'input, R, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'input, R, const SIZE: usize> AsRef<R> for FlagByteReader<'input, R, SIZE> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

#[deny(clippy::missing_trait_methods)]
impl<'input, R, const SIZE: usize> ByteRead<'input> for FlagByteReader<'input, R, SIZE>
where
    R: ByteRead<'input>,
{
    fn read_ref(&mut self, count: usize) -> Option<&'input [u8]> {
        self.0.read_ref(count)
    }

    fn remaining(&mut self) -> Option<&'input [u8]> {
        self.0.remaining()
    }

    fn read<const COUNT: usize>(&mut self) -> Option<[u8; COUNT]> {
        self.0.read::<COUNT>()
    }

    fn endian(&self) -> Endian {
        self.0.endian()
    }

    fn flags<T>(&self) -> Result<&'input T, Error>
    where
        T: Any,
    {
        let type_id = TypeId::of::<T>();
        for (id, val) in &self.1 {
            if type_id == *id {
                return val.downcast_ref::<T>().ok_or(Error::flag_not_found::<T>());
            }
        }

        // In case the flag was not found the wrapped reader is queried.
        self.0.flags::<T>()
    }
}
