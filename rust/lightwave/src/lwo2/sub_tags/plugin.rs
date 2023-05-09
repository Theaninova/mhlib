use binrw::{binread, NullString, PosValue};

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct PluginServerNameAndData {
    #[br(temp)]
    start_pos: PosValue<()>,
    #[br(align_after = 2)]
    pub server_name: NullString,
    pub flags: u16,
    #[br(temp)]
    end_pos: PosValue<()>,
    #[br(count = length as u64 - (end_pos.pos - start_pos.pos))]
    pub parameters: Vec<u8>,
}
