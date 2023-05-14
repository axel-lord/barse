use std::any::{Any, TypeId};

use crate::{ByteRead, Endian, Result};

use super::DynamicByteReader;

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

    fn get_flag(&self, id: TypeId) -> Option<&dyn Any> {
        (**self).get_flag(id)
    }
}
