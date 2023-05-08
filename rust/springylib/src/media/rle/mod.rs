use binrw::prelude::*;
use binrw::Endian;
use std::io::{Read, Seek};

#[cfg(all(feature = "rle_gif"))]
pub mod gif;

#[binread]
#[br(little, magic = 0x67u32)]
pub struct RleImage {
    pub hash: u64,
    pub color_table: [[u8; 4]; 512],
    pub width: u32,
    pub height: u32,
    pub numerator: u32,
    pub denominator: u32,
    #[br(temp)]
    pub frame_count: u32,
    #[br(count = frame_count)]
    pub frames: Vec<RleLayer>,
}

#[binread]
#[br(little)]
pub struct RleLayer {
    pub width: u32,
    pub height: u32,
    pub left: u32,
    pub top: u32,
    pub numerator: u32,
    pub denominator: u32,
    pub data_size: u32,
    pub unknown3: u32,
    #[br(args(width * height), parse_with = parse_rle)]
    pub data: Vec<u8>,
}

pub fn parse_rle<R: Read + Seek>(
    reader: &mut R,
    endian: Endian,
    (size,): (u32,),
) -> BinResult<Vec<u8>> {
    let mut data = Vec::with_capacity(size as usize);

    while data.len() != size as usize {
        let count: i8 = reader.read_type(endian)?;
        if count > 0 {
            let value: u8 = reader.read_type(endian)?;
            for _ in 0..count {
                data.push(value);
            }
        } else {
            for _ in 0..-count {
                data.push(reader.read_type(endian)?);
            }
        }
    }

    Ok(data)
}

impl RleImage {
    pub fn get_image_data(&self, layer: &RleLayer) -> Vec<u8> {
        let mut data = Vec::<u8>::with_capacity(self.width as usize * self.height as usize * 4);
        let mut i = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if y < layer.top
                    || y >= layer.top + layer.height
                    || x < layer.left
                    || x >= layer.left + layer.width
                {
                    data.push(0);
                    data.push(0);
                    data.push(0);
                    data.push(0);
                } else {
                    let color = self.color_table[layer.data[i] as usize];
                    i += 1;
                    data.push(color[2]);
                    data.push(color[1]);
                    data.push(color[0]);
                    data.push(if color[2] == 0 && color[1] == 0 && color[0] == 0 {
                        0
                    } else {
                        255
                    });
                }
            }
        }

        data
    }
}

pub fn bgra_to_rgba(pixel: [u8; 4]) -> [u8; 4] {
    [pixel[2], pixel[1], pixel[0], pixel[3]]
}
