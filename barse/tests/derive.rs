#![cfg(feature = "derive")]
#![allow(dead_code)]
use barse::endian;
use barse::reader::Cursor;

use barse::{from_reader::Padding, FromByteReader};

#[derive(Debug, FromByteReader, PartialEq, Eq)]
struct StructDerive {
    a: u8,
    b: u16,
    #[barse(reveal)]
    c: u32,
    d: u64,
    e: u128,
}

impl StructDerive {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.a.to_le_bytes());
        data.extend(self.b.to_le_bytes());
        data.extend(self.c.to_le_bytes());
        data.extend(self.d.to_le_bytes());
        data.extend(self.e.to_le_bytes());
        data
    }
}

#[test]
pub fn parse_derived_struct() {
    let test = StructDerive {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
    };

    let data = test.to_bytes();

    let parsed = StructDerive::from_byte_reader::<_, endian::Little>(Cursor::new(&data)).unwrap();

    assert_eq!(test, parsed)
}

#[derive(Debug, FromByteReader, PartialEq, Eq)]
struct TupleStructDerive(u32, Padding<12>, u128);

impl TupleStructDerive {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.extend(self.0.to_le_bytes());
        data.extend([0; 12]);
        data.extend(self.2.to_le_bytes());

        data
    }
}

#[test]
pub fn parse_derived_tuple_struct() {
    let test = TupleStructDerive(16, Padding::default(), 256);

    let data = test.to_bytes();

    let parsed =
        TupleStructDerive::from_byte_reader::<_, endian::Little>(Cursor::new(&data)).unwrap();

    assert_eq!(parsed, test)
}

#[derive(FromByteReader)]
#[barse(err = "anyhow::Error")]
struct FromStruct {
    #[barse(from = "u8")]
    pub a: u32,
    pub b: u32,
    #[barse(try_from = "u32")]
    pub c: f64,
}

#[derive(FromByteReader)]
struct WrappedFromStruct(FromStruct);
