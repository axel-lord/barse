//! Helper types implementing [Barse][crate::Barse] for common usages.

pub use self::{
    byte_array::ByteArray,
    padding::Padding,
    slice_sink::{SliceSink, SliceSinkFull},
    slice_source::{SliceSrc, SliceSrcEmpty},
    use_endian::UseEndian,
};

mod byte_array;

mod use_endian;

mod padding;

mod slice_source;

mod slice_sink;

#[cfg(feature = "zerocopy")]
pub mod zerocopy {
    //! [Barse] implementations using zerocopy.

    use ::zerocopy::{FromBytes, Immutable, IntoBytes};

    use crate::Barse;

    /// Value implementing [FromBytes] and [IntoBytes].
    ///
    /// Endianess will be platform specific ([Native][crate::endian::Native]) in all cases.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UseIntoFromBytes<T>(T);

    impl<T> UseIntoFromBytes<T>
    where
        T: FromBytes + IntoBytes + Immutable + Barse,
    {
        /// Construct a new [UseIntoFromBytes] from a value.
        pub const fn new(value: T) -> Self {
            Self(value)
        }

        /// Unwrap [UseIntoFromBytes] to wrapped value.
        pub fn into_inner(self) -> T {
            let Self(value) = self;
            value
        }
    }

    impl<T> Barse for UseIntoFromBytes<T>
    where
        T: FromBytes + IntoBytes + Immutable + Barse,
    {
        fn read<E, B>(from: &mut B) -> Result<Self, crate::Error<B::Err>>
        where
            E: crate::Endian,
            B: crate::ByteSource,
        {
            let mut value = T::new_zeroed();

            from.read_slice(value.as_mut_bytes())?;

            Ok(Self(value))
        }

        fn write<E, B>(&self, to: &mut B) -> Result<(), crate::Error<B::Err>>
        where
            E: crate::Endian,
            B: crate::ByteSink,
        {
            to.write_slice(self.0.as_bytes())?;

            Ok(())
        }
    }
}

#[cfg(feature = "bytemuck")]
pub mod bytemuck {
    //! [Barse] implementations using bytemuck.

    use ::bytemuck::{AnyBitPattern, NoUninit};

    use crate::Barse;

    /// Value implementing [AnyBitPattern] and [NoUninit].
    ///
    /// Endianess will be platform specific ([Native][crate::endian::Native]) in all cases.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UseAnyBitPattern<T>(T);

    impl<T> UseAnyBitPattern<T>
    where
        T: AnyBitPattern + NoUninit + Barse,
    {
        /// Construct a new [UseAnyBitPattern] from a value.
        pub const fn new(value: T) -> Self {
            Self(value)
        }

        /// Unwrap [UseAnyBitPattern] to wrapped value.
        pub const fn into_inner(self) -> T {
            let Self(value) = self;
            value
        }
    }

    impl<T> Barse for UseAnyBitPattern<T>
    where
        T: AnyBitPattern + NoUninit,
    {
        fn read<E, B>(from: &mut B) -> Result<Self, crate::Error<B::Err>>
        where
            E: crate::Endian,
            B: crate::ByteSource,
        {
            let mut value = T::zeroed();

            from.read_slice(::bytemuck::bytes_of_mut(&mut value))?;

            Ok(Self(value))
        }

        fn write<E, B>(&self, to: &mut B) -> Result<(), crate::Error<B::Err>>
        where
            E: crate::Endian,
            B: crate::ByteSink,
        {
            to.write_slice(::bytemuck::bytes_of(&self.0))?;
            Ok(())
        }
    }
}
