use crate::types::Compression;
use std::fmt::Display;

impl TryFrom<[u8; 4]> for Compression {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Self::try_from(u32::from_ne_bytes(value))
    }
}

impl TryFrom<u32> for Compression {
    type Error = crate::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::BiRle8),
            4 => Ok(Self::BiRle4),
            _ => Err(crate::Error::InvalidCompression),
        }
    }
}

impl Display for Compression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Uncompressed"),
            Self::BiRle8 => write!(f, "8-bit RLE Encoding"),
            Self::BiRle4 => write!(f, "4-bit RLE Encoding"),
        }
    }
}
