use crate::archive::{Archive, FilePointer};
use binrw::{binread, parser, until_exclusive, BinResult, NullString};

#[binread]
#[br(little, import(string_size: usize))]
#[derive(Debug)]
pub struct Container {
    #[br(temp, args(string_size))]
    pub header: FileEntry,
    #[br(parse_with = until_end, args_raw(string_size))]
    pub entries: Vec<FileEntry>,
}

#[parser(reader, endian)]
fn until_end(string_size: usize) -> BinResult<Vec<FileEntry>> {
    until_exclusive(|entry: &FileEntry| entry.name.to_string().as_str() == "****")(
        reader,
        endian,
        (string_size,),
    )
}

impl From<Container> for Archive {
    fn from(value: Container) -> Self {
        Archive(
            value
                .entries
                .into_iter()
                .map(|entry| (entry.name.to_string(), entry.into()))
                .collect(),
        )
    }
}

#[binread]
#[br(little, import(string_size: usize))]
#[derive(Debug)]
pub struct FileEntry {
    #[br(pad_size_to = string_size)]
    pub name: NullString,
    #[br(pad_size_to = 0x10)]
    pub pointer: [u32; 2],
}

impl From<FileEntry> for FilePointer {
    fn from(value: FileEntry) -> Self {
        FilePointer {
            position: value.pointer[0] as usize,
            length: value.pointer[1] as usize,
        }
    }
}
