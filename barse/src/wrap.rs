//! For some [`crate::FromByteReaderWith`][FromByteReaderWith] implementations an implementation that is generic over a trait
//! may be desired, this might lead to collisions, to remedy this newtype wrappers are used.

/// Wrapper to allow [`FromByteReaderWith::from_byte_reader_with`][from_byte_reader_with]
/// for vecs to take an iterator as input.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Iter<I: IntoIterator>(pub I);

/// Wrapper to allow [`FromByteReaderWith::from_byte_reader_with`][from_byte_reader_with]
/// for vecs to take a length as input.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Len(pub usize);

/// Wrapper to allow [`FromByteReaderWith::from_byte_reader_with`][from_byte_reader_with]
/// for byte slice types.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Size(pub usize);

/// Wrapper to allow a value to be specified as a callable allowing [`FromByteReaderWith`] to be
/// implemented for all types.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Fn<F>(pub F);

/// Wrapper to allow a value to be carried forward or set to a value in a with impl.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Value<T>(pub T);

/// Wrapper to allow a value to be default initialized.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug, Hash)]
pub struct Default;
