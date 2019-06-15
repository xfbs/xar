use crate::error::*;
use libflate::zlib::Decoder;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use std::fmt;
use std::io::{Read, BufRead};

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
            match reader.read_event(&mut buf) {
                Err(_) => break,
                Ok(Event::Start(ref e)) if e.name() == b"toc" => {
                    self.parse_toc(reader, e);
                }
                Ok(Event::Eof) => break,
                _ => {}
            }

            buf.clear();
        }
    }

    fn parse_toc<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>, tag: &BytesStart) {
        Self::handle(
            reader,
            |_, _| {},
            |reader, start| match start.name() {
                b"creation-time" => {
                    self.parse_creation_time(reader, start);
                }
                b"checksum" => {
                    println!("got checksum!");
                    Self::ignore(reader);
                }
                b"file" => {
                    println!("got file");
                    Self::ignore(reader);
                }
                _ => {}
            },
        );
    }

    fn parse_creation_time<B: BufRead>(&mut self, reader: &mut Reader<B>, tag: &BytesStart) {
        Self::handle(
            reader,
            |_, text| {
                self.creation_time = Some(String::from_utf8_lossy(text.escaped()).to_string());
            },
            |reader, tag| Self::ignore(reader),
            );
    }

    fn handle<
        B: std::io::BufRead,
        T: FnMut(&mut Reader<B>, &BytesText),
        S: FnMut(&mut Reader<B>, &BytesStart),
    >(
        reader: &mut Reader<B>,
        mut text: T,
        mut start: S,
    ) {
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Text(ref e)) => text(reader, e),
                Ok(Event::Start(ref e)) => start(reader, e),
                Err(_) => break,
                Ok(Event::Eof) => break,
                Ok(Event::End(_)) => break,
                _ => {}
            }
        }
    }

    fn ignore<B: std::io::BufRead>(reader: &mut Reader<B>) {
        let mut buf = Vec::new();
        let mut depth = 1;

        while 0 < depth {
            match reader.read_event(&mut buf) {
                Ok(Event::End(_)) => depth -= 1,
                Ok(Event::Start(_)) => depth += 1,
                Err(_) => break,
                Ok(Event::Eof) => break,
                _ => {}
            }
        }
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
