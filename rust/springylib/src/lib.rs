use crate::archive::FilePointer;
use crate::error::Error;
use crate::media::level::LevelLayer;
use crate::media::rle::RleImage;
use crate::media::sprites::Sprites;
use crate::media::txt::{decrypt_exposed_txt, decrypt_txt};
use crate::media::ui::UiTag;
use binrw::prelude::BinRead;
use encoding_rs::WINDOWS_1252;
use itertools::Itertools;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

pub mod archive;
pub mod error;
pub mod media;

pub enum DatafileFile {
    Txt(String),
    Level(LevelLayer),
    Sprites(Vec<Sprites>),
    RleSprite(Box<RleImage>),
    Bitmap(Vec<u8>),
    Vorbis(Vec<u8>),
    TileCollision(String),
    Ui(UiTag),
    Translations(HashMap<String, Vec<String>>),
}

impl FilePointer {
    pub fn load_from<R>(&self, reader: &mut R) -> Result<DatafileFile, Error>
    where
        R: Read + Seek,
    {
        reader.seek(SeekFrom::Start(self.position as u64))?;
        let mut data = vec![0u8; self.length as usize];
        reader.read_exact(&mut data)?;
        let path = Path::new(&self.path);

        match path
            .extension()
            .and_then(OsStr::to_str)
            .ok_or(Error::InvalidExtension(None))?
        {
            "dat" => Ok(DatafileFile::Level(LevelLayer::read(&mut Cursor::new(
                data,
            ))?)),
            "rle" => Ok(DatafileFile::RleSprite(Box::new(RleImage::read(
                &mut Cursor::new(data),
            )?))),
            "bmp" => Ok(DatafileFile::Bitmap(data)),
            "ogg" => Ok(DatafileFile::Vorbis(data)),
            "xml" => Ok(DatafileFile::Ui(
                serde_xml_rs::from_str::<UiTag>(String::from_utf8(data)?.as_str())?.post_process(),
            )),
            "txt" => {
                let stem = path
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| Error::InvalidPath(path.to_string_lossy().to_string()))?;
                let decr = decrypt_txt(data.into_iter())?;
                if stem.starts_with("tile_collision") {
                    Ok(DatafileFile::TileCollision(decr))
                } else if stem == "sprites" {
                    Ok(DatafileFile::Sprites(Sprites::parse(decr.as_str())?))
                } else if stem.starts_with("profile") || stem.starts_with("highscores") {
                    Ok(DatafileFile::Txt(decrypt_exposed_txt(decr)?))
                } else {
                    Ok(DatafileFile::Txt(decr))
                }
            }
            "csv" => Ok(DatafileFile::Translations(
                WINDOWS_1252
                    .decode(data.as_slice())
                    .0
                    .split('\n')
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .map(|l| {
                        l.splitn(2, ';')
                            .map(|s| s.to_string())
                            .collect_tuple::<(String, String)>()
                            .expect("Invalid csv")
                    })
                    .into_group_map(),
            )),
            ext => Err(Error::UnknownFormat(ext.to_string())),
        }
    }
}
