use crate::error::*;
use libflate::zlib::Decoder;
use quick_xml::events::{BytesText, Event};
use quick_xml::Reader;
use std::fmt;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct Toc {
    data: String,
    creation_time: Option<String>,
    offset: Option<String>,
    size: Option<String>,
}

impl Toc {
    pub fn new() -> Self {
        Toc {
            data: String::new(),
            creation_time: None,
            offset: None,
            size: None,
        }
    }

    pub fn from_read<T: Read>(reader: &mut T, expected: usize) -> Result<Toc> {
        // decompress table of contents
        let mut data = Vec::with_capacity(expected);
        let mut decoder =
            Decoder::new(reader).chain_err(|| "Error decompressing table of contents")?;
        decoder
            .read_to_end(&mut data)
            .chain_err(|| "Error decompressing table of contents")?;
        let data = String::from_utf8(data).chain_err(|| "Error decompressing table of contents")?;

        let mut reader = Reader::from_str(&data);

        let mut toc = Toc::new();
        toc.parse(&mut reader);

        toc.data = data;

        Ok(toc)
    }

    fn parse<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>) {
        let mut buf = Vec::new();

        loop {
            let event = match reader.read_event(&mut buf) {
                Ok(e) => e,
                Err(_) => break,
            };

            match event {
                Event::Start(ref e) => match e.name() {
                    b"creation-time" => self.read_creation_time(reader),
                    b"checksum" => self.read_checksum(reader),
                    _ => {}
                },
                Event::Eof => break,
                _ => {}
            }

            buf.clear();
        }
    }

    fn read_creation_time<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>) {
        let text: Vec<String> =
            self.read_text(reader, |data| String::from_utf8_lossy(data).to_string());
        let text = text.join(" ");
        self.creation_time = Some(text);
    }

    fn read_checksum<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>) {
        let mut buf = Vec::new();
        let mut depth = 1;

        while depth > 0 {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) if e.name() == b"offset" => {
                    self.offset = Some(self.read_text(reader, |data| String::from_utf8_lossy(data).to_string()).join(" "));
                },
                Ok(Event::Start(ref e)) if e.name() == b"size" => {
                    self.size = Some(self.read_text(reader, |data| String::from_utf8_lossy(data).to_string()).join(" "));
                },
                Ok(Event::Start(_)) => depth += 1,
                Ok(Event::End(_)) => depth -= 1,
                Err(_) => break,
                Ok(Event::Eof) => break,
                _ => {},
            }
        }
    }

    fn read_text<T, F: Fn(&[u8]) -> T, B: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<B>,
        handler: F,
    ) -> Vec<T> {
        let mut buf = Vec::new();
        let mut depth = 1;

        let mut text_events: Vec<T> = Vec::new();

        while depth > 0 {
            match reader.read_event(&mut buf) {
                Ok(Event::Text(ref e)) if depth == 1 => {
                    text_events.push(handler(e.escaped()));
                }
                Ok(Event::Start(_)) => depth += 1,
                Ok(Event::End(_)) => depth -= 1,
                Err(_) => break,
                _ => {}
            }
        }

        text_events
    }
}

impl std::fmt::Display for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:25}: {}\n",
            "creation_time",
            self.creation_time.as_ref().unwrap_or(&"None".to_string())
        )?;
        write!(
            f,
            "{:25}: {}\n",
            "offset",
            self.offset.as_ref().unwrap_or(&"None".to_string())
        )?;
        write!(
            f,
            "{:25}: {}\n",
            "size",
            self.size.as_ref().unwrap_or(&"None".to_string())
        )?;
        write!(f, "\n{}", self.data)
    }
}
