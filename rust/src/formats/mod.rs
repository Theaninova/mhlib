use crate::formats::datafile::FileEntry;
use crate::formats::level::LevelLayer;
use crate::formats::rle::RleImage;
use crate::formats::sprites::Sprites;
use crate::formats::txt::{decrypt_txt, DecryptError};
use crate::formats::ui_xml::UiTag;
use binrw::BinRead;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::Path;

pub mod datafile;
pub mod level;
pub mod rle;
pub mod sprites;
pub mod txt;
pub mod ui_xml;

pub enum DatafileFile {
    Txt(String),
    Level(LevelLayer),
    Sprites(Vec<Sprites>),
    RleSprite(Box<RleImage>),
    Bitmap(Vec<u8>),
    Vorbis(Vec<u8>),
    TileCollision(String),
    Ui(UiTag),
}

pub enum Error {
    Deserialization,
    UnknownFormat(String),
    UnknownError,
    Custom(String),
    DecryptError(DecryptError),
}

fn custom_err<T>(e: T) -> Error
where
    T: Debug,
{
    Error::Custom(format!("{:#?}", e))
}

pub fn load_data<R>(entry: &FileEntry, reader: &mut R) -> Result<DatafileFile, Error>
where
    R: Read + Seek,
{
    reader
        .seek(SeekFrom::Start(entry.pos as u64))
        .map_err(custom_err)?;
    let mut data = vec![0u8; entry.len as usize];
    reader.read_exact(&mut data).map_err(custom_err)?;

    let name = entry.name.to_string();
    let path = Path::new(&name);

    match path
        .extension()
        .and_then(OsStr::to_str)
        .ok_or(Error::Custom("No extension".to_string()))?
    {
        "dat" => Ok(DatafileFile::Level(
            LevelLayer::read(&mut Cursor::new(data)).map_err(custom_err)?,
        )),
        "rle" => Ok(DatafileFile::RleSprite(Box::new(
            RleImage::read(&mut Cursor::new(data)).map_err(custom_err)?,
        ))),
        "bmp" => Ok(DatafileFile::Bitmap(data)),
        "ogg" => Ok(DatafileFile::Vorbis(data)),
        "xml" => {
            serde_xml_rs::from_str::<UiTag>(String::from_utf8(data).map_err(custom_err)?.as_str())
                .map_err(custom_err)
                .map(DatafileFile::Ui)
        }
        "txt" => {
            let stem = path
                .file_stem()
                .and_then(OsStr::to_str)
                .ok_or(Error::Custom("Stem".to_string()))?;
            let decr = decrypt_txt(data.into_iter()).map_err(|e| Error::DecryptError(e))?;
            if stem.starts_with("tile_collision") {
                Ok(DatafileFile::TileCollision(decr))
            } else if stem == "sprites" {
                Ok(DatafileFile::Sprites(
                    Sprites::parse(decr.as_str()).map_err(custom_err)?,
                ))
            } else {
                Ok(DatafileFile::Txt(decr))
            }
        }
        /*Some("rle") => {
            let image: RleImage = RleImage::read(&mut Cursor::new(data)).unwrap();
            let path = Path::new(dat_path).with_file_name("res.gif");
            println!("{:?}", path);
            let mut encoder = GifEncoder::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(path)
                    .unwrap(),
            );
            encoder.set_repeat(Repeat::Infinite).unwrap();
            encoder.try_encode_frames(image.into_frames()).unwrap();
        }
        Some("dat") => {
            let image = level_tile_data_to_image(&data).unwrap();
            let path = Path::new(dat_path).with_file_name("res.png");
            println!("{:?}", path);
            image.save_with_format(path, ImageFormat::Png).unwrap();
        }*/
        ext => Err(Error::UnknownFormat(ext.to_string())),
    }
}
