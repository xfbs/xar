use crate::header;
use crate::{Header, ReadHeader};
use std::io::Cursor;

const NULL_XAR: &'static [u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/null.xar"));

#[test]
fn test_can_load_header() {
    let mut cursor = Cursor::new(NULL_XAR);
    let header = cursor.read_header().unwrap();
    assert!(header.check().is_ok());
}

#[test]
fn test_header_load_fails_with_invalid_magic() {
    for i in 0..4 {
        let mut copy: Vec<u8> = NULL_XAR.into();
        copy[i] = copy[i] + 1;
        let mut cursor = Cursor::new(&copy);
        let header = cursor.read_header().unwrap();
        assert_eq!(header.check(), Err(header::Error::MagicError));
    }
}
