//! [SliceSink] implementation.

use ::core::{hash::Hash, ptr::NonNull};

use crate::{error::SliceSinkFull, ByteSink};

/// [ByteSink] implementor wrapping a slice.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SliceSink<'src> {
    /// Wrapped slice.
    slice: &'src mut [u8],
}

unsafe impl Send for SliceSink<'_> {}
unsafe impl Sync for SliceSink<'_> {}

impl<'src> SliceSink<'src> {
    /// Create a new [SliceSink] backed by given slice.
    #[inline]
    pub const fn new(slice: &'src mut [u8]) -> Self {
        Self { slice }
    }

    /// Get remaining bytes as a slice.
    pub const fn as_bytes(&self) -> &[u8] {
        self.slice
    }

    /// Get how many more bytes may be written.
    pub const fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns `true` if no more bytes may be written.
    pub const fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }

    /// Move past current ptr by size if possible and return old ptr.
    ///
    /// # Safety
    /// If Some, the internal pointer has moved by size bytes.
    const fn next_ptr(&mut self, size: usize) -> Option<NonNull<u8>> {
        // Get length of replacement slice. If size is larger than current length fail.
        // Ensures size <= self.len.
        let Some(len) = self.slice.len().checked_sub(size) else {
            return None;
        };

        // Get start of returned slice.
        let start = self.slice.as_mut_ptr();

        // Since len exists and is smaller than old len by size we know slice is valid.
        self.slice = unsafe { ::core::slice::from_raw_parts_mut(start.add(size), len) };

        // Since start is gotten from a slice we know it is not null.
        // Slice has also already been replaced.
        Some(unsafe { NonNull::new_unchecked(start) })
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
        Some(self.len())
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
        assert_eq!(sink.remaining(), Some(0));
        assert_eq!(sink.len(), 0);
        assert!(sink.is_empty());
        assert_eq!(SliceSink::default(), SliceSink::new(&mut []));
    }

    #[test]
    fn write_slice() {
        const LEN: usize = 128;
        const MESSAGES: &[&[u8]] = &[
            b"Hello There!",
            b"Nice Weather!",
            b"abcdefg",
            b"Test String Cool",
        ];

        let mut buf = [0u8; 128];
        for msg in MESSAGES {
            let mut sink = SliceSink::new(&mut buf);
            sink.write_slice(msg).expect("write should succeed");

            assert_eq!(msg.len(), LEN - sink.len());
            assert_eq!(&buf[..msg.len()], *msg);
        }

        let mut sink = SliceSink::new(&mut buf);
        let mut acc = 0;
        for msg in MESSAGES {
            let len = sink.len();
            acc += msg.len();
            sink.write_slice(msg).expect("write should succeed");

            assert_eq!(len - msg.len(), sink.len());
        }
        assert_eq!(acc, LEN - sink.len());

        let mut start = 0;
        for msg in MESSAGES {
            let end = start + msg.len();
            assert_eq!(&buf[start..end], *msg);
            start = end;
        }
    }
}
