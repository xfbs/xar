use chrono::NaiveDateTime;
use libflate::zlib::Decoder;
use std::fmt;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use xmltree::Element;
use super::Error;

/// Table of contents.
#[derive(Debug, Clone)]
pub struct Toc {
    data: Element,
}

impl Toc {
    /// Contstruct a toc from a reader pointed at the start of it.
    pub fn from_read<T: Read>(reader: &mut T, _expected: usize) -> Result<Toc, Error> {
        // TODO: check expected toc size.

        let mut decoder = Decoder::new(reader).map_err(Error::DecompressionFailed)?;
        let element = Element::parse(&mut decoder).map_err(Error::RootElementInvalid)?;

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
        let text = time.get_text().ok_or(Error::CreationTimeMissing)?;
        let parsed = NaiveDateTime::parse_from_str(&text, "%Y-%m-%dT%H:%M:%S").map_err(Error::CreationTimeParseFailed)?;
        Ok(parsed)
    }

    fn creation_time_element(&self) -> Result<&Element, Error> {
        self.toc_element()?
            .get_child("creation-time")
            .ok_or(Error::CreationTimeMissing)
    }

    /// Get what type of checksum was used for the Toc.
    pub fn checksum_type(&self) -> Result<&String, Error> {
        self.checksum_element()?
            .attributes
            .get("style")
            .ok_or(Error::ChecksumTypeMissing)
    }

    /// Find out at which offset the checksum is.
    pub fn checksum_offset(&self) -> Result<usize, Error> {
        self
            .checksum_element()?
            .get_child("offset")
            .ok_or(Error::ChecksumOffsetInvalid)?
            .get_text()
            .ok_or(Error::ChecksumOffsetInvalid)?
            .parse::<usize>()
            .map_err(Error::ChecksumOffsetParseFailed)
    }

    /// Find out how many bytes the checksum is.
    pub fn checksum_size(&self) -> Result<usize, Error> {
        self
            .checksum_element()?
            .get_child("size")
            .ok_or(Error::ChecksumSizeInvalid)?
            .get_text()
            .ok_or(Error::ChecksumSizeInvalid)?
            .parse::<usize>()
            .map_err(Error::ChecksumSizeParseFailed)
    }

    fn checksum_element(&self) -> Result<&Element, Error> {
        self.toc_element()?
            .get_child("checksum")
            .ok_or(Error::ChecksumElementMissing)
    }

    fn toc_element(&self) -> Result<&Element, Error> {
        self.data.get_child("toc").ok_or(Error::TocElementMissing)
    }

    pub fn files(&self) -> Result<Files, Error> {
        Ok(Files {
            data: self.toc_element()?,
            path: PathBuf::new(),
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

    pub fn from_name(name: &str) -> Option<FileElement> {
        use FileElement::*;
        match name {
            "data" => Some(Data),
            "ctime" => Some(CTime),
            "mtime" => Some(MTime),
            "atime" => Some(ATime),
            "group" => Some(Group),
            "gid" => Some(GID),
            "user" => Some(User),
            "uid" => Some(UID),
            "mode" => Some(Mode),
            "inode" => Some(INode),
            "type" => Some(Type),
            "name" => Some(Name),
            "deviceno" => Some(DeviceNo),
            _ => None,
        }
    }

    pub fn error(&self) -> Error {
        match self {
            _ => Error::FileTypeElementMissing,
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

    pub fn error(&self) -> Error {
        match self {
            _ => Error::FileTypeElementMissing,
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
    pub name: Option<String>,
    pub id: Option<usize>,
    pub ftype: Option<FileType>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub uid: Option<usize>,
    pub gid: Option<usize>,
    pub deviceno: Option<usize>,
    pub inode: Option<usize>,
}

impl FileAttr {
    pub fn new() -> Self {
        FileAttr {
            name: None,
            id: None,
            ftype: None,
            user: None,
            group: None,
            uid: None,
            gid: None,
            deviceno: None,
            inode: None,
        }
    }

    pub fn parse(data: &Element) -> FileAttr {
        let mut attrs = FileAttr::new();

        for child in &data.children {
            if let Some(child_element) = child.as_element() {
              let _ = attrs.parse_child(child_element);
            }
        }

        attrs
    }

    fn parse_child(&mut self, child: &Element) -> Result<(), Error> {
        let e = FileElement::from_name(&child.name).ok_or(Error::TocElementMissing)?;

        use FileElement::*;
        match e {
            Group => Self::parse_text(child, &mut self.group),
            User => Self::parse_text(child, &mut self.user),
            Name => Self::parse_text(child, &mut self.name),
            Type => Self::parse_type(child, &mut self.ftype),
            Data => self.parse_dummy(),
            CTime => self.parse_dummy(),
            MTime => self.parse_dummy(),
            ATime => self.parse_dummy(),
            GID => Self::parse_usize(e, child, &mut self.gid),
            UID => Self::parse_usize(e, child, &mut self.uid),
            Mode => self.parse_dummy(),
            INode => Self::parse_usize(e, child, &mut self.inode),
            DeviceNo => Self::parse_usize(e, child, &mut self.deviceno),
        }
    }

    fn parse_text(
        child: &Element,
        text: &mut Option<String>,
    ) -> Result<(), Error> {
        *text = child.get_text().map(|s|s.into_owned());
        Ok(())
    }

    fn parse_type(
        child: &Element,
        ftype: &mut Option<FileType>,
    ) -> Result<(), Error> {
        if let Some(text) = &child.get_text() {
            if let Some(nftype) = FileType::from_str(text) {
                *ftype = Some(nftype);
            }
        }

        Ok(())
    }

    fn parse_usize(
        element: FileElement,
        child: &Element,
        out: &mut Option<usize>,
    ) -> Result<(), Error> {
        let amt = child
            .get_text()
            .ok_or(element.error())?
            .parse::<usize>()
            .or(Err(element.error()))?;
        *out = Some(amt);
        Ok(())
    }

    fn parse_dummy(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

/// File object.
#[derive(Debug, Clone)]
pub struct File<'a, 'b> {
    data: &'a Element,
    pub path: &'b Path,
}

impl<'a, 'b> File<'a, 'b> {
    pub fn new(element: &'a Element, path: &'b Path) -> File<'a, 'b> {
        File {
            data: element,
            path: path,
        }
    }

    pub fn files(&self) -> Files<'a> {
        let mut path = self.path.to_path_buf();
        let attrs = self.attrs();
        // TODO: what if no name?
        if let Some(name) = attrs.name {
            path.push(name)
        }

        Files {
            data: self.data,
            path: path,
        }
    }

    pub fn attrs(&self) -> FileAttr {
        FileAttr::parse(&self.data)
    }
}

/// Iterator over the files (in the current level).
#[derive(Debug, Clone)]
pub struct Files<'a> {
    data: &'a Element,
    path: PathBuf,
}

impl<'a> Files<'a> {
    pub fn iter(&self) -> FilesIter {
        FilesIter {
            data: self.data,
            path: &self.path,
            pos: 0,
        }
    }

    pub fn find(&self, path: &Path) -> Option<Files> {
        // TODO: Implement this
        Some(self.clone())
    }
}

/// Iterator over the files
#[derive(Debug, Clone)]
pub struct FilesIter<'a, 'b> {
    data: &'a Element,
    path: &'b Path,
    pos: usize,
}

impl<'a, 'b> Iterator for FilesIter<'a, 'b> {
    type Item = File<'a, 'b>;
    fn next(&mut self) -> Option<Self::Item> {
        for (i, child) in self.data.children.iter().enumerate().skip(self.pos) {
            if let Some(child_element) = child.as_element() {
                if child_element.name == "file" {
                    self.pos = i + 1;
                    return Some(File {
                        data: child_element,
                        path: self.path,
                    });
                }
            }
        }
        None
    }
}
