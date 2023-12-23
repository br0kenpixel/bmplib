use std::{fs, path::Path};
use types::{Header, InfoHeader};

mod bit_count;
mod compression;
mod error;
mod header;
pub(crate) mod helpers;
mod info_header;
mod types;

pub use error::Error;

#[derive(Debug)]
pub struct BitmapImage {
    header: Header,
    info: InfoHeader,
}

impl BitmapImage {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = fs::read(path)?;

        let header = &bytes[..14];
        let info_header = &bytes[14..(14 + 40)];

        debug_assert!(header.len() == 14);
        debug_assert!(info_header.len() == 40);

        let header = Header::try_from(header)?;
        let info_header = InfoHeader::try_from(info_header)?;

        Ok(Self {
            header,
            info: info_header,
        })
    }

    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    #[must_use]
    pub const fn info_header(&self) -> &InfoHeader {
        &self.info
    }
}
