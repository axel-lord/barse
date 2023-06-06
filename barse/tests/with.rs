use std::{assert_eq, io::Cursor};

use barse::{wrap, Error, FromByteReader, FromByteReaderWith};

#[test]
pub fn option() {
    /// Parse a seq of 5 (or more) if the first byte is a number the rest of the string is parsed.
    fn parse_seq(seq: &[u8]) -> Result<Option<[u8; 4]>, Error> {
        let mut reader = Cursor::new(seq);
        let first = <u8 as FromByteReader>::from_byte_reader(&mut reader)?;

        FromByteReaderWith::from_byte_reader_with(reader, (first as char).is_ascii_digit())
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

#[test]
pub fn vec() {
    fn parse_seq(seq: &[u8]) -> Result<Vec<u8>, Error> {
        let mut reader = Cursor::new(seq);
        let size: usize = u8::from_byte_reader(&mut reader)?.into();

        Vec::from_byte_reader_with(reader, wrap::Len(size))
    }

    assert!(parse_seq(b"").is_err());

    assert!(parse_seq(b"\x05nice").is_err());
    assert_eq!(parse_seq(b"\x02Ok").unwrap(), b"Ok");
    assert_eq!(parse_seq(b"\x05Hello").unwrap(), b"Hello");
    assert_eq!(parse_seq(b"\x00Nice to meet you").unwrap(), b"");
}

#[test]
pub fn vec_option() {
    fn parse_seq(seq: &[u8]) -> Result<Vec<Option<[u8; 2]>>, Error> {
        let mut reader = Cursor::new(seq);
        let size: usize = u8::from_byte_reader(&mut reader)?.into();

        let state = Vec::<u8>::from_byte_reader_with(&mut reader, wrap::Len(size))?;

        Vec::from_byte_reader_with(reader, (wrap::Iter(&state), |b: &u8| *b != b'0'))
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

#[test]
pub fn option_vec() {
    fn parse_seq(seq: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let mut reader = Cursor::new(seq);

        let size: usize = u8::from_byte_reader(&mut reader)?.into();

        Option::from_byte_reader_with(reader, (size != 0, wrap::Len(size)))
    }

    assert!(parse_seq(b"").is_err());

    assert_eq!(parse_seq(b"\x00Hello").unwrap(), None);
    assert_eq!(
        parse_seq(b"\x05There").unwrap(),
        Some(Vec::from(b"There" as &[u8]))
    );
}
