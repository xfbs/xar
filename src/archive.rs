use crate::header::Header;
use crate::error::*;
use std::io::Read;

pub struct Archive {
    header: Header,
}

impl Archive {
    pub fn from_read<T: Read>(reader: &mut T) -> Result<Archive> {
        Ok(Archive {
            header: Header::from_read(reader).chain_err(|| "Error reading header")?
        })
    }
}
