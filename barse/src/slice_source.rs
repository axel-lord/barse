//! [SliceSrc] implementation.

use ::core::{hash::Hash, ptr::NonNull};

use crate::{error::SliceSrcEmpty, ByteSource};

/// [ByteSource] implementor wrapping a slice.
///
/// The head may be higher than the slice length but not higher than isize::MAX (functions using
/// the head may panic).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SliceSrc<'src> {
    /// Wrapped slice.
    slice: &'src [u8],
}

impl<'src> SliceSrc<'src> {
    /// Create a new [SliceSrc] backed by given slice.
    #[inline]
    pub const fn new(slice: &'src [u8]) -> Self {
        Self { slice }
    }

    /// Get remaining bytes as a slice.
    pub const fn as_bytes(&self) -> &[u8] {
        self.slice
    }

    /// Get how many more bytes may be read.
    pub const fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns `true` if no more bytes may be read.
    pub const fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }

    /// Move past current ptr by size if possible and return old ptr.
    ///
    /// # Safety
    /// If Some, the internal pointer has been moved by size bytes.
    #[inline]
    const fn next_ptr(&mut self, size: usize) -> Option<NonNull<u8>> {
        // Get length of replacement slice. If size is larger than current length fail.
        // Ensures size <= self.len.
        let Some(len) = self.slice.len().checked_sub(size) else {
            return None;
        };

        // Get start of returned slice.
        let start = self.slice.as_ptr();

        // Since len exists and is smaller than old len by size we know slice is valid.
        self.slice = unsafe { ::core::slice::from_raw_parts(start.add(size), len) };

        // Since start is gotten from a slice we know it is not null.
        // Slice has also already been replaced.
        Some(unsafe { NonNull::new_unchecked(start as *mut u8) })
    }

    /// Skip count bytes if possible.
    ///
    /// # Returns
    /// True if bytes were skipped and false otherwise.
    #[inline]
    #[must_use]
    const fn skip_bytes(&mut self, count: usize) -> bool {
        self.next_ptr(count).is_some()
    }

    /// Get next slice of specified size if possible.
    /// Head will be moved past it.
    ///
    /// # Safety
    /// If a slice is returned it is guaranteed to have a length of size.
    #[inline]
    pub const fn next_slice(&mut self, size: usize) -> Option<&'src [u8]> {
        if let Some(start) = self.next_ptr(size) {
            Some(unsafe { ::core::slice::from_raw_parts(start.as_ptr(), size) })
        } else {
            None
        }
    }

    /// Get next array ref of specified size.
    /// Head will be moved past it.
    #[inline]
    pub const fn next_array<const SIZE: usize>(&mut self) -> Option<&'src [u8; SIZE]> {
        if let Some(start) = self.next_ptr(SIZE) {
            Some(unsafe { start.cast().as_ref() })
        } else {
            None
        }
    }
}

impl ByteSource for SliceSrc<'_> {
    type Err = SliceSrcEmpty;

    #[inline]
    fn read_slice(&mut self, buf: &mut [u8]) -> Result<(), Self::Err> {
        buf.copy_from_slice(self.next_slice(buf.len()).ok_or(SliceSrcEmpty)?);
        Ok(())
    }

    #[inline]
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], Self::Err> {
        let arr = self.next_array().ok_or(SliceSrcEmpty)?;
        Ok(*arr)
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, Self::Err> {
        let [byte] = self.next_array().ok_or(SliceSrcEmpty)?;
        Ok(*byte)
    }

    #[inline]
    fn skip(&mut self, count: usize) -> Result<(), Self::Err> {
        self.skip_bytes(count).then_some(()).ok_or(SliceSrcEmpty)
    }

    #[inline]
    fn skip_n<const N: usize>(&mut self) -> Result<(), Self::Err> {
        self.skip_bytes(N).then_some(()).ok_or(SliceSrcEmpty)
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::missing_panics_doc)]
    use super::*;

    #[test]
    fn empty_source() {
        let mut source = SliceSrc::default();

        assert_eq!(source.read_slice(&mut [0u8; 16]), Err(SliceSrcEmpty));
        assert_eq!(source.read_array::<1>(), Err(SliceSrcEmpty));
        assert_eq!(source.skip(1), Err(SliceSrcEmpty));
        assert_eq!(source.read_byte(), Err(SliceSrcEmpty));
        assert_eq!(source.skip_n::<6>(), Err(SliceSrcEmpty));
        assert_eq!(source.remaining(), Some(0));
        assert_eq!(source.len(), 0);
        assert_eq!(source.read_slice(&mut [0u8; 0]), Ok(()));
        assert!(source.is_empty());
    }

    #[test]
    fn read_slice() {
        let buf = b"hello there! Nice weather! Cool!";
        let mut source = SliceSrc::new(buf);

        let mut buf_a = [0u8; 12];
        let mut buf_b = [0u8; 5];

        assert_eq!(source.read_slice(&mut buf_a), Ok(()));
        assert_eq!(&buf_a, b"hello there!");
        assert_eq!(source.read_byte(), Ok(b' '));
        assert_eq!(source.skip_n::<128>(), Err(SliceSrcEmpty));
        assert_eq!(source.read_array::<13>(), Ok(*b"Nice weather!"));
        assert_eq!(source.read_byte(), Ok(b' '));
        assert_eq!(source.read_slice(&mut buf_b), Ok(()));
        assert_eq!(&buf_b, b"Cool!");
        assert_eq!(source.remaining(), Some(0));
        assert_eq!(source.len(), 0);
        assert_eq!(source.skip(1), Err(SliceSrcEmpty));
        assert_eq!(source.skip(0), Ok(()));
        assert!(source.is_empty());
    }
}
