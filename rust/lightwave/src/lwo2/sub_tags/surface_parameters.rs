use crate::iff::SubChunk;
use crate::lwo2::sub_tags::blocks::SurfaceBlocks;
use crate::lwo2::sub_tags::{ValueEnvelope, VectorEnvelope, VxReference};
use crate::lwo2::vx;
use binrw::{binread, NullString};

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
    #[br(magic(b"RFOP"))]
    ReflectionOptions(SubChunk<ReflectionOptions>),
    #[br(magic(b"RIMG"))]
    ReflectionMapImage(SubChunk<VxReference>),
    #[br(magic(b"RSAN"))]
    ReflectionMapSeamAngle(SubChunk<VectorEnvelope>),
    #[br(magic(b"RBLR"))]
    ReflectionBlurring(SubChunk<ValueEnvelope>),
    #[br(magic(b"RIND"))]
    RefractiveIndex(SubChunk<ValueEnvelope>),
    #[br(magic(b"TROP"))]
    TransparencyOptions(SubChunk<ReflectionOptions>),
    #[br(magic(b"TIMG"))]
    RefractionMapImage(SubChunk<VxReference>),
    #[br(magic(b"TBLR"))]
    RefractionBlurring(SubChunk<ValueEnvelope>),
    #[br(magic(b"CLRH"))]
    ColorHighlights(SubChunk<ValueEnvelope>),
    #[br(magic(b"CLRF"))]
    ColorFilter(SubChunk<ValueEnvelope>),
    #[br(magic(b"ADTR"))]
    AdditiveTransparency(SubChunk<ValueEnvelope>),
    #[br(magic(b"GLOW"))]
    GlowEffect(SubChunk<GlowEffect>),
    #[br(magic(b"LINE"))]
    RenderOutlines(SubChunk<RenderOutlines>),
    #[br(magic(b"ALPH"))]
    AlphaMode(SubChunk<AlphaMode>),
    #[br(magic(b"VCOL"))]
    VertexColorMap(SubChunk<VertexColorMap>),
    #[br(magic(b"BLOK"))]
    Blocks(SubChunk<SurfaceBlocks>),
}

/// The vertex color map subchunk identifies an RGB or RGBA VMAP that will be used to color the surface.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VertexColorMap {
    pub intensity: f32,
    #[br(parse_with = vx)]
    pub envelope: u32,
    pub vmap_type: [u8; 4],
    #[br(align_after = 2)]
    pub name: NullString,
}

/// The alpha mode defines the alpha channel output options for the surface.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct AlphaMode {
    pub mode: AlphaModeMode,
    pub value: f32,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum AlphaModeMode {
    /// The surface has no effect on the alpha channel when rendered.
    UnaffectedBySurface = 0,
    /// The alpha channel will be written with the constant value following the mode in the subchunk.
    ConstantValue = 1,
    /// The alpha value is derived from surface opacity, which is the default if the ALPH chunk is missing.
    SurfaceOpacity = 2,
    /// The alpha value comes from the shadow density.
    ShadowDensity = 3,
}

/// The line effect draws the surface as a wireframe of the polygon edges. Currently the only flag
/// defined is an enable switch in the low bit. The size is the thickness of the lines in pixels,
/// and the color, if not given, is the base color of the surface. Note that you may encounter
/// LINE subchunks with no color information (these will have a subchunk length of 8 bytes) and
/// possibly without size information (subchunk length 2).
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct RenderOutlines {
    pub flags: u16,
    #[br(if(length > 2))]
    pub size: f32,
    #[br(if(length > 2))]
    #[br(parse_with = vx)]
    pub size_envelope: u32,
    #[br(if(length > 8))]
    pub color: [f32; 3],
    #[br(if(length > 8))]
    #[br(parse_with = vx)]
    pub color_envelope: u32,
}

/// The glow effect causes a surface to spread and affect neighboring areas of the image. The type
/// can be 0 for Hastings glow, and 1 for image convolution. The size and intensity define how large
/// and how strong the effect is.
///
/// You may also encounter glow information written in a GVAL subchunk containing only the intensity
/// and its envelope (the subchunk length is 6).
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct GlowEffect {
    pub kind: GlowType,
    pub intensity: f32,
    #[br(parse_with = vx)]
    pub intensity_envelope: u32,
    #[br(if(length > 6))]
    pub size: f32,
    #[br(if(length > 6))]
    #[br(parse_with = vx)]
    pub size_envelope: u32,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum GlowType {
    HastingsGlow = 0,
    ImageConvolution = 1,
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
