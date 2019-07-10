use chrono::NaiveDateTime;
use failure::*;
use libflate::zlib::Decoder;
use std::fmt;
use std::io::{Read, Write};
use xmltree::Element;

#[derive(Fail, Debug)]
pub enum Errors {
    #[fail(display = "<toc> element doesn't exist in Toc.")]
    NoTocElement,
    #[fail(display = "<creation-time> element doesn't exist in Toc.")]
    NoCreationTime,
    #[fail(display = "<checksum> element missing.")]
    NoChecksumElement,
    #[fail(display = "style attribute in <checksum> element missing.")]
    NoChecksumType,
    #[fail(display = "style attribute in <checksum> element missing.")]
    NoFileTypeElement,
    #[fail(display = "style attribute in <checksum> element missing.")]
    NoFileNameElement,
    #[fail(display = "style attribute in <checksum> element missing.")]
    NoFileId,
    #[fail(display = "style attribute in <checksum> element missing.")]
    ChecksumOffsetInvalid,
}

/// Table of contents.
#[derive(Debug, Clone)]
pub struct Toc {
    data: Element,
}

impl Toc {
    /// Contstruct a toc from a reader pointed at the start of it.
    pub fn from_read<T: Read>(reader: &mut T, _expected: usize) -> Result<Toc, Error> {
        // TODO: check expected toc size.

        let mut decoder = Decoder::new(reader)?;
        let element = Element::parse(&mut decoder)?;

        Ok(Toc { data: element })
    }

    pub fn data(&self) -> &Element {
        &self.data
    }

    /// Print the toc as XML to writer.
    pub fn write<W: Write>(&self, writer: W) -> Result<(), xmltree::Error> {
        self.data.write(writer)
    }

    /// Compute creation time of Toc.
    pub fn creation_time(&self) -> Result<NaiveDateTime, Error> {
        let time = self.creation_time_element()?;
        let text = time.text.as_ref().ok_or(Errors::NoCreationTime)?;
        let parsed = NaiveDateTime::parse_from_str(&text, "%Y-%m-%dT%H:%M:%S")?;
        Ok(parsed)
    }

    fn creation_time_element(&self) -> Result<&Element, Errors> {
        self.toc_element()?
            .get_child("creation-time")
            .ok_or(Errors::NoCreationTime)
    }

    /// Get what type of checksum was used for the Toc.
    pub fn checksum_type(&self) -> Result<&String, Errors> {
        self.checksum_element()?
            .attributes
            .get("style")
            .ok_or(Errors::NoChecksumType)
    }

    /// Find out at which offset the checksum is.
    pub fn checksum_offset(&self) -> Result<usize, Error> {
        let re = self
            .checksum_element()?
            .get_child("offset")
            .ok_or(Errors::ChecksumOffsetInvalid)?
            .text
            .as_ref()
            .ok_or(Errors::ChecksumOffsetInvalid)?
            .parse::<usize>()?;
        Ok(re)
    }

    /// Find out how many bytes the checksum is.
    pub fn checksum_size(&self) -> Result<usize, Error> {
        let re = self
            .checksum_element()?
            .get_child("size")
            .ok_or(Errors::ChecksumOffsetInvalid)?
            .text
            .as_ref()
            .ok_or(Errors::ChecksumOffsetInvalid)?
            .parse::<usize>()?;
        Ok(re)
    }

    fn checksum_element(&self) -> Result<&Element, Errors> {
        self.toc_element()?
            .get_child("checksum")
            .ok_or(Errors::NoChecksumElement)
    }

    fn toc_element(&self) -> Result<&Element, Errors> {
        self.data.get_child("toc").ok_or(Errors::NoTocElement)
    }

    pub fn files(&self) -> Result<Files, Errors> {
        Ok(Files {
            data: self.toc_element()?,
        })
    }
}

impl std::fmt::Display for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "creation-time {:?}\n", self.creation_time())?;
        write!(f, "checksum-kind {:?}\n", self.checksum_type())?;
        write!(f, "checksum-offset {:?}\n", self.checksum_offset())?;
        write!(f, "checksum-size {:?}\n", self.checksum_size())
    }
}

#[derive(Clone, Debug, Copy)]
pub enum FileElement {
    Data,
    CTime,
    MTime,
    ATime,
    Group,
    GID,
    User,
    UID,
    Mode,
    INode,
    Type,
    Name,
    DeviceNo,
}

impl FileElement {
    pub fn name(&self) -> &'static str {
        use FileElement::*;
        match self {
            Data => "data",
            CTime => "ctime",
            MTime => "mtime",
            ATime => "atime",
            Group => "group",
            GID => "gid",
            User => "user",
            UID => "uid",
            Mode => "mode",
            INode => "inode",
            Type => "type",
            Name => "name",
            DeviceNo => "deviceno",
        }
    }

    pub fn error(&self) -> Errors {
        use FileElement::*;
        match self {
            _ => Errors::NoFileTypeElement,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FileDataElement {
    Length,
    Offset,
    Size,
    Encoding,
    ExtractedChecksum,
    ArchivedChecksum,
}

impl FileDataElement {
    pub fn name(&self) -> &'static str {
        use FileDataElement::*;
        match self {
            Length => "length",
            Offset => "offset",
            Size => "size",
            Encoding => "encoding",
            ExtractedChecksum => "extracted-checksum",
            ArchivedChecksum => "archived-checksum",
        }
    }

    pub fn error(&self) -> Errors {
        use FileElement::*;
        match self {
            _ => Errors::NoFileTypeElement,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    File,
    Directory,
    CharacterSpecial,
}

impl FileType {
    pub fn from_str(name: &str) -> Option<FileType> {
        use FileType::*;
        match name {
            "file" => Some(File),
            "directory" => Some(Directory),
            "character special" => Some(CharacterSpecial),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileAttr {
    name: Option<String>,
    id: Option<usize>,
    ftype: Option<FileType>,
    user: Option<String>,
    group: Option<String>,
    uid: Option<usize>,
    gid: Option<usize>,
    deviceno: Option<usize>,
    inode: Option<usize>,
}

/// File object.
#[derive(Debug, Clone)]
pub struct File<'a> {
    data: &'a Element,
}

impl<'a> File<'a> {
    pub fn new(element: &'a Element) -> File {
        File { data: element }
    }

    pub fn files(&self) -> Files {
        Files { data: self.data }
    }

    pub fn ftype(&self) -> Result<FileType, Errors> {
        let text = self
            .element(FileElement::Type)?
            .text
            .as_ref()
            .ok_or(Errors::NoFileTypeElement)?;

        FileType::from_str(text.as_str()).ok_or(Errors::NoFileTypeElement)
    }

    pub fn id(&self) -> Result<usize, Error> {
        Ok(self
            .data
            .attributes
            .get("id")
            .ok_or(Errors::NoFileId)?
            .parse::<usize>()?)
    }

    pub fn name(&self) -> Result<&String, Errors> {
        self.element_text(FileElement::Name)
    }

    pub fn user(&self) -> Result<&String, Errors> {
        self.element_text(FileElement::User)
    }

    pub fn group(&self) -> Result<&String, Errors> {
        self.element_text(FileElement::Group)
    }

    pub fn uid(&self) -> Result<usize, Error> {
        self.element_text_usize(FileElement::UID)
    }

    pub fn gid(&self) -> Result<usize, Error> {
        self.element_text_usize(FileElement::GID)
    }

    pub fn deviceno(&self) -> Result<usize, Error> {
        self.element_text_usize(FileElement::DeviceNo)
    }

    pub fn inode(&self) -> Result<usize, Error> {
        self.element_text_usize(FileElement::INode)
    }

    pub fn length(&self) -> Result<usize, Error> {
        self.data_element_text_usize(FileDataElement::Length)
    }

    pub fn offset(&self) -> Result<usize, Error> {
        self.data_element_text_usize(FileDataElement::Offset)
    }

    pub fn size(&self) -> Result<usize, Error> {
        self.data_element_text_usize(FileDataElement::Size)
    }

    fn element(&self, element: FileElement) -> Result<&Element, Errors> {
        self.data.get_child(element.name()).ok_or(element.error())
    }

    fn data_element(&self, element: FileDataElement) -> Result<&Element, Errors> {
        self.element(FileElement::Data)?
            .get_child(element.name()).ok_or(element.error())
    }

    fn element_text(&self, element: FileElement) -> Result<&String, Errors> {
        Ok(self.element(element)?
           .text
           .as_ref()
           .ok_or(element.error())?)
    }

    fn data_element_text(&self, element: FileDataElement) -> Result<&String, Errors> {
        Ok(self.data_element(element)?
           .text
           .as_ref()
           .ok_or(element.error())?)
    }

    fn element_text_usize(&self, element: FileElement) -> Result<usize, Error> {
        let ret = self.element_text(element)?
            .parse::<usize>()?;
        Ok(ret)
    }

    fn data_element_text_usize(&self, element: FileDataElement) -> Result<usize, Error> {
        let ret = self.data_element_text(element)?
            .parse::<usize>()?;
        Ok(ret)
    }
}

/// Iterator over the files (in the current level).
#[derive(Debug, Clone)]
pub struct Files<'a> {
    data: &'a Element,
}

impl<'a> Files<'a> {
    pub fn iter(&self) -> FilesIter {
        FilesIter {
            data: self.data,
            pos: 0,
        }
    }
}

/// Iterator over the files
#[derive(Debug, Clone)]
pub struct FilesIter<'a> {
    data: &'a Element,
    pos: usize,
}

impl<'a> Iterator for FilesIter<'a> {
    type Item = File<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        for (i, child) in self.data.children.iter().enumerate().skip(self.pos) {
            if child.name == "file" {
                self.pos = i + 1;
                return Some(File { data: child });
            }
        }
        None
    }
}
