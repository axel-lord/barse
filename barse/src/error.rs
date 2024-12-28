//! [Error] type.

use ::core::{any::TypeId, fmt::Display};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Crate error type used to report sink/source specific errors and general read/write errors.
#[derive(Debug)]
pub enum WrappedErr<E> {
    /// Wrapped source/sink error.
    Wrapped(E),

    /// Other error produced by barse impl.
    Other(Error),
}

impl<E> From<E> for WrappedErr<E> {
    fn from(value: E) -> Self {
        Self::Wrapped(value)
    }
}

impl<E> WrappedErr<E> {
    /// Convert [Error] into wrapped error.
    #[inline]
    pub const fn from_err(value: Error) -> Self {
        Self::Other(value)
    }

    /// Merge variants into a type which may converted to from both of them.
    #[inline]
    pub fn merge_into<T>(self) -> T
    where
        T: From<Error> + From<E>,
    {
        match self {
            WrappedErr::Wrapped(err) => err.into(),
            WrappedErr::Other(err) => err.into(),
        }
    }
}

impl<E> Display for WrappedErr<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WrappedErr::Wrapped(e) => Display::fmt(e, f),
            WrappedErr::Other(error) => Display::fmt(error, f),
        }
    }
}

impl<E> ::core::error::Error for WrappedErr<E> where E: ::core::error::Error {}

/// Crate error type without any source/sink errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error is only a message.
    Msg(&'static str),

    /// Error is tracked using a [TypeId] and a payload of some kind (perhaps an index into a
    /// list).
    Any(TypeId, u64),

    /// Error is tracked using a reference to a static [::core::error::Error] implementor.
    Dyn(&'static (dyn ::core::error::Error + Send + Sync)),

    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    /// Error is tracked using a boxed [::core::error::Error].
    Box(alloc::boxed::Box<dyn ::core::error::Error + Send + Sync>),
}

impl From<&'static str> for Error {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self::Msg(value)
    }
}

impl From<&'static (dyn ::core::error::Error + Send + Sync)> for Error {
    #[inline]
    fn from(value: &'static (dyn ::core::error::Error + Send + Sync)) -> Self {
        Self::Dyn(value)
    }
}

#[cfg(feature = "alloc")]
impl From<alloc::boxed::Box<dyn ::core::error::Error + Send + Sync>> for Error {
    #[inline]
    fn from(value: alloc::boxed::Box<dyn ::core::error::Error + Send + Sync>) -> Self {
        Self::Box(value)
    }
}

#[cfg(feature = "std")]
impl From<Error> for ::std::io::Error {
    #[inline]
    fn from(value: Error) -> Self {
        match value {
            Error::Box(err) => ::std::io::Error::other(err),
            err => ::std::io::Error::other(err),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Msg(msg) => f.write_str(msg),
            Error::Any(type_id, id) => write!(f, "type id error [{type_id:?}], {id}"),
            Error::Dyn(err) => Display::fmt(err, f),
            #[cfg(feature = "alloc")]
            Error::Box(err) => Display::fmt(err, f),
        }
    }
}

impl Error {
    /// Convert error into any kind of [WrappedErr].
    #[inline]
    pub const fn into_wrapped<E>(self) -> WrappedErr<E> {
        WrappedErr::Other(self)
    }
}

impl ::core::error::Error for Error {}
