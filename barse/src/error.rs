//! [Error] type.

use ::core::fmt::Display;

/// Crate error type used to report sink/source specific errors and general read/write errors.
#[derive(Debug)]
pub enum Error<E> {
    /// Wrapped source/sink error.
    Wrapped(E),

    /// Some means of identifying a custom error by a code.
    Custom(u64),

    /// Some means of identifying a custom error by reference.
    Any(&'static (dyn ::core::any::Any + Send + Sync)),

    /// Some means of identifying a custom error as a message.
    Message(&'static str),
}

impl<E> Display for Error<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Wrapped(e) => E::fmt(e, f),
            Error::Custom(c) => write!(f, "Custom Error: {c}"),
            Error::Any(a) => write!(f, "Custom Error: {:?}", *a),
            Error::Message(m) => f.write_str(m),
        }
    }
}

impl<E> ::core::error::Error for Error<E>
where
    E: 'static + ::core::error::Error,
{
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        if let Self::Wrapped(err) = self {
            Some(err)
        } else {
            None
        }
    }
}

impl<E> From<E> for Error<E> {
    fn from(value: E) -> Self {
        Self::Wrapped(value)
    }
}
