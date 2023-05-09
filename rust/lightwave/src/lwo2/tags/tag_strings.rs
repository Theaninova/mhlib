use crate::binrw_helpers::until_size_limit;
use binrw::{binread, NullString};

/// Lists the tag strings that can be associated with polygons by the PTAG chunk.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct TagStrings {
    #[br(parse_with = until_size_limit(length as u64))]
    pub tag_strings: Vec<NullString>,
}
