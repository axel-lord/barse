//! [Native] impl.

use crate::Endian;

/// Native [Endian] implementor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Native;

impl crate::sealed::Sealed for Native {}
impl Endian for Native {
    fn write<T: crate::sealed::ToFromEndian>(t: T) -> T::Bytes {
        t.to_native()
    }

    fn read<T: crate::sealed::ToFromEndian>(b: T::Bytes) -> T {
        T::from_native(b)
    }
}
