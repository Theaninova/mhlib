use crate::binrw_helpers::until_size_limit;
use crate::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
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
    pub attributes: Vec<SurfaceParameterSubChunk>,
}
