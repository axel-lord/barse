//! Struct Derive tests.
#![allow(dead_code)]

use ::barse::{
    self as barse2,
    endian::Little,
    util::{SliceSink, SliceSrc, UseEndian},
    Barse, ByteSinkExt, ByteSourceExt,
};

#[derive(Barse)]
#[barse(
    where(
        T: ::barse::Barse<ReadWith = (), WriteWith = ()>,
    ),
    error_path = ::barse::Error,
    crate_path = barse2,
    write_with = _with: i32,
    read_with = _with: u16,
)]
struct Wrap<T>(T);

/// Basic test.
#[test]
fn basic() {
    #[derive(Barse, PartialEq, Debug)]
    #[barse(field_prefix = field_)]
    struct TestStruct {
        a: f32,
        b: i64,
        c: u128,
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
