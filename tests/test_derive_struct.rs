//! Struct Derive tests.
#![allow(dead_code)]

use ::barse::{
    self as barse2,
    util::{SliceSink, SliceSrc},
    ByteSinkExt, ByteSourceExt,
};

#[derive(::barse::Barse, PartialEq, Debug)]
#[barse(field_prefix = field_)]
struct TestStruct {
    a: f32,
    b: i64,
    c: u128,
    d: [u8; 5],
}

#[derive(::barse::Barse)]
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

/// Entry
#[test]
fn basic() {
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
