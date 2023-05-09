use crate::iff::SubChunk;
use crate::lwo2::sub_tags::surface_blocks::SurfaceBlocks;
use crate::lwo2::sub_tags::{ValueEnvelope, VectorEnvelope, VxReference};
use binrw::binread;

#[binread]
#[derive(Debug)]
pub enum SurfaceParameterSubChunk {
    #[br(magic(b"COLR"))]
    BaseColor(SubChunk<VectorEnvelope>),
    #[br(magic(b"DIFF"))]
    BaseShadingValueDiffuse(SubChunk<ValueEnvelope>),
    #[br(magic(b"LUMI"))]
    BaseShadingValueLuminosity(SubChunk<ValueEnvelope>),
    #[br(magic(b"SPEC"))]
    BaseShadingValueSpecular(SubChunk<ValueEnvelope>),
    #[br(magic(b"REFL"))]
    BaseShadingValueReflectivity(SubChunk<ValueEnvelope>),
    #[br(magic(b"TRAN"))]
    BaseShadingValueTransparency(SubChunk<ValueEnvelope>),
    #[br(magic(b"TRNL"))]
    BaseShadingValueTranslucency(SubChunk<ValueEnvelope>),
    #[br(magic(b"GLOS"))]
    SpecularGlossiness(SubChunk<ValueEnvelope>),
    #[br(magic(b"SHRP"))]
    DiffuseSharpness(SubChunk<ValueEnvelope>),
    #[br(magic(b"BUMP"))]
    BumpIntensity(SubChunk<ValueEnvelope>),
    #[br(magic(b"SIDE"))]
    PolygonSidedness(SubChunk<PolygonSidedness>),
    #[br(magic(b"SMAN"))]
    MaxSmoothingAngle(SubChunk<MaxSmoothingAngle>),
    #[br(magic(b"BLOK"))]
    Blocks(SubChunk<SurfaceBlocks>),
    #[br(magic(b"RFOP"))]
    ReflectionOptions(SubChunk<ReflectionOptions>),
    #[br(magic(b"RIMG"))]
    ReflectionMapImage(SubChunk<VxReference>),
    #[br(magic(b"TBLR"))]
    RefractionBlurring(SubChunk<ValueEnvelope>),
    #[br(magic(b"CLRH"))]
    ColorHighlights(SubChunk<ValueEnvelope>),
    #[br(magic(b"CLRF"))]
    ColorFilter(SubChunk<ValueEnvelope>),
    #[br(magic(b"ADTR"))]
    AdditiveTransparency(SubChunk<ValueEnvelope>),
}

#[binread]
#[br(repr = u16, import(_length: u32))]
#[derive(Debug)]
pub enum ReflectionOptions {
    BackdropOnly = 0,
    RaytracingAndBackdrop = 1,
    SphericalMap = 2,
    RaytracingAndSphericalMap = 3,
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
