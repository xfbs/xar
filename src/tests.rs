use crate::header;
use crate::Header;
use std::io::Cursor;

const NULL_XAR: &'static [u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/null.xar"));

const NULL_TOC_SHA256_XAR: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/null_toc_sha256.xar"
));

const NULL_TOC_SHA512_XAR: &'static [u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/data/null_toc_sha512.xar"
));

#[test]
fn test_can_load_header() {
    let mut cursor = Cursor::new(NULL_XAR);
    let header = Header::from_read(&mut cursor).unwrap();
    assert!(header.check().is_ok());
    assert_eq!(header.size, 28);
    assert_eq!(header.version, 1);
    assert_eq!(header.toc_length_compressed, 256 + 97);
    assert_eq!(header.toc_length_uncompressed, 512 + 198);
    assert_eq!(header.checksum_alg, header::ChecksumAlg::SHA1);
}

#[test]
fn test_can_load_header_toc_cksum_sha256() {
    let mut cursor = Cursor::new(NULL_TOC_SHA256_XAR);
    let header = Header::from_read(&mut cursor).unwrap();
    assert!(header.check().is_ok());
    assert_eq!(header.size, 28);
    assert_eq!(header.version, 1);
    assert_eq!(header.toc_length_compressed, 256 + 101);
    assert_eq!(header.toc_length_uncompressed, 512 + 200);
    assert_eq!(header.checksum_alg, header::ChecksumAlg::SHA256);
}

#[test]
fn test_can_load_header_toc_cksum_sha512() {
    let mut cursor = Cursor::new(NULL_TOC_SHA512_XAR);
    let header = Header::from_read(&mut cursor).unwrap();
    assert!(header.check().is_ok());
    assert_eq!(header.size, 28);
    assert_eq!(header.version, 1);
    assert_eq!(header.toc_length_compressed, 357);
    assert_eq!(header.toc_length_uncompressed, 712);
    assert_eq!(header.checksum_alg, header::ChecksumAlg::SHA512);
}

#[test]
fn test_header_load_fails_with_invalid_magic() {
    for i in 0..4 {
        let mut copy: Vec<u8> = NULL_XAR.into();
        copy[i] = copy[i] + 1;
        let mut cursor = Cursor::new(&copy);
        let header = Header::from_read(&mut cursor).unwrap();
        assert_eq!(header.check(), Err(header::Error::MagicError));
    }
}

#[test]
fn test_header_load_fails_with_invalid_version() {
    let mut copy: Vec<u8> = NULL_XAR.into();
    copy[6] = 0;
    copy[7] = 0;
    let mut cursor = Cursor::new(&copy);
    let header = Header::from_read(&mut cursor).unwrap();
    assert_eq!(header.check(), Err(header::Error::Version(0)));
}
