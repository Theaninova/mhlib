use crate::iff::SubChunk;
use crate::lwo2::sub_tags::texture_mapping::TextureMapping;
use crate::lwo2::sub_tags::{ValueEnvelope, VxReference};
use crate::lwo2::vx;
use binrw::{binread, NullString};

#[binread]
#[derive(Debug)]
pub enum SurfaceBlockImageTextureSubChunk {
    #[br(magic(b"TMAP"))]
    TextureMapping(SubChunk<TextureMapping>),
    #[br(magic(b"PROJ"))]
    ProjectionMode(SubChunk<ProjectionMode>),
    #[br(magic(b"AXIS"))]
    MajorAxis(SubChunk<MajorAxis>),
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

/// The major axis used for planar, cylindrical and spherical projections. The value is 0, 1 or 2
/// for the X, Y or Z axis.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct MajorAxis {
    pub texture_axis: u16,
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
