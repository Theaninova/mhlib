use binrw::{binread, NullString};

/// Store an object description. Optional. This should be a simple line of upper and lowercase
/// characters, punctuation and spaces which describes the contents of the object file. There
/// should be no control characters in this text string and it should generally be kept short.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct DescriptionLine {
    #[br(align_after = 2)]
    pub description_line: NullString,
}

/// An iconic or thumbnail image for the object which can be used when viewing the file in a
/// browser. Currently the only suported encoding is 0, meaning uncompressed RGB byte triples.
/// The width is the number of pixels in each row of the image, and the height (number of rows)
/// is (chunkSize - 4)/width. This chunk is optional.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct ThumbnailIconImage {
    pub encoding: ThumbnailImageEncoding,
    pub width: u16,
    #[br(calc = (length as u16 - 4) / width)]
    pub height: u16,
    #[br(count = length - 4)]
    pub data: Vec<u8>,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum ThumbnailImageEncoding {
    UncompressedRgb = 0,
}
