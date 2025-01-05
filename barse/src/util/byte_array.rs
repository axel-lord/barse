//! [ByteArray] implementation.

use ::core::ops::{Deref, DerefMut};

use crate::Barse;

/// Byte array wrapper with specialized barse read/write impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteArray<const N: usize>([u8; N]);

impl<const N: usize> ByteArray<N> {
    /// Construct a new [ByteArray] from passed bytes.
    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Unwrap [ByteArray] to wrapped bytes.
    #[inline]
    pub const fn into_inner(self) -> [u8; N] {
        self.0
    }
}

impl<const N: usize> Default for ByteArray<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> Barse for ByteArray<N> {
    type ReadWith = ();
    type WriteWith = ();

    #[inline]
    fn read_with<E, B>(from: &mut B, _with: ()) -> Result<Self, crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSource,
    {
        Ok(ByteArray(from.read_array()?))
    }

    #[inline]
    fn write_with<E, B>(&self, to: &mut B, _with: ()) -> Result<(), crate::WrappedErr<B::Err>>
    where
        E: crate::Endian,
        B: crate::ByteSink,
    {
        Ok(to.write_array(self.0)?)
    }
}

impl<const N: usize> Deref for ByteArray<N> {
    type Target = [u8; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for ByteArray<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> AsRef<[u8; N]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8; N] {
        self
    }
}

impl<const N: usize> AsRef<[u8]> for ByteArray<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> AsMut<[u8; N]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        self
    }
}

impl<const N: usize> AsMut<[u8]> for ByteArray<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<const N: usize> From<[u8; N]> for ByteArray<N> {
    fn from(value: [u8; N]) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub(crate) mod boxed_byte_array {
    //! [ByteBox] impl.

    extern crate alloc;

    use ::core::ops::{Deref, DerefMut};

    use alloc::boxed::Box;

    use crate::Barse;

    /// A boxed byte array avoiding stack allocation.
    /// The given constant
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ByteBox<const N: usize>(Box<[u8; N]>);

    impl<const N: usize> ByteBox<N> {
        /// Construct a new [ByteBox] from passed bytes.
        #[inline]
        pub const fn new(bytes: Box<[u8; N]>) -> Self {
            Self(bytes)
        }

        /// Unwrap [ByteBox] to wrapped box.
        #[inline]
        pub fn into_inner(self) -> Box<[u8; N]> {
            self.0
        }
    }

    impl<const N: usize> Default for ByteBox<N> {
        fn default() -> Self {
            let mut data = Box::<[u8]>::new_uninit_slice(N);

            for i in &mut data {
                i.write(0u8);
            }

            unsafe { Self(data.assume_init().try_into().unwrap_unchecked()) }
        }
    }

    impl<const N: usize> Deref for ByteBox<N> {
        type Target = [u8; N];

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<const N: usize> DerefMut for ByteBox<N> {
        #[inline]
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<const N: usize> AsRef<[u8; N]> for ByteBox<N> {
        #[inline]
        fn as_ref(&self) -> &[u8; N] {
            self
        }
    }

    impl<const N: usize> AsRef<[u8]> for ByteBox<N> {
        #[inline]
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }

    impl<const N: usize> AsMut<[u8; N]> for ByteBox<N> {
        #[inline]
        fn as_mut(&mut self) -> &mut [u8; N] {
            self
        }
    }

    impl<const N: usize> AsMut<[u8]> for ByteBox<N> {
        #[inline]
        fn as_mut(&mut self) -> &mut [u8] {
            self.0.as_mut()
        }
    }

    impl<const N: usize> From<[u8; N]> for ByteBox<N> {
        #[inline]
        fn from(value: [u8; N]) -> Self {
            Self(Box::new(value))
        }
    }

    impl<const N: usize> From<Box<[u8; N]>> for ByteBox<N> {
        #[inline]
        fn from(value: Box<[u8; N]>) -> Self {
            Self(value)
        }
    }

    impl<const N: usize> Barse for ByteBox<N> {
        type ReadWith = ();

        type WriteWith = ();

        #[inline]
        fn read_with<E, B>(
            from: &mut B,
            _with: Self::ReadWith,
        ) -> Result<Self, crate::WrappedErr<B::Err>>
        where
            E: crate::Endian,
            B: crate::ByteSource,
        {
            let mut s = Self::default();
            from.read_slice(s.as_mut())?;
            Ok(s)
        }

        #[inline]
        fn write_with<E, B>(
            &self,
            to: &mut B,
            _with: Self::WriteWith,
        ) -> Result<(), crate::WrappedErr<B::Err>>
        where
            E: crate::Endian,
            B: crate::ByteSink,
        {
            to.write_slice(self.as_ref())?;
            Ok(())
        }
    }
}
