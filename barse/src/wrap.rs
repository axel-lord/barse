//! For some [`crate::FromByteReaderWith`][FromByteReaderWith] implementations an implementation that is generic over a trait
//! may be desired, this might lead to collisions, to remedy this newtype wrappers are used.

/// Wrapper to allow [`FromByteReaderWith::from_byte_reader_with`][from_byte_reader_with]
/// for vecs to take an iterator as input.
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Iter<I>(I);

impl<I> Iter<I> {
    /// Construct a iterator wrapper.
    pub fn new(into_iter: I) -> Self
    where
        I: IntoIterator,
    {
        Self(into_iter)
    }

    /// Convert self into the wrapped value.
    pub fn into_inner(self) -> I {
        self.0
    }
}
