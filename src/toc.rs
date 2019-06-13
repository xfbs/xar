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
    creation_time: Option<String>,
}

impl Toc {
    pub fn new() -> Self {
        Toc {
            data: String::new(),
            creation_time: None,
        }
    }

    pub fn from_read<T: Read>(reader: &mut T, expected: usize) -> Result<Toc> {
        // decompress table of contents
        let mut data = Vec::with_capacity(expected);
        let mut decoder = Decoder::new(reader).chain_err(|| "Error decompressing table of contents")?;
        decoder.read_to_end(&mut data).chain_err(|| "Error decompressing table of contents")?;
        let data = String::from_utf8(data).chain_err(|| "Error decompressing table of contents")?;

        let mut reader = Reader::from_str(&data);
        let mut buf = Vec::new();

        let mut toc = Toc::new();

        loop {
            let event = match reader.read_event(&mut buf) {
                Ok(e) => e,
                Err(e) => break,
            };

            match event {
                Event::Start(ref e) => {
                    match e.name() {
                        b"creation-time" => toc.read_creation_time(&mut reader),
                        _ => {}
                    }
                },
                Event::Text(ref e) => {
                },
                Event::Eof => break,
                _ => {},
            }

            buf.clear();
        }

        toc.data = data;

        Ok(toc)
    }

    fn read_creation_time<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>) {
        let mut buf = Vec::new();

        match reader.read_event(&mut buf) {
            Ok(Event::Text(ref e)) => {
                self.creation_time = Some(String::from_utf8_lossy(e.escaped()).to_string());
            },
            _ => {},
        }
    }
}

impl std::fmt::Display for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:25}: {}\n", "creation_time", self.creation_time.as_ref().unwrap_or(&"None".to_string()));
        write!(f, "{}", self.data)
    }
}
