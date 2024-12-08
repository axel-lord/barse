//! [SliceSrc] implementation.

use crate::ByteSource;

/// [ByteSource] implementor wrapping a slice.
///
/// The head may be higher than the slice length but not higher than isize::MAX (functions using
/// the head may panic).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SliceSrc<'src> {
    /// Slice that is read from.
    pub slice: &'src [u8],

    /// Where to read from slice.
    pub head: usize,
}

impl<'src> SliceSrc<'src> {
    /// Create a new [SliceSrc] backed by given slice.
    pub const fn new(slice: &'src [u8]) -> Self {
        Self { slice, head: 0 }
    }
}

/// Error returned by [ByteSource] implementation for [SliceSrc] when bytes cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, thiserror::Error)]
#[error("a value was too large to be read from remaining length at head of SliceSrc")]
pub struct SliceSrcEmpty;

impl ByteSource for SliceSrc<'_> {
    type Err = SliceSrcEmpty;

    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        debug_assert!(self.head <= isize::MAX as usize);

        let start = self.head;
        let end = self.head + buf.len();

        if end > self.slice.len() {
            return Err(SliceSrcEmpty);
        }

        buf.copy_from_slice(&self.slice[start..end]);
        self.head = end;

        Ok(())
    }
}

