use binrw::prelude::BinRead;
use binrw::NullString;
use std::collections::HashMap;
use std::io::{Read, Seek};
use std::ops::Deref;

pub mod error;
mod v1;
mod v2;

use crate::archive::error::{Error, Result};

/// Archive info
pub struct Archive(HashMap<String, FilePointer>);

/// Pointer to the file inside the archive
#[derive(Debug, PartialEq)]
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
pub enum ArchiveKind {
    /// Appears in a variety of Moorhuhn Shoot 'em Up
    /// games, starting with Moorhuhn Winter.
    ///
    /// The name can have a max length of 0x30/0x40, however the header
    /// does not store the amount of files and instead is delimited
    /// by a final entry with the name `****`
    ///
    /// File Entries have a max path length of 0x30/0x40 with a total entry
    /// size of 0x40.
    V1(usize),
    /// Appears in Moorhuhn Jump 'n Run games as well
    /// as Moorhuhn Kart 2, starting with Moorhuhn Kart 2.
    ///
    /// This one has a name with a max length of 0x20,
    /// with a total header size of 0x40
    ///
    /// File Entries have a max path length of 0x68,
    /// with a total entry size of 0x80
    V2,
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
            "MHJNR-XXL" | "MHJNR-XS" | "Moorhuhn Kart 2" => Ok(ArchiveKind::V2),
            "MH-W V1.0" | "MH3 V1.0 " | "MH 1 REMAKE" => Ok(ArchiveKind::V1(0x30)),
            "MHP XXL" | "MHINV XXL V1.0" => Ok(ArchiveKind::V1(0x40)),
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
        let kind = ArchiveKind::guess(reader)?;
        Archive::read_kind(reader, kind)
    }

    /// Reads a specific archive kind from a binary stream
    ///
    /// Usually you want to use `read` instead.
    pub fn read_kind<R>(reader: &mut R, kind: ArchiveKind) -> Result<Archive>
    where
        R: Read + Seek,
    {
        match kind {
            ArchiveKind::V1(size) => Ok(v1::Container::read_args(reader, (size,))?.into()),
            ArchiveKind::V2 => Ok(v2::Container::read(reader)?.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::archive::{Archive, FilePointer};
    use std::io::Cursor;

    #[test]
    fn it_should_load_v2() {
        let bin = include_bytes!("v2a.dat");
        let archive = Archive::read(&mut Cursor::new(bin)).unwrap();
        assert_eq!(archive.len(), 2);
        assert_eq!(
            archive["data\\config.txt"],
            FilePointer {
                position: 0x57b40,
                length: 0xf4
            }
        );
        assert_eq!(
            archive["data\\fonts\\dangerfont.bmp"],
            FilePointer {
                position: 0x57c40,
                length: 0x7dfd8,
            }
        )
    }

    #[test]
    fn it_should_load_v1a() {
        let bin = include_bytes!("v1a.dat");
        let archive = Archive::read(&mut Cursor::new(bin)).unwrap();
        assert_eq!(archive.len(), 2);
        assert_eq!(
            archive["data\\mhx.fnt"],
            FilePointer {
                position: 0x1200,
                length: 0x8d9,
            }
        );
        assert_eq!(
            archive["data\\text.txt"],
            FilePointer {
                position: 0x1c00,
                length: 0x427e,
            }
        )
    }

    #[test]
    fn it_should_load_v1b() {
        let bin = include_bytes!("v1b.dat");
        let archive = Archive::read(&mut Cursor::new(bin)).unwrap();
        assert_eq!(archive.len(), 2);
        assert_eq!(
            archive["data\\endbranding_xxl.txt"],
            FilePointer {
                position: 0x7000,
                length: 0x40,
            }
        );
        assert_eq!(
            archive["data\\settings_xxl.txt"],
            FilePointer {
                position: 0x7200,
                length: 0x872,
            }
        )
    }
}
