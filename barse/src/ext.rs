//! Extension traits.

pub use self::{
    barse_read::BarseReadExt, barse_write::BarseWriteExt, byte_sink_ext::ByteSinkExt,
    byte_source_ext::ByteSourceExt,
};

#[cfg(feature = "barse_as")]
pub use self::barse_as::{ReadAsExt, WriteAsExt};

#[cfg(feature = "std")]
pub use if_std::{AsByteSink, AsByteSource};

mod byte_source_ext;

mod byte_sink_ext;

mod barse_read;

mod barse_write;

#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
mod if_std;

#[cfg_attr(docsrs, doc(cfg(feature = "barse_as")))]
#[cfg(feature = "barse_as")]
mod barse_as;

/// Trait marking nested empty tuples and arrays.
///
/// Tuple impl goes up to 3 fields with any nesting.
pub trait EmptyWith {
    /// Get an instance of self.
    fn instance() -> Self;
}

impl EmptyWith for () {
    #[inline(always)]
    fn instance() -> Self {}
}

impl<T> EmptyWith for (T,)
where
    T: EmptyWith,
{
    #[inline(always)]
    fn instance() -> Self {
        (T::instance(),)
    }
}

impl<T, V> EmptyWith for (T, V)
where
    T: EmptyWith,
    V: EmptyWith,
{
    #[inline(always)]
    fn instance() -> Self {
        (T::instance(), V::instance())
    }
}

impl<T, V, U> EmptyWith for (T, V, U)
where
    T: EmptyWith,
    V: EmptyWith,
    U: EmptyWith,
{
    #[inline(always)]
    fn instance() -> Self {
        (T::instance(), V::instance(), U::instance())
    }
}

impl<T, const SIZE: usize> EmptyWith for [T; SIZE]
where
    T: EmptyWith,
{
    #[inline(always)]
    fn instance() -> Self {
        ::core::array::from_fn(|_| T::instance())
    }
}
