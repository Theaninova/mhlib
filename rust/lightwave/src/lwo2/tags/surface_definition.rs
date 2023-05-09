use crate::binrw_helpers::until_size_limit;
use crate::iff::SubChunk;
use crate::lwo2::vx;
use binrw::{binread, NullString, PosValue};

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct SurfaceDefinition {
    #[br(temp)]
    pub start_pos: PosValue<()>,
    #[br(align_after = 2)]
    pub name: NullString,
    #[br(align_after = 2)]
    pub source: NullString,
    #[br(temp)]
    pub end_pos: PosValue<()>,
    #[br(parse_with = until_size_limit(length as u64 - (end_pos.pos - start_pos.pos)))]
    pub attributes: Vec<SurfaceSubTag>,
}

#[binread]
#[derive(Debug)]
pub enum SurfaceSubTag {
    #[br(magic(b"COLR"))]
    BaseColor(SubChunk<BaseColor>),
    #[br(magic(b"DIFF"))]
    BaseShadingValueDiffuse(SubChunk<BaseShadingValues>),
    #[br(magic(b"LUMI"))]
    BaseShadingValueLuminosity(SubChunk<BaseShadingValues>),
    #[br(magic(b"SPEC"))]
    BaseShadingValueSpecular(SubChunk<BaseShadingValues>),
    #[br(magic(b"REFL"))]
    BaseShadingValueReflectivity(SubChunk<BaseShadingValues>),
    #[br(magic(b"TRAN"))]
    BaseShadingValueTransmission(SubChunk<BaseShadingValues>),
    #[br(magic(b"TRNL"))] // TODO
    BaseShadingValueTrnl(SubChunk<BaseShadingValues>),
    #[br(magic(b"GLOS"))]
    SpecularGlossiness(SubChunk<BaseShadingValues>),
    #[br(magic(b"SHRP"))]
    DiffuseSharpness(SubChunk<BaseShadingValues>),
    #[br(magic(b"BUMP"))]
    BumpIntensity(SubChunk<BaseShadingValues>),
    #[br(magic(b"SIDE"))]
    PolygonSidedness(SubChunk<PolygonSidedness>),
    #[br(magic(b"SMAN"))]
    MaxSmoothingAngle(SubChunk<MaxSmoothingAngle>),
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct BaseColor {
    pub base_color: [f32; 3],
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct BaseShadingValues {
    pub value: f32,
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct PolygonSidedness {
    pub sidedness: u16,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct MaxSmoothingAngle {
    pub max_smoothing_angle: f32,
}
