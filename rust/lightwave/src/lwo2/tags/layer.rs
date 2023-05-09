use binrw::{binread, NullString, PosValue};

/// Signals the start of a new layer. All the data chunks which follow will be included in this
/// layer until another layer chunk is encountered. If data is encountered before a layer chunk,
/// it goes into an arbitrary layer. If the least significant bit of flags is set, the layer is
/// hidden. The parent index indicates the default parent for this layer and can be -1 or missing
/// to indicate no parent.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct Layer {
    pub number: u16,
    pub flags: u16,
    pub pivot: [f32; 3],
    #[br(align_after = 2)]
    pub name: NullString,
    pub status: PosValue<()>,
    #[br(if(status.pos < length as u64))]
    pub parent: Option<u16>,
}
