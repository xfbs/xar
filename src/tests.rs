use crate::{Header, ReadHeader};
use std::io::Cursor;

const NULL_XAR: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/null.xar"));

#[test]
fn test_can_load_header() {
    let mut cursor = Cursor::new(NULL_XAR);

    let header = cursor.read_header().unwrap();

    assert!(header.check().is_ok());
}
