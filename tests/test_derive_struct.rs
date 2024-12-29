//! Struct Derive tests.
#![allow(dead_code)]

use ::barse::{Barse, ReadAs, WriteAs};

#[derive(Barse)]
struct Simple {
    a: u8,
    b: u16,
    c: u32,
}

use ::std::marker::PhantomData;

use ::barse::{
    endian::Little,
    ext::{ByteSinkExt, ByteSourceExt},
    util::{SliceSink, SliceSrc, UseEndian},
};

#[derive(Barse)]
#[barse(
    write_with = hello: i32,
    read_with = hello: u16,
)]
#[barse(
    where
        T: ::barse::Barse<ReadWith = u16, WriteWith = i32>
)]
struct Wrap<T>(#[barse(with)] T)
where
    T: Sized;

#[derive(Barse)]
#[barse(
    endian = ::barse::endian::Little,
    read_with = <T as Barse>::ReadWith,
    write_with = <T as Barse>::WriteWith
)]
struct AlwaysLittle<T>(#[barse(with)] T);

#[derive(Barse)]
#[barse(
    where
        T: Barse,
        E: barse::Endian,
)]
#[barse(
    read_with = T::ReadWith,
    write_with = T::WriteWith,
    endian = E,
)]
struct WithEndian<T, E>(#[barse(with)] T, #[barse(ignore)] PhantomData<fn() -> E>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SizedVec;

impl ReadAs<Vec<i32>, u16> for SizedVec {
    fn read_with<E, B>(self, from: &mut B, with: u16) -> Result<Vec<i32>, barse::WrappedErr<B::Err>>
    where
        E: barse::Endian,
        B: barse::ByteSource,
    {
        (0..with)
            .map(|_| from.read::<_, E>())
            .collect::<Result<Vec<_>, _>>()
    }
}

impl WriteAs<Vec<i32>, ()> for SizedVec {
    fn write_with<E, B>(
        self,
        value: &Vec<i32>,
        to: &mut B,
        _with: (),
    ) -> Result<(), barse::WrappedErr<B::Err>>
    where
        E: barse::Endian,
        B: barse::ByteSink,
    {
        for item in value {
            to.write::<_, E>(item)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Barse)]
struct PrefixedVec {
    size: u16,
    #[barse(as SizedVec)]
    #[barse(read_with = size)]
    vec: Vec<i32>,
}

/// Basic test.
#[test]
fn basic() {
    #[derive(Barse, PartialEq, Debug)]
    #[barse(field_prefix = field_)]
    struct TestStruct {
        #[barse(endian = barse::endian::Little)]
        a: f32,
        b: i64,
        #[barse(endian = barse::endian::Big)]
        c: u128,
        #[barse(bytes = 5)]
        d: [u8; 5],
    }

    let test_struct = TestStruct {
        a: 5.6,
        b: -15,
        c: u64::MAX as u128 * 16,
        d: *b"hello",
    };

    let mut buf = [0u8; size_of::<TestStruct>()];

    SliceSink::new(&mut buf).write_be(&test_struct).unwrap();

    let barsed_test_struct = SliceSrc::new(&buf).read_be::<TestStruct>().unwrap();

    assert_eq!(test_struct, barsed_test_struct);
}

/// Test of deterministic property with different types.
#[test]
fn deterministic() {
    #[derive(Debug, PartialEq, Eq, Barse, Clone, Copy)]
    struct A {
        a: i16,
        b: u16,
        c: i32,
    }
    #[derive(Debug, PartialEq, Eq, Barse, Clone, Copy)]
    struct B(i16, u16, i32);

    let (a, b, c) = (-5, 67, -35_000);

    let struct_a = A { a, b, c };
    let struct_b = B(a, b, c);

    let mut buf = [0u8; size_of::<A>()];

    SliceSink::new(&mut buf)
        .write_be(&UseEndian::<A, Little>::new(struct_a))
        .unwrap();

    let barsed_b = SliceSrc::new(&buf).read_le::<B>().unwrap();

    assert_eq!(struct_b, barsed_b);
}
