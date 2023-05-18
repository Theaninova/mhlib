use crate::pro::Chunk;
use binrw::{binread, until_eof, NullString};

#[binread]
pub struct PrMaterialList {
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
