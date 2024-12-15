//! Struct Derive tests.
#![allow(dead_code)]

use barse as barse2;

#[derive(::barse::Barse)]
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
    println!("hello")
}
