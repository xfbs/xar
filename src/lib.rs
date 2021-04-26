#[cfg(test)]
mod tests;

pub mod archive;
pub mod header;
pub mod toc;
pub use archive::Archive;
pub use header::Header;
pub use toc::Toc;

use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("<checksum> element missing.")]
    ChecksumElementMissing,
    #[error("Checksum offset missing.")]
    ChecksumOffsetInvalid,
    #[error("Parsing of checksum offset failed: {0}.")]
    ChecksumOffsetParseFailed(ParseIntError),
    #[error("Checksum size missing.")]
    ChecksumSizeInvalid,
    #[error("style attribute in <checksum> element missing.")]
    ChecksumTypeMissing,
    #[error("Parsing of checksum size failed: {0}.")]
    ChecksumSizeParseFailed(ParseIntError),
    #[error("Creation time parse failed: {0}.")]
    CreationTimeParseFailed(chrono::ParseError),
    #[error("<creation-time> element doesn't exist in Toc.")]
    CreationTimeMissing,
    #[error("Decompression failed: {0}.")]
    DecompressionFailed(std::io::Error),
    #[error("style attribute in <checksum> element missing.")]
    FileTypeElementMissing,
    #[error("style attribute in <checksum> element missing.")]
    FileNameElementMissing,
    #[error("style attribute in <checksum> element missing.")]
    FileIdMissing,
    #[error("Header read error: {0}.")]
    HeaderReadError(std::io::Error),
    #[error("Header too small: {0} bytes, expected 28.")]
    HeaderTooSmall(u16),
    #[error("Wrong magic number.")]
    MagicError,
    #[error("Root element invalid: {0}.")]
    RootElementInvalid(xmltree::ParseError),
    #[error("<toc> element doesn't exist in Toc.")]
    TocElementMissing,
    #[error("Wrong version: {0}, expected 1.")]
    Version(u16),
}
