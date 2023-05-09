use crate::iff::SubChunk;
use crate::lwo2::sub_tags::blocks::Algorithm;
use binrw::binread;

#[binread]
#[derive(Debug)]
pub enum ProceduralTextureSubChunk {
    #[br(magic(b"AXIS"))]
    Axis(SubChunk<Axis>),
    #[br(magic(b"VALU"))]
    BasicValue(SubChunk<BasicValue>),
    #[br(magic(b"FUNC"))]
    AlgorithmAndParameters(SubChunk<Algorithm>),
}

/// Procedurals are often modulations between the current channel value and another value, given
/// here. This may be a scalar or a vector.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct BasicValue {
    #[br(count = length / 4)]
    pub value: Vec<f32>,
}

/// If the procedural has an axis, it may be defined with this chunk using a value of 0, 1 or 2.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Axis {
    pub axis: u16,
}
