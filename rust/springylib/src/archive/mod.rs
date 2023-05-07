use binrw::prelude::BinRead;
use binrw::NullString;
use std::collections::HashMap;
use std::io::{Read, Seek};
use std::ops::Deref;

pub mod error;
mod mhjnr;
mod standard;

use crate::archive::error::{Error, Result};

/// Archive info
pub struct Archive(HashMap<String, FilePointer>);

/// Pointer to the file inside the archive
pub struct FilePointer {
    pub position: usize,
    pub length: usize,
}

impl Deref for Archive {
    type Target = HashMap<String, FilePointer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<String, FilePointer>> for Archive {
    fn from(value: HashMap<String, FilePointer>) -> Self {
        Archive(value)
    }
}

/// These are all slightly divergent data layouts
enum ArchiveKind {
    /// Appears in a variety of Moorhuhn Shoot 'em Up
    /// games, starting with Moorhuhn Winter.
    ///
    /// The name can have a max length of 0x30, however the header
    /// does not store the amount of files and instead is delimited
    /// by a final entry with the name `****`
    ///
    /// File Entries have a max path length of 0x30 with a total entry
    /// size of 0x40.
    V1,
    /// Appears in Moorhuhn Jump 'n Run games as well
    /// as Moorhuhn Kart 2, starting with Moorhuhn Kart 2.
    ///
    /// This one has a name with a max length of 0x20,
    /// with a total header size of 0x40
    ///
    /// File Entries have a max path length of 0x68,
    /// with a total entry size of 0x80
    V2,
    /// Appears in later Moorhuhn Shoot 'em Up games, starting with Moorhuhn
    /// Invasion.
    ///
    /// Works the same as V2, but has the maximum header and path string size
    /// increased from 0x30 to 0x40
    V3,
}

impl ArchiveKind {
    /// Guesses the archive type based on the file type
    pub fn guess<R>(reader: &mut R) -> Result<ArchiveKind>
    where
        R: Read + Seek,
    {
        let name = NullString::read(reader)?.to_string();
        reader.rewind()?;
        match name.as_str() {
            "MHJNR-XXL" | "MHJNR-XS" | "Moorhuhn Kart 2" => Ok(ArchiveKind::V1),
            "MH-W V1.0" | "MH3 V1.0 " | "MH 1 REMAKE" => Ok(ArchiveKind::V2),
            "MHP XXL" | "MHINV XXL V1.0" => Ok(ArchiveKind::V3),
            name => Err(Error::Unsupported {
                reason: name.to_string(),
            }),
        }
    }
}

impl Archive {
    /// Reads the archive info from a binary stream
    pub fn read<R>(reader: &mut R) -> Result<Archive>
    where
        R: Read + Seek,
    {
        match ArchiveKind::guess(reader)? {
            ArchiveKind::V1 => Ok(standard::Container::read_args(reader, (0x30,))?.into()),
            ArchiveKind::V2 => Ok(mhjnr::Container::read(reader)?.into()),
            ArchiveKind::V3 => Ok(standard::Container::read_args(reader, (0x40,))?.into()),
        }
    }
}
