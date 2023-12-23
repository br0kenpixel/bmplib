use crate::{helpers::iter_get, types::Header};

const HEADER_LEN: usize = 14;

impl TryFrom<&[u8]> for Header {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != HEADER_LEN {
            return Err(crate::Error::BufLengthMismatch);
        }

        let mut bytes_iter = value.iter().copied();

        let signature = i16::from_ne_bytes(iter_get(&mut bytes_iter));
        let file_size = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let data_offset = u32::from_ne_bytes(iter_get(&mut bytes_iter));

        Ok(Self {
            signature,
            file_size,
            _reserved: 0,
            data_offset,
        })
    }
}

impl Header {
    pub const fn signature(&self) -> i16 {
        self.signature
    }

    pub const fn file_size(&self) -> u32 {
        self.file_size
    }

    pub const fn data_offset(&self) -> u32 {
        self.data_offset
    }
}
