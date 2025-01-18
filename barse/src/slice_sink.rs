//! [SliceSink] implementation.

use ::core::{hash::Hash, marker::PhantomData, ptr::NonNull};

use crate::{error::SliceSinkFull, ByteSink};

/// [ByteSink] implementor wrapping a slice.
#[derive(Debug)]
pub struct SliceSink<'src> {
    /// Pointer to slice.
    ptr: NonNull<u8>,

    /// Length remaining.
    len: usize,

    /// Lifetime
    _p: PhantomData<&'src mut [u8]>,
}

impl Default for SliceSink<'_> {
    fn default() -> Self {
        Self::new(&mut [])
    }
}

unsafe impl Send for SliceSink<'_> {}
unsafe impl Sync for SliceSink<'_> {}

impl<'src> SliceSink<'src> {
    /// Create a new [SliceSink] backed by given slice.
    #[inline]
    pub const fn new(slice: &'src mut [u8]) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(slice.as_mut_ptr()) },
            len: slice.len(),
            _p: PhantomData,
        }
    }

    /// Get remaining bytes as a slice.
    const fn as_ref(&self) -> &[u8] {
        unsafe { ::core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Get how many more bytes may be written.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if no more bytes may be written.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Move past current ptr by size if possible and return old ptr.
    ///
    /// # Safety
    /// If Some, the internal pointer has moved by size bytes.
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
    pub const fn next_slice(&mut self, size: usize) -> Option<&'src mut [u8]> {
        if let Some(start) = self.next_ptr(size) {
            Some(unsafe { ::core::slice::from_raw_parts_mut(start.as_ptr(), size) })
        } else {
            None
        }
    }

    /// Get next array ref of specified size.
    /// Head will be moved past it.
    #[inline]
    pub const fn next_array_mut<const SIZE: usize>(&mut self) -> Option<&'src mut [u8; SIZE]> {
        if let Some(start) = self.next_ptr(SIZE) {
            Some(unsafe { start.cast().as_mut() })
        } else {
            None
        }
    }
}

impl PartialEq for SliceSink<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl Eq for SliceSink<'_> {}
impl Hash for SliceSink<'_> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}
impl Ord for SliceSink<'_> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}
impl PartialOrd for SliceSink<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ByteSink for SliceSink<'_> {
    type Err = SliceSinkFull;

    #[inline]
    fn write_slice(&mut self, buf: &[u8]) -> Result<(), Self::Err> {
        self.next_slice(buf.len())
            .ok_or(SliceSinkFull)?
            .copy_from_slice(buf);

        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, byte: u8) -> Result<(), Self::Err> {
        let [to] = self.next_array_mut().ok_or(SliceSinkFull)?;
        *to = byte;
        Ok(())
    }

    #[inline]
    fn write_array<const N: usize>(&mut self, bytes: [u8; N]) -> Result<(), Self::Err> {
        *self.next_array_mut().ok_or(SliceSinkFull)? = bytes;
        Ok(())
    }

    #[inline]
    fn remaining(&self) -> Option<usize> {
        Some(self.len)
    }
}

#[cfg(test)]
mod tests {
    #![expect(clippy::missing_panics_doc)]
    use super::*;

    #[test]
    fn empty_sink() {
        let mut sink = SliceSink::default();

        assert_eq!(sink.write_slice(b"Hello!"), Err(SliceSinkFull));
        assert_eq!(sink.write_array(*b"Hi..."), Err(SliceSinkFull));
        assert_eq!(sink.write_slice(b""), Ok(()));
        assert_eq!(sink.write_array(*b""), Ok(()));
        assert_eq!(sink.write_byte(b'T'), Err(SliceSinkFull));
        assert_eq!(sink.len(), 0);
        assert!(sink.is_empty());
        assert_eq!(SliceSink::default(), SliceSink::new(&mut []));
    }
}
