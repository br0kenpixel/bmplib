#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Header {
    pub(crate) signature: i16,
    pub(crate) file_size: u32,
    pub(crate) _reserved: u32,
    pub(crate) data_offset: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InfoHeader {
    pub(crate) size: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) planes: u16,
    pub(crate) bit_count: BitCount,
    pub(crate) compression: Compression,
    pub(crate) image_size: u32,
    pub(crate) x_pixels_per_m: u32,
    pub(crate) y_pixels_per_m: u32,
    pub(crate) colors_used: u32,
    pub(crate) important_colors: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum BitCount {
    Monochrome = 1,
    FourBit = 4,
    EightBit = 8,
    SixteenBit = 16,
    TwentyfourBit = 24,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Compression {
    None = 0,
    BiRle8 = 1,
    BiRle4 = 2,
}
