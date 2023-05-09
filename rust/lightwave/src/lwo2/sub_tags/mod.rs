use crate::lwo2::sub_tags::envelope_type::EnvelopeType;
use crate::lwo2::vx;
use binrw::binread;

pub mod envelope_type;
pub mod surface_block_image_texture;
pub mod surface_blocks;
pub mod surface_parameters;
pub mod texture_mapping;

#[binread]
#[derive(Debug)]
pub enum SubTag {
    #[br(magic(b"TYPE"))]
    EnvelopeType(EnvelopeType),
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VectorEnvelope {
    pub base_color: [f32; 3],
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ValueEnvelope {
    pub value: f32,
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VxReference {
    #[br(parse_with = vx)]
    pub texture_image: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct EnableState {
    pub enable: u16,
}
