use crate::error::*;
use std::io::Read;
use libflate::zlib::Decoder;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Toc {
    data: Vec<u8>
}

impl Toc {
    pub fn from_read<T: Read>(reader: &mut T, expected: usize) -> Result<Toc> {
        let mut data = Vec::with_capacity(expected);
        let mut decoder = Decoder::new(reader).chain_err(|| "Error decompressing table of contents")?;
        decoder.read_to_end(&mut data).chain_err(|| "Error decompressing table of contents")?;

        Ok(Toc {
            data
        })
    }
}

impl std::fmt::Display for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.data.clone()).unwrap_or("".into()))
    }
}
