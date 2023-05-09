use crate::lwo2::vx;
use binrw::{binread, NullString};

pub mod blocks;
pub mod plugin;
pub mod surface_parameters;

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
pub struct Name {
    #[br(align_after = 2)]
    pub name: NullString,
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
