//! Trait and implementations for endianess.

use crate::sealed::Sealed;

endian_trait!(u8, u16, u32, u64, u128, i8, i32, i64, i128);

macro_rules! endian_trait {
    ($($ty:ty),*) => {
        paste::paste! {
        #[doc = "Trait defining endianess, [Big], [Little] and [Native] is available."]
        pub trait Endian: Sealed {
            $(
            #[doc = concat!("Read a value from and array of bytes.")]
            fn [< $ty _from_bytes >](bytes: [u8; size_of::<$ty>()]) -> $ty;
            )*

            $(
            #[doc = concat!("Read a value from and array of bytes.")]
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
macro_rules! endian_define {
    ($($kind:ident: ($short:ident, $($ty:ty),*)),*) => {
        $(
            #[doc = concat!(stringify!($kind), " endian is used.")]
            pub enum $kind {}
            impl Sealed for $kind {}

            endian_impl!($kind, $short, $($ty),*);
        )*
    };
}
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
