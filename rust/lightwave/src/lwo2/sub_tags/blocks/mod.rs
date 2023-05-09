use crate::binrw_helpers::until_size_limit;
use crate::iff::SubChunk;
use crate::lwo2::sub_tags::blocks::gradient_texture::GradientTextureSubChunk;
use crate::lwo2::sub_tags::blocks::image_texture::SurfaceBlockImageTextureSubChunk;
use crate::lwo2::sub_tags::blocks::procedural_texture::ProceduralTextureSubChunk;
use crate::lwo2::sub_tags::EnableState;
use crate::lwo2::vx;
use binrw::{binread, NullString};

pub mod gradient_texture;
pub mod image_texture;
pub mod procedural_texture;
pub mod texture_mapping;

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub enum SurfaceBlocks {
    #[br(magic(b"IMAP"))]
    ImageMapTexture {
        header: SubChunk<SurfaceBlockHeader>,
        #[br(parse_with = until_size_limit(length as u64 - (header.length as u64 + 2 + 4)))]
        attributes: Vec<SurfaceBlockImageTextureSubChunk>,
    },
    #[br(magic(b"PROC"))]
    ProceduralTexture {
        header: SubChunk<SurfaceBlockHeader>,
        #[br(parse_with = until_size_limit(length as u64 - (header.length as u64 + 2 + 4)))]
        attributes: Vec<ProceduralTextureSubChunk>,
    },
    #[br(magic(b"GRAD"))]
    GradientTexture {
        header: SubChunk<SurfaceBlockHeader>,
        #[br(parse_with = until_size_limit(length as u64 - (header.length as u64 + 2 + 4)))]
        attributes: Vec<GradientTextureSubChunk>,
    },
    #[br(magic(b"SHDR"))]
    ShaderPlugin {
        header: SubChunk<SurfaceBlockHeader>,
        #[br(magic(b"FUNC"))]
        algorithm: SubChunk<Algorithm>,
    },
}

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct Algorithm {
    #[br(align_after = 2)]
    pub algorithm_name: NullString,
    #[br(count = length - (algorithm_name.len() as u32 + 1))]
    pub data: Vec<u8>,
}

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct SurfaceBlockHeader {
    #[br(pad_before = 2)]
    #[br(parse_with = until_size_limit(length as u64 - 4))]
    pub block_attributes: Vec<SurfaceBlockHeaderSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum SurfaceBlockHeaderSubChunk {
    #[br(magic(b"CHAN"))]
    Channel(SubChunk<Channel>),
    #[br(magic(b"ENAB"))]
    EnabledState(SubChunk<EnableState>),
    #[br(magic(b"OPAC"))]
    Opacity(SubChunk<Opacity>),
    #[br(magic(b"AXIS"))]
    DisplacementAxis(SubChunk<DisplacementAxis>),
    #[br(magic(b"NEGA"))]
    Negative(SubChunk<EnableState>),
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
