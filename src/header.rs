use byteorder::{BigEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Read;
use super::Error;

/// Minimal size of header.
const HEADER_SIZE: usize = 28;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ChecksumAlg {
    None,
    SHA1,
    MD5,
    SHA256,
    SHA512,
    Other(String),
    Unknown(u32),
}

impl From<u32> for ChecksumAlg {
    fn from(i: u32) -> ChecksumAlg {
        match i {
            0 => ChecksumAlg::None,
            1 => ChecksumAlg::SHA1,
            2 => ChecksumAlg::MD5,
            3 => ChecksumAlg::SHA256,
            4 => ChecksumAlg::SHA512,
            i => ChecksumAlg::Unknown(i),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub magic: u32,
    pub size: u16,
    pub version: u16,
    pub toc_length_compressed: u64,
    pub toc_length_uncompressed: u64,
    pub checksum_alg: ChecksumAlg,
    pub data: Vec<u8>,
}

impl Header {
    pub fn check(&self) -> Result<(), Error> {
        // needs to start with magic sequence 'xar!'.
        if self.magic != 0x78617221 {
            return Err(Error::MagicError);
        }

        // header size has to be legal.
        if self.size < 28 {
            return Err(Error::HeaderTooSmall(self.size));
        }

        // we only accept version 1.
        if self.version != 1 {
            return Err(Error::Version(self.version));
        }

        Ok(())
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_read<T: Read>(reader: &mut T) -> Result<Header, std::io::Error> {
        let magic = reader.read_u32::<BigEndian>()?;
        let size = reader.read_u16::<BigEndian>()?;
        let version = reader.read_u16::<BigEndian>()?;
        let toc_length_compressed = reader.read_u64::<BigEndian>()?;
        let toc_length_uncompressed = reader.read_u64::<BigEndian>()?;
        let checksum_alg = reader.read_u32::<BigEndian>()?;

        // Read extra data until we've read in the whole header.
        let data_size = (size as usize).saturating_sub(HEADER_SIZE);
        let mut data = Vec::with_capacity(data_size);
        data.resize(data_size, 0);
        reader.read_exact(&mut data)?;

        let checksum_alg = checksum_alg.into();

        Ok(Header {
            magic,
            size,
            version,
            toc_length_compressed,
            toc_length_uncompressed,
            checksum_alg,
            data,
        })
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:25}: {}\n", "magic", self.magic)?;
        write!(f, "{:25}: {}\n", "size (header)", self.size)?;
        write!(
            f,
            "{:25}: {}\n",
            "toc length (compressed)", self.toc_length_compressed
        )?;
        write!(f, "{:25}: {}\n", "toc length", self.toc_length_uncompressed)?;
        write!(f, "{:25}: {:?}\n", "checksum_alg", self.checksum_alg)?;
        write!(f, "{:25}: {:?}", "extra data", self.data)
    }
}
