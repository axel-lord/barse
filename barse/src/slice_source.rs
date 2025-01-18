//! [SliceSrc] implementation.

use ::core::{hash::Hash, marker::PhantomData, ptr::NonNull};

use crate::{error::SliceSrcEmpty, ByteSource};

/// [ByteSource] implementor wrapping a slice.
///
/// The head may be higher than the slice length but not higher than isize::MAX (functions using
/// the head may panic).
#[derive(Debug, Clone, Copy)]
pub struct SliceSrc<'src> {
    /// Pointer to slice.
    ptr: NonNull<u8>,

    /// Length remaining.
    len: usize,

    /// Indicate lifetime.
    _p: PhantomData<&'src [u8]>,
}

impl Default for SliceSrc<'_> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'src> SliceSrc<'src> {
    /// Create a new [SliceSrc] backed by given slice.
    #[inline]
    pub const fn new(slice: &'src [u8]) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(slice.as_ptr() as *mut u8) },
            len: slice.len(),
            _p: PhantomData,
        }
    }

    /// Get remaining bytes as a slice.
    const fn as_ref(&self) -> &[u8] {
        unsafe { ::core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Get how many more bytes may be read.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if no more bytes may be read.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Skip count bytes if possible.
    ///
    /// # Returns
    /// True if bytes were skipped and false otherwise.
    #[inline]
    #[must_use]
    const fn skip_bytes(&mut self, count: usize) -> bool {
        if let Some(len) = self.len.checked_sub(count) {
            self.ptr = unsafe { self.ptr.add(count) };
            self.len = len;
            true
        } else {
            false
        }
    }

    /// Move past current ptr by size if possible and return old ptr.
    ///
    /// # Safety
    /// If Some, the internal pointer has been moved by size bytes.
    #[inline]
    const fn next_ptr(&mut self, size: usize) -> Option<NonNull<u8>> {
        // Get length of replacement slice. If size is larger than current length fail.
        // Ensures size <= self.len.
        let Some(len) = self.len.checked_sub(size) else {
            return None;
        };

        // Get start of returned slice.
        let start = self.ptr;

        // Move past size. Safe as size <= len.
        self.ptr = unsafe { self.ptr.add(size) };
        self.len = len;

        Some(start)
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

impl PartialEq for SliceSrc<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl Eq for SliceSrc<'_> {}
impl Hash for SliceSrc<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}
impl Ord for SliceSrc<'_> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}
impl PartialOrd for SliceSrc<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
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
        Some(self.len)
    }
}
