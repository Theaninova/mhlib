use crate::iff::SubChunk;
use crate::lwo2::sub_tags::Name;
use binrw::binread;

#[binread]
#[derive(Debug)]
pub enum GradientTextureSubChunk {
    #[br(magic(b"PNAM"))]
    ParameterName(SubChunk<Name>),
    #[br(magic(b"INAM"))]
    ItemName(SubChunk<Name>),
    #[br(magic(b"GRST"))]
    GradientRangeStart(SubChunk<GradientRange>),
    #[br(magic(b"GREN"))]
    GradientRangeEnd(SubChunk<GradientRange>),
    #[br(magic(b"GRPT"))]
    RepeatMode(SubChunk<RepeatMode>),
    #[br(magic(b"FKEY"))]
    KeyValues(SubChunk<KeyValues>),
    #[br(magic(b"IKEY"))]
    KeyParameters(SubChunk<KeyParameters>),
}

/// The repeat mode. This is currently undefined.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct KeyParameters {
    #[br(count = length / 2)]
    pub repeat_mode: Vec<u16>,
}

/// The transfer function is defined by an array of keys, each with an input value and an RGBA
/// output vector. Given an input value, the gradient can be evaluated by selecting the keys whose
/// positions bracket the value and interpolating between their outputs. If the input value is lower
/// than the first key or higher than the last key, the gradient value is the value of the closest
/// key.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct KeyValues {
    #[br(count = length / 18)]
    pub key_values: Vec<KeyValue>,
}

#[binread]
#[derive(Debug)]
pub struct KeyValue {
    pub input: f32,
    pub output: [f32; 4],
}

/// The start and end of the input range. These values only affect the display of the gradient
/// in the user interface. They don't affect rendering.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct GradientRange {
    pub name: f32,
}

/// The repeat mode. This is currently undefined.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct RepeatMode {
    pub repeat_mode: u16,
}
