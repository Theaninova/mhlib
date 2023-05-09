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
pub struct PolygonSidedness {
    pub sidedness: u16,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct MaxSmoothingAngle {
    pub max_smoothing_angle: f32,
}

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct SurfaceBlocks {
    #[br(temp)]
    start_pos: PosValue<()>,
    pub header: SurfaceBlockHeader,
    #[br(temp)]
    end_pos: PosValue<()>,
    #[br(parse_with = until_size_limit(length as u64 - (end_pos.pos - start_pos.pos)))]
    pub attributes: Vec<SurfaceBlockSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum SurfaceBlockHeader {
    #[br(magic(b"IMAP"))]
    ImageMapTexture(SubChunk<SurfaceBlockHeaders>),
    #[br(magic(b"PROC"))]
    ProceduralTexture(SubChunk<SurfaceBlockHeaders>),
    #[br(magic(b"GRAD"))]
    GradientTexture(SubChunk<SurfaceBlockHeaders>),
    #[br(magic(b"SHDR"))]
    ShaderPlugin(SubChunk<SurfaceBlockHeaders>),
}

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct SurfaceBlockHeaders {
    #[br(pad_before = 2, parse_with = until_size_limit(length as u64))]
    pub block_attributes: Vec<SurfaceBlockSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum SurfaceBlockSubChunk {
    #[br(magic(b"CHAN"))]
    Channel(SubChunk<Channel>),
    #[br(magic(b"ENAB"))]
    EnabledState(SubChunk<EnableState>),
    #[br(magic(b"OPAC"))]
    Opacity(SubChunk<Opacity>),
    #[br(magic(b"AXIS"))]
    DisplacementAxis(SubChunk<DisplacementAxis>),
    #[br(magic(b"TMAP"))]
    TextureMapping(SubChunk<TextureMapping>),
    #[br(magic(b"NEGA"))]
    Negative(SubChunk<EnableState>),
    #[br(magic(b"PROJ"))]
    ProjectionMode(SubChunk<ProjectionMode>),
    #[br(magic(b"IMAG"))]
    ImageMap(SubChunk<VxReference>),
    #[br(magic(b"WRAP"))]
    ImageWrapOptions(SubChunk<ImageWrapOptions>),
    #[br(magic(b"WRPW"))]
    ImageWrapAmountWidth(SubChunk<ImageWrapAmount>),
    #[br(magic(b"WRPH"))]
    ImageWrapAmountHeight(SubChunk<ImageWrapAmount>),
    #[br(magic(b"VMAP"))]
    UvVertexMap(SubChunk<UvMap>),
    #[br(magic(b"AAST"))]
    AntialiasingStrength(SubChunk<AntialiasingStrength>),
    #[br(magic(b"PIXB"))]
    PixelBlending(SubChunk<PixelBlending>),
    #[br(magic(b"TAMP"))]
    TextureAmplitude(SubChunk<ValueEnvelope>),
}

/// Pixel blending enlarges the sample filter when it would otherwise be smaller than a single
/// image map pixel. If the low-order flag bit is set, then pixel blending is enabled.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct PixelBlending {
    pub flags: u16,
}

/// The low bit of the flags word is an enable flag for texture antialiasing. The antialiasing
/// strength is proportional to the width of the sample filter, so larger values sample a larger
/// area of the image.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct AntialiasingStrength {
    pub flags: u16,
    pub strength: f32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct UvMap {
    #[br(align_after = 2)]
    pub txuv_map_name: NullString,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ImageWrapAmount {
    pub cycles: f32,
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ImageWrapOptions {
    pub width_wrap: ImageWrapType,
    pub height_wrap: ImageWrapType,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum ImageWrapType {
    Reset = 0,
    Repeat = 1,
    Mirror = 2,
    Edge = 3,
}

#[binread]
#[br(repr = u16, import(_length: u32))]
#[derive(Debug)]
pub enum ProjectionMode {
    Planar = 0,
    Cylindrical = 1,
    Spherical = 2,
    Cubic = 3,
    FrontProjection = 4,
    UV = 5,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VxReference {
    #[br(parse_with = vx)]
    pub texture_image: u32,
}

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct TextureMapping {
    #[br(parse_with = until_size_limit(length as u64))]
    pub attributes: Vec<TextureMappingSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum TextureMappingSubChunk {
    #[br(magic(b"CNTR"))]
    Center(SubChunk<VectorEnvelope>),
    #[br(magic(b"SIZE"))]
    Size(SubChunk<VectorEnvelope>),
    #[br(magic(b"ROTA"))]
    Rotation(SubChunk<VectorEnvelope>),
    #[br(magic(b"OREF"))]
    ReferenceObject(SubChunk<ReferenceObject>),
    #[br(magic(b"FALL"))]
    Falloff(SubChunk<Falloff>),
    #[br(magic(b"CSYS"))]
    CoordinateSystem(SubChunk<CoordinateSystem>),
}

#[binread]
#[br(repr = u16, import(_length: u32))]
#[derive(Debug)]
pub enum CoordinateSystem {
    ObjectCoordinates = 0,
    WorldCoordinates = 1,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct ReferenceObject {
    #[br(align_after = 2)]
    pub object_name: NullString,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Falloff {
    pub kind: FalloffType,
    pub vector: [f32; 3],
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum FalloffType {
    Cubic = 0,
    Spherical = 1,
    LinearX = 2,
    LinearY = 3,
    LinearZ = 4,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct DisplacementAxis {
    pub displacement_axis: u16,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Opacity {
    pub kind: OpacityType,
    pub opacity: f32,
    #[br(parse_with = vx)]
    pub envelope: u32,
}

#[binread]
#[br(repr = u16)]
#[derive(Debug)]
pub enum OpacityType {
    Normal = 0,
    Subtractive = 1,
    Difference = 2,
    Multiply = 3,
    Divide = 4,
    Alpha = 5,
    TextureDisplacement = 6,
    Additive = 7,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct EnableState {
    pub enable: u16,
}

#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct Channel {
    pub texture_channel: TextureChannel,
}

#[binread]
#[derive(Debug)]
pub enum TextureChannel {
    #[br(magic(b"COLR"))]
    Color,
    #[br(magic(b"DIFF"))]
    Diffuse,
    #[br(magic(b"LUMI"))]
    Luminosity,
    #[br(magic(b"SPEC"))]
    Specular,
    #[br(magic(b"GLOS"))]
    Glossy,
    #[br(magic(b"REFL"))]
    Reflectivity,
    #[br(magic(b"TRAN"))]
    Transparency,
    #[br(magic(b"RIND"))]
    RefractiveIndex,
    #[br(magic(b"TRNL"))]
    Translucency,
    #[br(magic(b"BUMP"))]
    Bump,
}
