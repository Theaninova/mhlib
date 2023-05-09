use binrw::{binread, NullString};

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct ShaderAlgorithm {
    #[br(align_after = 2)]
    pub algorithm_name: NullString,
    #[br(count = length - (algorithm_name.len() as u32 + 1))]
    pub data: Vec<u8>,
}
