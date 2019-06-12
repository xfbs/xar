use crate::error::*;
use std::io::Read;
use libflate::zlib::Decoder;
use std::fmt;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Cursor;

#[derive(Debug, Clone)]
pub struct Toc {
    data: String,
}

impl Toc {
    pub fn from_read<T: Read>(reader: &mut T, expected: usize) -> Result<Toc> {
        // decompress table of contents
        let mut data = Vec::with_capacity(expected);
        let mut decoder = Decoder::new(reader).chain_err(|| "Error decompressing table of contents")?;
        decoder.read_to_end(&mut data).chain_err(|| "Error decompressing table of contents")?;
        let data = String::from_utf8(data).chain_err(|| "Error decompressing table of contents")?;

        let mut reader = Reader::from_str(&data);
        let mut buf = Vec::new();

        loop {
            let event = match reader.read_event(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(e) => e,
                Err(e) => break,
            };
            println!("{}", String::from_utf8(buf.clone()).unwrap());
        }

        Ok(Toc {
            data
        })
    }
}

impl std::fmt::Display for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
