use binrw::prelude::*;
use binrw::{BinRead, Error};
use image;
use image::error::{DecodingError, ImageFormatHint};
use image::{ImageError, ImageResult, Rgb, RgbImage};
use std::io::Cursor;

#[binrw]
#[brw(little)]
pub struct LevelTile {
    pub index: u8,
    pub id: u8,
}

#[binrw]
#[brw(little)]
pub struct LevelLayer {
    pub tile_count: u32,
    pub width: u32,
    pub height: u32,
    pub unknown_2: u32,
    #[br(count = width * height)]
    pub tiles: Vec<LevelTile>,
}

pub fn level_tile_data_to_image(tile_data: &[u8]) -> ImageResult<RgbImage> {
    let mut cursor = Cursor::new(tile_data);
    let layer = LevelLayer::read(&mut cursor).map_err(to_decoding_err)?;

    let mut image = RgbImage::new(layer.width, layer.height);
    for y in 0..layer.height {
        for x in 0..layer.width {
            let tile = LevelTile::read(&mut cursor).map_err(to_decoding_err)?;
            image.put_pixel(x, y, Rgb([tile.id, tile.index, 0]));
        }
    }

    Ok(image)
}

fn to_decoding_err(err: Error) -> ImageError {
    ImageError::Decoding(DecodingError::new(
        ImageFormatHint::Name(String::from("mhjnr_layer")),
        err,
    ))
}
