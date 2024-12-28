//! [Little] impl.

use crate::Endian;

/// Little [Endian] implementor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Little;

impl crate::sealed::Sealed for Little {}
impl Endian for Little {
    fn write<T: crate::sealed::ToFromEndian>(t: T) -> T::Bytes {
        t.to_little()
    }

    fn read<T: crate::sealed::ToFromEndian>(b: T::Bytes) -> T {
        T::from_little(b)
    }
}
