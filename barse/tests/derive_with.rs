use std::io::Cursor;

use barse::{wrap, Error, FromByteReader};

fn u8_is_digit(value: u8) -> bool {
    value.is_ascii_digit()
}

#[derive(FromByteReader)]
struct Option4U8 {
    #[barse(reveal = "first")]
    _first: u8,
    #[barse(with = "u8_is_digit(*first)")]
    slice: Option<[u8; 4]>,
}

#[test]
pub fn option() {
    fn parse_seq(seq: &[u8]) -> Result<Option<[u8; 4]>, Error> {
        Option4U8::from_byte_reader(Cursor::new(seq)).map(|opt| opt.slice)
    }

    // first character cannot be parsed
    assert!(parse_seq(b"").is_err());

    // second to fifth character missing
    assert!(parse_seq(b"4").is_err());
    assert!(parse_seq(b"7").is_err());

    // fifth character missing
    assert!(parse_seq(b"9wow").is_err());

    // first character is not a digit
    assert!(parse_seq(b"Hello").unwrap().is_none());
    assert!(parse_seq(b"There").unwrap().is_none());

    // successes
    assert_eq!(parse_seq(b"2nice").unwrap().as_ref(), Some(b"nice"));
    assert_eq!(parse_seq(b"5full").unwrap().as_ref(), Some(b"full"));

    // successes with larger input
    assert_eq!(parse_seq(b"7there").unwrap().as_ref(), Some(b"ther"));
}

#[derive(FromByteReader)]
struct VecU8 {
    #[barse(from = "u8", reveal = "size")]
    _size: usize,
    #[barse(with = "wrap::Len(*size)")]
    vec: Vec<u8>,
}

#[test]
pub fn vec() {
    fn parse_seq(seq: &[u8]) -> Result<Vec<u8>, Error> {
        VecU8::from_byte_reader(Cursor::new(seq)).map(|vec| vec.vec)
    }

    assert!(parse_seq(b"").is_err());

    assert!(parse_seq(b"\x05nice").is_err());
    assert_eq!(parse_seq(b"\x02Ok").unwrap(), b"Ok");
    assert_eq!(parse_seq(b"\x05Hello").unwrap(), b"Hello");
    assert_eq!(parse_seq(b"\x00Nice to meet you").unwrap(), b"");
}

#[derive(FromByteReader)]
struct VecOpt2U8 {
    #[barse(from = "u8", reveal = "size")]
    _size: usize,
    #[barse(with = "wrap::Len(*size)", reveal = "status")]
    _status: Vec<u8>,
    #[barse(with = "(wrap::Iter(status), |b: &u8| *b != b'0')")]
    vec: Vec<Option<[u8; 2]>>,
}

#[test]
pub fn vec_option() {
    fn parse_seq(seq: &[u8]) -> Result<Vec<Option<[u8; 2]>>, Error> {
        VecOpt2U8::from_byte_reader(Cursor::new(seq)).map(|v| v.vec)
    }

    assert!(parse_seq(b"").is_err());

    assert_eq!(
        parse_seq(b"\x040101abcd").unwrap(),
        vec![
            None,
            Some((b"ab" as &[u8]).try_into().unwrap()),
            None,
            Some((b"cd" as &[u8]).try_into().unwrap())
        ]
    );
}

#[derive(FromByteReader)]
struct OptVecU8 {
    #[barse(from = "u8", reveal = "size")]
    _size: usize,
    #[barse(with = "(*size != 0, wrap::Len(*size))")]
    vec: Option<Vec<u8>>,
}

#[test]
pub fn option_vec() {
    fn parse_seq(seq: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        OptVecU8::from_byte_reader(Cursor::new(seq)).map(|v| v.vec)
    }

    assert!(parse_seq(b"").is_err());

    assert_eq!(parse_seq(b"\x00Hello").unwrap(), None);
    assert_eq!(
        parse_seq(b"\x05There").unwrap(),
        Some(Vec::from(b"There" as &[u8]))
    );
}

#[derive(FromByteReader)]
#[barse(with = "bool", reveal = "cond")]
struct CarryOpt(#[barse(with = "cond")] Option<u8>);
