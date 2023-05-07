use crate::archive::{Archive, FilePointer};
use binrw::prelude::binread;
use binrw::NullString;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct Container {
    #[br(align_after = 0x20)]
    pub name: NullString,
    #[br(temp)]
    pub count: u32,
    #[br(align_after = 0x20)]
    pub unk1: u32,
    #[br(count = count)]
    pub files: Vec<FileEntry>,
}

impl From<Container> for Archive {
    fn from(value: Container) -> Self {
        Archive(
            value
                .files
                .into_iter()
                .map(|it| (it.name.to_string(), it.into()))
                .collect(),
        )
    }
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

impl From<FileEntry> for FilePointer {
    fn from(value: FileEntry) -> Self {
        FilePointer {
            position: value.pos as usize,
            length: value.len as usize,
        }
    }
}
