//! [Sealed] trait.

/// Trait used to prevent implementations.
pub trait Sealed {}

/// Trait used to convert values to/from bytes.
pub trait ToFromEndian: Sized {
    /// Bytes used by trait.
    type Bytes;

    /// Convert to native.
    fn to_native(self) -> Self::Bytes;

    /// Convert to big.
    fn to_big(self) -> Self::Bytes;

    /// Convert to little.
    fn to_little(self) -> Self::Bytes;

    /// Convert from native.
    fn from_native(bytes: Self::Bytes) -> Self;

    /// Convert from big.
    fn from_big(bytes: Self::Bytes) -> Self;

    /// Convert from little.
    fn from_little(bytes: Self::Bytes) -> Self;
}
