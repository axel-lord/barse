//! [SliceSink] implementation.

use ::core::fmt::Display;

use crate::ByteSink;

/// [ByteSink] implementor wrapping a slice.
///
/// The head may be higher than the slice length but not higher than isize::MAX (functions using
/// the head may panic).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SliceSink<'src> {
    /// Slice that is read from.
    pub slice: &'src mut [u8],

    /// Where to read from slice.
    pub head: usize,
}

impl<'src> SliceSink<'src> {
    /// Create a new [SliceSink] backed by given slice.
    pub const fn new(slice: &'src mut [u8]) -> Self {
        Self { slice, head: 0 }
    }
}

/// Error returned by [ByteSink] implementation for [SliceSink] when no more bytes can be written
/// to sink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceSinkFull;

impl Display for SliceSinkFull {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("a value was too large to be written to remaining length at head of SliceSink")
    }
}

impl ::core::error::Error for SliceSinkFull {}

impl From<SliceSinkFull> for crate::Error {
    fn from(_value: SliceSinkFull) -> Self {
        static ERR: SliceSinkFull = SliceSinkFull;
        crate::Error::Dyn(&ERR)
    }
}

impl ByteSink for SliceSink<'_> {
    type Err = SliceSinkFull;

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        debug_assert!(self.head <= isize::MAX as usize);

        let start = self.head;
        let end = self.head + buf.len();

        if end > self.slice.len() {
            return Err(SliceSinkFull);
        }

        self.slice[start..end].copy_from_slice(buf);
        self.head = end;

        Ok(())
    }
}
