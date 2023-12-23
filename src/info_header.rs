use crate::{
    helpers::iter_get,
    types::{BitCount, Compression, InfoHeader},
};

const INFO_HEADER_LEN: usize = 40;

impl TryFrom<&[u8]> for InfoHeader {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != INFO_HEADER_LEN {
            return Err(crate::Error::BufLengthMismatch);
        }

        let mut bytes_iter = value.iter().copied();

        let size = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let width = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let height = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let planes = u16::from_ne_bytes(iter_get(&mut bytes_iter));
        let bit_count = BitCount::try_from(iter_get(&mut bytes_iter))?;
        let compression = Compression::try_from(iter_get(&mut bytes_iter))?;
        let image_size = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let x_pixels_per_m = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let y_pixels_per_m = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let colors_used = u32::from_ne_bytes(iter_get(&mut bytes_iter));
        let important_colors = u32::from_ne_bytes(iter_get(&mut bytes_iter));

        Ok(Self {
            size,
            width,
            height,
            planes,
            bit_count,
            compression,
            image_size,
            x_pixels_per_m,
            y_pixels_per_m,
            colors_used,
            important_colors,
        })
    }
}

impl InfoHeader {
    pub const fn size(&self) -> u32 {
        self.size
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub const fn planes(&self) -> u16 {
        self.planes
    }

    pub const fn bit_count(&self) -> BitCount {
        self.bit_count
    }

    pub const fn compression(&self) -> Compression {
        self.compression
    }

    pub const fn x_pixels_per_m(&self) -> u32 {
        self.x_pixels_per_m
    }

    pub const fn y_pixels_per_m(&self) -> u32 {
        self.y_pixels_per_m
    }

    pub const fn colors_used(&self) -> u32 {
        self.colors_used
    }

    pub const fn important_colors(&self) -> u32 {
        self.important_colors
    }
}
