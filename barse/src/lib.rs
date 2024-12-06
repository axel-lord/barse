#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

mod error;

mod barse;

#[cfg(feature = "std")]
mod if_std;

mod sealed {
    //! [Sealed] trait.

    /// Trait used to prevent implementations.
    pub trait Sealed {}
}

pub use self::{barse::Barse, error::Error};

#[doc(inline)]
pub use self::endian::Endian;

#[cfg(feature = "std")]
pub use if_std::{AsByteSink, AsByteSource};

pub mod endian;

pub mod util;

/// Source of bytes for reading.
pub trait ByteSource {
    /// Error reported by source.
    type Err;

    /// Try to fill buf with bytes.
    ///
    /// # Errors
    /// If source cannot fill buffer, or otherwise fails.
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err>;

    /// Read an array of bytes.
    ///
    /// # Errors
    /// If N bytes cannot be read from source.
    #[inline(always)]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        let mut bytes = [0u8; N];
        self.read_slice(&mut bytes)?;
        Ok(bytes)
    }
}

/// Sink for writing of bytes.
pub trait ByteSink {
    /// Error reported by sink.
    type Err;

    /// Try to write buf to sink.
    ///
    /// # Errors
    /// If bytes cannot be written or sink otherwise fails.
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err>;

    /// Write an array of bytes.
    ///
    /// # Errors
    /// If bytes cannot be written or sink otherwise fails.
    #[inline(always)]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        self.write_slice(&bytes)
    }
}
