use crate::error::*;
use libflate::zlib::Decoder;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::Reader;
use std::fmt;
use std::io::{BufRead, Read};

#[derive(Debug, Clone)]
pub struct Toc {
    data: String,
    creation_time: Option<String>,
    checksum_type: Option<String>,
    checksum_offset: Option<String>,
    checksum_size: Option<String>,
    files: Vec<File>,
}

#[derive(Debug, Clone)]
pub struct File {
    id: Option<String>,
    filetype: Option<FileType>,
    name: Option<String>,
    ctime: Option<String>,
    mtime: Option<String>,
    atime: Option<String>,
    group: Option<String>,
    gid: Option<String>,
    user: Option<String>,
    uid: Option<String>,
    mode: Option<String>,
    inode: Option<String>,
    children: Vec<File>
}

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Directory,
    CharacterSpecial,
}

impl File {
    pub fn new() -> Self {
        File {
            id: None,
            filetype: None,
            name: None,
            ctime: None,
            mtime: None,
            atime: None,
            group: None,
            gid: None,
            user: None,
            uid: None,
            mode: None,
            inode: None,
            children: Vec::new(),
        }
    }
}

impl Toc {
    pub fn new() -> Self {
        Toc {
            data: String::new(),
            creation_time: None,
            checksum_type: None,
            checksum_offset: None,
            checksum_size: None,
            files: Vec::new(),
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

    fn parse_toc<B: std::io::BufRead>(&mut self, reader: &mut Reader<B>, _tag: &BytesStart) {
        Self::handle(
            reader,
            |_, _| {},
            |reader, start| match start.name() {
                b"creation-time" => {
                    self.parse_creation_time(reader, start);
                }
                b"checksum" => {
                    self.parse_checksum(reader, start);
                }
                b"file" => {
                    let file = self.parse_file(reader, start);
                    self.files.push(file);
                }
                _ => Self::ignore(reader),
            },
        );
    }

    fn parse_creation_time<B: BufRead>(&mut self, reader: &mut Reader<B>, _tag: &BytesStart) {
        Self::handle(
            reader,
            |_, text| {
                self.creation_time = Some(String::from_utf8_lossy(text.escaped()).to_string());
            },
            |reader, _| Self::ignore(reader),
        );
    }

    fn parse_checksum<B: BufRead>(&mut self, reader: &mut Reader<B>, tag: &BytesStart) {
        for attr in tag.attributes() {
            if let Ok(attr) = attr {
                match attr.key {
                    b"style" => {
                        self.checksum_type = Some(String::from_utf8_lossy(&attr.value).to_string());
                    },
                    _ => {},
                }
            }
        }

        Self::handle(
            reader,
            |_, _| {},
            |reader, tag| match tag.name() {
                b"offset" => self.parse_checksum_offset(reader, tag),
                b"size" => self.parse_checksum_size(reader, tag),
                _ => Self::ignore(reader),
            });
    }

    fn parse_checksum_offset<B: BufRead>(&mut self, reader: &mut Reader<B>, _tag: &BytesStart) {
        Self::handle(
            reader,
            |_, text| self.checksum_offset = Some(String::from_utf8_lossy(text.escaped()).to_string()),
            |reader, _| Self::ignore(reader),
            );
    }

    fn parse_checksum_size<B: BufRead>(&mut self, reader: &mut Reader<B>, _tag: &BytesStart) {
        Self::handle(
            reader,
            |_, text| self.checksum_size = Some(String::from_utf8_lossy(text.escaped()).to_string()),
            |reader, _| Self::ignore(reader),
            );
    }

    fn parse_file<B: BufRead>(&mut self, reader: &mut Reader<B>, tag: &BytesStart) -> File {
        let mut file = File::new();

        for attr in tag.attributes() {
            if let Ok(attr) = attr {
                match attr.key {
                    b"id" => file.id = Some(String::from_utf8_lossy(&attr.value).to_string()),
                    _ => {},
                }
            }
        }

        Self::handle(
            reader,
            |_, _| {},
            |reader, tag| match tag.name() {
                b"name" => file.name = self.parse_file_name(reader, tag),
                b"type" => file.filetype = self.parse_file_type(reader, tag),
                b"file" => {
                    let f = self.parse_file(reader, tag);
                    file.children.push(f);
                }
                _ => Self::ignore(reader),
            });

        file
    }

    fn parse_file_name<B: BufRead>(&mut self, reader: &mut Reader<B>, _tag: &BytesStart) -> Option<String> {
        let mut name = None;

        Self::handle(
            reader,
            |_, text| name = Some(String::from_utf8_lossy(text.escaped()).to_string()),
            |reader, _| Self::ignore(reader),
            );

        name
    }

    fn parse_file_type<B: BufRead>(&mut self, reader: &mut Reader<B>, _tag: &BytesStart) -> Option<FileType> {
        let mut filetype = None;

        Self::handle(
            reader,
            |_, text| match text.escaped() {
                b"directory" => filetype = Some(FileType::Directory),
                b"character special" => filetype = Some(FileType::CharacterSpecial),
                _ => {},
            },
            |reader, _| Self::ignore(reader),
            );

        filetype
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
            "checksum_type",
            self.checksum_type.as_ref().unwrap_or(&"None".to_string())
        )?;
        write!(
            f,
            "{:25}: {}\n",
            "checksum_offset",
            self.checksum_offset.as_ref().unwrap_or(&"None".to_string())
        )?;
        write!(
            f,
            "{:25}: {}\n",
            "checksum_size",
            self.checksum_size.as_ref().unwrap_or(&"None".to_string())
        )?;

        for file in &self.files {
            write!(f, "{:?}\n", file)?;
        }

        write!(f, "\n{}", self.data)
    }
}
