use binrw::{binread, NullString};
use std::collections::HashMap;

#[binread]
#[br(little, magic = b"MHJNR")]
#[derive(Debug)]
pub struct Datafile {
    #[br(align_after = 0x20)]
    pub edition: Edition,

    #[br(temp)]
    pub count: u32,
    #[br(align_after = 0x20)]
    pub unk1: u32,

    #[br(count = count)]
    pub files: Vec<FileEntry>,
}

#[binread]
#[derive(Debug)]
pub enum Edition {
    #[br(magic = b"-XS")]
    Xs,
    #[br(magic = b"-XXL")]
    Xxl,
}

#[derive(Debug)]
#[binread]
pub struct FileEntry {
    #[br(pad_size_to = 0x68)]
    pub name: NullString,
    pub pos: u32,
    #[br(pad_after = 0x10)]
    pub len: u32,
}

impl Datafile {
    pub fn into_index(self) -> HashMap<String, FileEntry> {
        self.files
            .into_iter()
            .map(|entry| (entry.name.to_string(), entry))
            .collect()
    }
}
