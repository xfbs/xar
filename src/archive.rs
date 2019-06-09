use crate::header::Header;
use crate::toc::Toc;
use crate::error::*;
use std::io::{Read, Seek};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Archive {
    header: Header,
    toc: Toc,
}

impl Archive {
    pub fn from_read<T: Read + Seek>(reader: &mut T) -> Result<Archive> {
        let header = Header::from_read(reader).chain_err(|| "Error reading header")?;
        let toc = Toc::from_read(reader, header.toc_length_uncompressed as usize).chain_err(|| "Error reading archive")?;

        Ok(Archive {
            header,
            toc
        })
    }
}

impl std::fmt::Display for Archive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.header, self.toc)
    }
}
