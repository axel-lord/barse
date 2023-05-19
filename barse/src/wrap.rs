//! For some [`crate::FromByteReaderWith`][FromByteReaderWith] implementations an implementation that is generic over a trait
//! may be desired, this might lead to collisions, to remedy this newtype wrappers are used.

/// Wrapper to allow [`FromByteReaderWith::from_byte_reader_with`][from_byte_reader_with]
/// for vecs to take an iterator as input.
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Iter<I: IntoIterator>(pub I);

/// Wrapper to allow [`FromByteReaderWith::from_byte_reader_with`][from_byte_reader_with]
/// for vecs to take a length as input.
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Length(pub usize);
