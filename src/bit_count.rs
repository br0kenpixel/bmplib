use crate::types::BitCount;
use std::fmt::Display;

impl TryFrom<[u8; 2]> for BitCount {
    type Error = crate::Error;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        Self::try_from(u16::from_ne_bytes(value))
    }
}

impl TryFrom<u16> for BitCount {
    type Error = crate::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Monochrome),
            4 => Ok(Self::FourBit),
            8 => Ok(Self::EightBit),
            16 => Ok(Self::SixteenBit),
            24 => Ok(Self::TwentyfourBit),
            _ => Err(crate::Error::InvalidBitCount),
        }
    }
}

impl Display for BitCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Monochrome => write!(f, "Monochrome"),
            Self::FourBit => write!(f, "4-bit"),
            Self::EightBit => write!(f, "8-bit"),
            Self::SixteenBit => write!(f, "16-bit"),
            Self::TwentyfourBit => write!(f, "24-bit"),
        }
    }
}
