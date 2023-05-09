use crate::binrw_helpers::until_size_limit;
use crate::iff::SubChunk;
use crate::lwo2::sub_tags::plugin::PluginServerNameAndData;
use crate::lwo2::sub_tags::{EnableState, ValueEnvelope};
use binrw::{binread, NullString, PosValue};

/// Describes an image or a sequence of images. Surface definitions specify images by referring to
/// CLIP chunks. The term "clip" is used to describe these because they can be numbered sequences
/// or animations as well as stills. The index identifies this clip uniquely and may be any non-zero
/// value less than 0x1000000. The filename and any image processing modifiers follow as a variable
/// list of subchunks, which are documented below in the Clip Subchunks section.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct ImageClip {
    pub index: u32,
    #[br(parse_with = until_size_limit(length as u64 - 4))]
    pub attributes: Vec<ImageClipSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum ImageClipSubChunk {
    #[br(magic(b"STIL"))]
    StillImage(SubChunk<StillImage>),
    #[br(magic(b"ISEQ"))]
    ImageSequence(SubChunk<ImageSequence>),
    #[br(magic(b"ANIM"))]
    PluginAnimation(SubChunk<PluginAnimation>),
    #[br(magic(b"XREF"))]
    Reference(SubChunk<Reference>),
    #[br(magic(b"FLAG"))]
    Flag(SubChunk<Flags>),
    #[br(magic(b"STCC"))]
    ColorCyclingStill(SubChunk<ColorCyclingStill>),
    #[br(magic(b"TIME"))]
    Time(SubChunk<Time>),
    #[br(magic(b"CLRS"))]
    ColorSpaceRgb(SubChunk<ColorSpace>),
    #[br(magic(b"CLRA"))]
    ColorSpaceAlpha(SubChunk<ColorSpace>),
    #[br(magic(b"FILT"))]
    ImageFiltering(SubChunk<Flags>),
    #[br(magic(b"DITH"))]
    ImageDithering(SubChunk<Flags>),
    #[br(magic(b"CONT"))]
    Contrast(SubChunk<ValueEnvelope>),
    #[br(magic(b"BRIT"))]
    Brightness(SubChunk<ValueEnvelope>),
    #[br(magic(b"SATR"))]
    Saturation(SubChunk<ValueEnvelope>),
    #[br(magic(b"HUE\0"))]
    Hue(SubChunk<ValueEnvelope>),
    #[br(magic(b"GAMM"))]
    GammaCorrection(SubChunk<ValueEnvelope>),
    #[br(magic(b"NEGA"))]
    Negative(SubChunk<EnableState>),
    #[br(magic(b"IFLT"))]
    PluginImageFilters(SubChunk<PluginServerNameAndData>),
    #[br(magic(b"PFLT"))]
    PluginPixelFilters(SubChunk<PluginServerNameAndData>),
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct PluginAnimation {
    #[br(temp)]
    start_pos: PosValue<()>,
    #[br(align_after = 2)]
    pub file_name: NullString,
    #[br(align_after = 2)]
    pub server_name: NullString,
    pub flags: u16,
    #[br(temp)]
    end_pos: PosValue<()>,
    #[br(count = end_pos.pos - start_pos.pos)]
    pub data: Vec<u8>,
}

/// Contains the color space of the texture. If the flag is 0, then the color space is contained
/// in the following 2 bytes. That color space is defined by the LWCOLORSPACE enum. If the flag
/// is set to 1, then the file name of the color space is save as a local string.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ColorSpace {
    pub flags: u16,
    pub color_space: u16,
    #[br(align_after = 2)]
    pub file_name: NullString,
}

/// A still image with color-cycling is a source defined by a neutral-format name and cycling
/// parameters. lo and hi are indexes into the image's color table. Within this range, the color
/// table entries are shifted over time to cycle the colors in the image. If lo is less than hi,
/// the colors cycle forward, and if hi is less than lo, they go backwards.
///
/// Except for the TIME subchunk, the subchunks after the source subchunk modify the source image
/// and are applied as filters layered on top of the source image.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ColorCyclingStill {
    pub lo: i16,
    pub hi: i16,
    #[br(align_after = 2)]
    pub name: NullString,
}

/// Defines source times for an animated clip.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Time {
    pub start_time: f32,
    pub duration: f32,
    pub frame_rate: f32,
}

/// TODO: What's this?
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Flags {
    pub flag: u32,
}

/// The source is a single still image referenced by a filename in neutral path format.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct StillImage {
    #[br(align_after = 2)]
    pub name: NullString,
}

/// The source is a numbered sequence of still image files. Each filename contains a fixed number
/// of decimal digits that specify a frame number, along with a prefix (the part before the frame
/// number, which includes the path) and a suffix (the part after the number, typically a PC-style
/// extension that identifies the file format). The prefix and suffix are the same for all files
/// in the sequence.
///
/// The flags include bits for looping and interlace. The offset is added to the current frame
/// number to obtain the digits of the filename for the current frame. The start and end values
/// define the range of frames in the sequence.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ImageSequence {
    pub num_digits: u8,
    pub flags: u8,
    pub offset: i16,
    pub reserved: u16,
    pub start: i16,
    pub end: i16,
    #[br(align_after = 2)]
    pub prefix: NullString,
    #[br(align_after = 2)]
    pub suffix: NullString,
}

/// The source is a copy, or instance, of another clip, given by the index. The string is a unique
/// name for this instance of the clip.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Reference {
    pub index: u32,
    #[br(align_after = 2)]
    pub string: NullString,
}
