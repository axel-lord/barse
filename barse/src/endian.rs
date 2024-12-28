//! Trait implementations of [Endian][crate::Endian].

use crate::sealed::Sealed;

endian_trait!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

/// Endian selected at runtime, does not implement [Endian] trait.
#[cfg(feature = "util")]
#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Runtime {
    /// Use big endian.
    Big,

    /// Use little endian.
    Little,

    /// Use platform native endian.
    #[default]
    Native,
}

/// Macro to generate endian trait and impls.
macro_rules! endian_trait {
    ($($ty:ty),*) => {
        paste::paste! {
        #[doc = "Trait defining endianess, [Big], [Little] and [Native] is available."]
        #[doc(hidden)]
        pub trait Endian: Sealed {
            $(
            #[doc = concat!("Convert bytes to ", stringify!($ty)," value.")]
            fn [< $ty _from_bytes >](bytes: [u8; size_of::<$ty>()]) -> $ty;
            )*

            $(
            #[doc = concat!("Convert ", stringify!($ty)," value to bytes.")]
            fn [< $ty _to_bytes >](value: $ty) -> [u8; size_of::<$ty>()];
            )*
        }
        }

        endian_define! {
            Big: (be, $($ty),*),
            Little: (le, $($ty),*),
            Native: (ne, $($ty),*)
        }
    };
}
/// Macro to generate Endian types.
macro_rules! endian_define {
    ($($kind:ident: ($short:ident, $($ty:ty),*)),*) => {
        $(
            #[doc = concat!(stringify!($kind), " endian is used.")]
            #[expect(missing_debug_implementations, reason = "type cannot be constructed")]
            pub enum $kind {}
            impl Sealed for $kind {}

            endian_impl!($kind, $short, $($ty),*);
        )*
    };
}
/// Macro to implement endianess.
macro_rules! endian_impl {
    ($name:ident, $short:ident, $($ty:ty),*) => {
        paste::paste! {
        impl Endian for $name {
            $(
                fn [< $ty _from_bytes >](bytes: [u8; size_of::<$ty>()]) -> $ty {
                    $ty :: [< from_ $short _bytes >](bytes)
                }

                fn [< $ty _to_bytes >](value: $ty) -> [u8; size_of::<$ty>()] {
                    $ty :: [< to_ $short _bytes >](value)
                }
            )*
        }
        }
    };
}
use endian_define;
use endian_impl;
use endian_trait;
