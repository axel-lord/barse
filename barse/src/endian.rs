//! Utilities for dealing with endianess.

use paste::paste;

/// Type representing an endian of either type. Does not implement [Endian].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Either {
    /// Big endian.
    Big,
    /// Little endian.
    Little,
}

impl From<Big> for Either {
    fn from(_: Big) -> Self {
        Self::Big
    }
}

impl From<Little> for Either {
    fn from(_: Little) -> Self {
        Self::Little
    }
}

/// Type representing big endian.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Big;

/// Type representing little endian.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Little;

endian! {
    u8, 1,
    i8, 1,
    u16, 2,
    i16, 2,
    u32, 4,
    i32, 4,
    u64, 8,
    i64, 8,
    u128, 16,
    i128, 16,
}

macro_rules! endian_trait {
    ($($req_id: ty, $req_size: literal,)+) => {
        paste! {
        #[doc = "Trait to express endianess in the type system."]
        pub trait Endian {
            $(
            #[doc = "Parse using specified endian."]
            fn [< parse_ $req_id >](from: [u8; $req_size]) -> $req_id;
            )*
        }
        }
    };
}
use endian_trait;

macro_rules! endian_impl {
    ($($req_id: ty, $req_size: literal,)+) => {
        paste! {
        impl Endian for Big {
            $(
            fn [< parse_ $req_id >](from: [u8; $req_size]) -> $req_id {
                $req_id::from_be_bytes(from)
            }
            )*
        }
        impl Endian for Little{
            $(
            fn [< parse_ $req_id >](from: [u8; $req_size]) -> $req_id {
                $req_id::from_le_bytes(from)
            }
            )*
        }
        }
    };
}
use endian_impl;

macro_rules! endian {
    ($($req_id: ty, $req_size: literal,)+) => {
        endian_trait! {
        $(
            $req_id, $req_size,
        )*
        }

        endian_impl! {
            $(
            $req_id, $req_size,
            )*
        }
    };
}
use endian;
