use crate::pro::chunk::Chunk;
use binrw::__private::magic;
use binrw::{binread, until_eof, BinRead, BinReaderExt, BinResult, Endian, NullString};
use std::fmt::Debug;
use std::io::{Read, Seek};

pub mod chunk;

pub fn read_power_render_object<R>(reader: &mut R) -> BinResult<Vec<PrChunks>>
where
    R: Read + Seek,
{
    magic(reader, 0x0303u16, Endian::Little)?;
    let data: Chunk<PrObject> = Chunk::<PrObject>::read(reader)?;
    Ok(data.0.chunks)
}

#[binread]
struct PrObject {
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
struct PrMaterialList {
    #[br(parse_with = until_eof)]
    pub chunks: Vec<PrMaterialChunks>,
}

#[binread]
#[derive(Debug)]
pub enum PrMaterialChunks {
    #[br(magic = 0x2101u16)]
    MaterialName(#[br(map = Chunk::inner)] NullString),
    #[br(magic = 0x2102u16)]
    MaterialMethod(#[br(map = Chunk::inner)] u32),
    #[br(magic = 0x2103u16)]
    MaterialTexNum(#[br(map = Chunk::inner)] u32),
    #[br(magic = 0x2104u16)]
    MaterialBaseColor(#[br(map = Chunk::inner)] u8),
    #[br(magic = 0x2105u16)]
    MaterialShades(#[br(map = Chunk::inner)] u32),
    #[br(magic = 0x2106u16)]
    MaterialTable(#[br(map = Chunk::inner)] u8),
    #[br(magic = 0x2107u16)]
    MaterialEnvMap(#[br(map = Chunk::inner)] (u8, u8)),
    #[br(magic = 0x2108u16)]
    MaterialMipMap(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x2109u16)]
    MaterialColor(#[br(map = Chunk::inner)] [f32; 4]),
    #[br(magic = 0x2110u16)]
    MaterialNumStages(#[br(map = Chunk::inner)] u32),
    #[br(magic = 0x2111u16)]
    MaterialTexNum2(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x2112u16)]
    MaterialEnvMap2(#[br(map = Chunk::inner)] ()), // TODO
    #[br(magic = 0x2113u16)]
    MaterialBump(#[br(map = Chunk::inner)] (f32, [f32; 4])),
    #[br(magic = 0x2114u16)]
    MaterialSpecular(#[br(map = Chunk::inner)] ([f32; 4], f32)),
    #[br(magic = 0x2115u16)]
    MaterialTwoSided(#[br(map = Chunk::inner)] u8),
    #[br(magic = 0x2116u16)]
    MaterialVertexShaderName(#[br(map = Chunk::inner)] NullString),
    #[br(magic = 0x2117u16)]
    MaterialPixelShaderName(#[br(map = Chunk::inner)] NullString),
    #[br(magic = 0x2199u16)]
    MaterialEnd(#[br(map = Chunk::inner)] ()),
}

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
    Faces(#[br(map = Chunk::inner)] ()), // TODO
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

#[derive(Debug)]
pub struct PrVertices {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
}

impl BinRead for PrVertices {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        let count = reader.read_type::<u32>(endian)? as usize;
        let mut result = PrVertices {
            vertices: Vec::with_capacity(count),
            normals: Vec::with_capacity(count),
        };
        for _ in 0..count {
            result.vertices.push(reader.read_type(endian)?);
            let normal: [i16; 3] = reader.read_type(endian)?;
            result.normals.push([
                normal[0] as f32 / 1024.0,
                normal[1] as f32 / 1024.0,
                normal[2] as f32 / 1024.0,
            ]);
        }
        Ok(result)
    }
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
    pub c0: i32,
    pub c1: i32,
    pub c2: i32,
    pub normal: [u16; 3],
    pub flags: u8,
    pub dot_prod: u16,
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
