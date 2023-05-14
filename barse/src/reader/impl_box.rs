use std::any::{self, Any};

use crate::{reader::DynamicByteReader, ByteRead, Endian, Result};

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
