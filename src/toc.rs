use libflate::zlib::Decoder;
use std::fmt;
use std::io::{BufRead, Read};
use failure::*;

#[derive(Debug, Clone)]
pub struct Toc {
}

impl Toc {
    pub fn from_read<T: Read>(reader: &mut T, expected: usize) -> Result<Toc, Error> {
        Ok(Toc {})
    }
}

impl std::fmt::Display for Toc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")
    }
}
