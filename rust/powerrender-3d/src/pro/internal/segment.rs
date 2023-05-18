use crate::pro::internal::PrBoundingInfo;
use crate::pro::Chunk;
use binrw::{binread, until_eof, NullString};

#[binread]
pub struct PrSegmentList {
    pub segments: u32,
    #[br(parse_with = until_eof)]
    pub chunks: Vec<PrSegmentChunks>,
}

#[binread]
#[derive(Debug)]
pub enum PrSegmentChunks {
    #[br(magic = 0x1010u16)]
    SegmentName(#[br(map = Chunk::inner)] NullString),
    #[br(magic = 0x1020u16)]
    SegmentFlags(#[br(map = Chunk::inner)] u32),
    #[br(magic = 0x1022u16)]
    SegmentBbox(#[br(map = Chunk::inner)] Box<PrBoundingInfo>),
    #[br(magic = 0x1030u16)]
    Vertices(#[br(map = Chunk::inner)] PrVertices),
    #[br(magic = 0x1040u16)]
    Faces(#[br(map = |it: Chunk<PrFaces>| it.inner().faces)] Vec<PrFace>),
    #[br(magic = 0x1050u16)]
    SegBuf(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1051u16)]
    SegBuf2(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1052u16)]
    SegBuf3(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1060u16)]
    LodInfo(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1F00u16)]
    KeyframePivot(#[br(map = Chunk::inner)] [f32; 3]),
    #[br(magic = 0x1F10u16)]
    KeyframeMatrix(#[br(map = Chunk::inner)] [f32; 16]),
    #[br(magic = 0x1F20u16)]
    KeyframeRotKeys(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1F30u16)]
    KeyframePosKeys(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1F40u16)]
    KeyframeScaleKeys(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x1F50u16)]
    KeyframeLinks(#[br(map = Chunk::inner)] [i32; 3]),
    #[br(magic = 0x4000u16)]
    TexCoords(#[br(map = Chunk::inner)] ()), // TODO
}

#[binread]
#[derive(Debug)]
pub struct PrVertices {
    #[br(temp)]
    count: i32,
    #[br(count = count)]
    pub vertices: Vec<PrVertex>,
}

#[binread]
#[derive(Debug)]
pub struct PrVertex {
    pub position: [i32; 3],
    pub normal: [i16; 3],
}

#[binread]
#[derive(Debug)]
struct PrFaces {
    #[br(temp)]
    pub num_faces: i32,
    #[br(count = num_faces)]
    pub faces: Vec<PrFace>,
}

#[binread]
#[derive(Debug)]
pub struct PrFace {
    pub material: u16,
    pub back_material: u16,
    pub i0: i32,
    pub i1: i32,
    pub i2: i32,
    pub u0: i32,
    pub u1: i32,
    pub u2: i32,
    pub v0: i32,
    pub v1: i32,
    pub v2: i32,
    pub color: [i32; 3],
    pub normal: [u16; 3],
    pub flags: u8,
    pub dot_prod: u16,
}
