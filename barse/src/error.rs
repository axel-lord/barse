//! [Error] type.

use ::core::{any::TypeId, fmt::Display};

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
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum Error {
    /// Error is only a message.
    Msg(&'static str),

    /// Error is tracked using a [TypeId] and a uique id of some kind (perhaps an index into a
    /// list).
    Any(TypeId, u64),
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Self::Msg(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Msg(msg) => f.write_str(msg),
            Error::Any(type_id, id) => write!(f, "type id error [{type_id:?}], {id}"),
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
