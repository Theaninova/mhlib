use binrw::binread;
use crate::lwo2::sub_tags::envelope_type::EnvelopeType;

pub mod envelope_type;

#[binread]
#[derive(Debug)]
pub enum SubTag {
    #[br(magic(b"TYPE"))]
    EnvelopeType(EnvelopeType)
}
