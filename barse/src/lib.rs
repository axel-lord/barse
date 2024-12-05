#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod sealed {
    //! [Sealed] trait.

    /// Trait used to prevent implementations.
    pub trait Sealed {}
}

pub use endian::Endian;

pub mod endian;

#[cfg(feature = "std")]
mod if_std {
    //! Trait implementations and types used with std feature.
}

/// Source of bytes for reading.
pub trait ByteSource {}

/// Sink for writing of bytes.
pub trait ByteSink {}

/// Trait to serialize and deserialize from binary data.
pub trait Barse: Sized {
    type Err;

    /// Read an instnce from source with given endianess.
    ///
    /// # Errors
    /// If Soure or implementation errors.
    fn read<E: Endian>(from: impl ByteSource) -> Result<Self, Self::Err>;

    /// Write an instance to a sink with given endianess.
    ///
    /// # Errors
    /// If Sink or implementation errors.
    fn write<E: Endian>(&self, to: impl ByteSink) -> Result<(), Self::Err>;
}
