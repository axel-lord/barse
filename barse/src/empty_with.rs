//! [Empty] trait.

use crate::sealed::Sealed;

/// Trait marking nested empty tuples and arrays.
///
/// Tuple impl goes up to 3 fields with any nesting.
pub trait Empty: Sealed {
    /// Get an instance of self.
    fn instance() -> Self;
}

impl Sealed for () {}
impl Empty for () {
    #[inline(always)]
    fn instance() -> Self {}
}

impl<T> Sealed for (T,) where T: Sealed {}
impl<T> Empty for (T,)
where
    T: Empty,
{
    #[inline(always)]
    fn instance() -> Self {
        (T::instance(),)
    }
}

impl<T, V> Sealed for (T, V)
where
    T: Sealed,
    V: Sealed,
{
}
impl<T, V> Empty for (T, V)
where
    T: Empty,
    V: Empty,
{
    #[inline(always)]
    fn instance() -> Self {
        (T::instance(), V::instance())
    }
}

impl<T, V, U> Sealed for (T, V, U)
where
    T: Sealed,
    V: Sealed,
    U: Sealed,
{
}
impl<T, V, U> Empty for (T, V, U)
where
    T: Empty,
    V: Empty,
    U: Empty,
{
    #[inline(always)]
    fn instance() -> Self {
        (T::instance(), V::instance(), U::instance())
    }
}

impl<T, const SIZE: usize> Sealed for [T; SIZE] where T: Sealed {}
impl<T, const SIZE: usize> Empty for [T; SIZE]
where
    T: Empty,
{
    #[inline(always)]
    fn instance() -> Self {
        [(); SIZE].map(|_| Empty::instance())
    }
}
