use crate::header::Header;
use crate::toc::Toc;
use crate::*;
use failure::Error;
use std::fmt;
use std::io::{Read, Seek};
use libflate::zlib::Decoder;
use std::io::SeekFrom;

#[derive(Debug, Clone)]
pub struct Archive {
    header: Header,
    toc: Toc,
}

impl Archive {
    pub fn from_read<T: Read>(reader: &mut T) -> Result<Archive, Error> {
        let header = Header::from_read(reader)?;

        // TODO: verify that only header.toc_length_compressed bytes were read.
        let toc = Toc::from_read(reader, header.toc_length_uncompressed as usize)?;

        Ok(Archive { header, toc })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn toc(&self) -> &Toc {
        &self.toc
    }
}

impl std::fmt::Display for Archive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.header, self.toc)
    }
}
