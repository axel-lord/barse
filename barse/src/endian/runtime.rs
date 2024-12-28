//! [Runtime] impl.

use crate::endian::{Big, Little, Native};

/// Endian selected at runtime, does not implement [Endian][crate::Endian] trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Runtime {
    /// Use big endian.
    Big,

    /// Use little endian.
    Little,

    /// Use platform native endian.
    #[default]
    Native,
}

impl From<Big> for Runtime {
    fn from(_value: Big) -> Self {
        Self::Big
    }
}

impl From<Little> for Runtime {
    fn from(_value: Little) -> Self {
        Self::Little
    }
}

impl From<Native> for Runtime {
    fn from(_value: Native) -> Self {
        Self::Native
    }
}
