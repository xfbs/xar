use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, PartialEq)]
pub enum Error {
    MagicError,
}

#[derive(Debug, Clone, Copy)]
pub struct Header {
    magic: u32,
    size: u16,
    version: u16,
    toc_length_compressed: u64,
    toc_length_uncompressed: u64,
    checksum_alg: u32,
}

impl Header {
    pub fn new(magic: u32, size: u16, version: u16, toc_length_compressed: u64, toc_length_uncompressed: u64, checksum_alg: u32) -> Header {
        Header { magic, size, version, toc_length_compressed, toc_length_uncompressed, checksum_alg }
    }

    pub fn check(&self) -> Result<(), Error> {
        if self.magic != 0x78617221 {
            return Err(Error::MagicError);
        }

        Ok(())
    }
}

pub trait ReadHeader {
    fn read_header(&mut self) -> Result<Header, std::io::Error>;
}

impl<T> ReadHeader for T where T: ReadBytesExt {
    fn read_header(&mut self) -> Result<Header, std::io::Error> {
        let magic = self.read_u32::<BigEndian>()?;
        let size = self.read_u16::<BigEndian>()?;
        let version = self.read_u16::<BigEndian>()?;
        let toc_length_compressed = self.read_u64::<BigEndian>()?;
        let toc_length_uncompressed = self.read_u64::<BigEndian>()?;
        let checksum_alg = self.read_u32::<BigEndian>()?;

        Ok(Header { magic, size, version, toc_length_compressed, toc_length_uncompressed, checksum_alg })
    }
}
