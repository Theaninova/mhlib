use crate::pro::chunk::Chunk;
use crate::pro::internal::materials::{PrMaterialChunks, PrMaterialList};
use crate::pro::internal::segment::{PrSegmentChunks, PrSegmentList};
use binrw::{binread, until_eof, BinRead, NullString};

pub mod materials;
pub mod segment;
pub mod surface;

#[binread]
pub struct PrObject {
    #[br(parse_with = until_eof)]
    pub chunks: Vec<PrChunks>,
}

#[binread]
#[derive(Debug)]
pub enum PrChunks {
    #[br(magic = 0x0000u16)]
    Version(#[br(map = Chunk::inner)] f32),
    #[br(magic = 0x0100u16)]
    ObjectName(#[br(map = Chunk::inner)] NullString),
    #[br(magic = 0x0101u16)]
    ObjectFlags(#[br(map = Chunk::inner)] u32),
    #[br(magic = 0x1000u16)]
    Segments(#[br(map = |it: Chunk::<PrSegmentList>| it.0.chunks)] Vec<PrSegmentChunks>),
    #[br(magic = 0x2010u16)]
    TextureList(#[br(map = |it: Chunk::<VecWithCount<(), NullString>>| it.0.data)] Vec<NullString>),
    #[br(magic = 0x2011u16)]
    TextureAlpha(#[br(map = |it: Chunk::<VecWithCount<(), u8>>| it.0.data)] Vec<u8>),
    #[br(magic = 0x2012u16)]
    TextureStage(#[br(map = |it: Chunk::<VecWithCount<(), u8>>| it.0.data)] Vec<u8>),
    #[br(magic = 0x2013u16)]
    TextureFormat(#[br(map = |it: Chunk::<VecWithCount<(), u8>>| it.0.data)] Vec<u8>),
    #[br(magic = 0x2014u16)]
    TextureMulti(#[br(map = |it: Chunk::<VecWithCount<(), u8>>| it.0.data)] Vec<u8>),
    #[br(magic = 0x2100u16)]
    MaterialList(#[br(map = |it: Chunk::<PrMaterialList>| it.0.chunks)] Vec<PrMaterialChunks>),
    #[br(magic = 0x3000u16)]
    Camera(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x3010u16)]
    ObjectBbox(#[br(map = Chunk::inner)] Box<PrBoundingInfo>),
    #[br(magic = 0x3100u16)]
    VertexShaderList(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x3101u16)]
    VertexShaderList2(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x3102u16)]
    VertexShaderList3(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x3200u16)]
    PixelShaderList(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x3202u16)]
    PixelShaderList3(#[br(map = Chunk::inner)] ()), // TODO
}

#[binread]
#[derive(Debug)]
pub struct PrBoundingInfo {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
    /// Four corners + center vertex
    pub box_center: [[f32; 3]; 9],
    /// Same, but transformed
    pub t_box_center: [[f32; 3]; 9],
    pub radius: f32,
}

#[binread]
pub struct VertexShaderList {
    pub name: NullString,
    pub flags: u32,
    pub size: u32,
    pub num_constants: u32,
}

#[binread]
pub(crate) struct VecWithCount<Args: Clone + Default, T: for<'a> BinRead<Args<'a> = Args> + 'static>
{
    #[br(temp)]
    count: u16,
    #[br(count = count)]
    pub data: Vec<T>,
}
