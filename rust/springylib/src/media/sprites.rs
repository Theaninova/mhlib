use crate::error::Error;

#[derive(Debug)]
pub struct Sprites {
    pub name: String,
    pub sprite_type: SpriteType,
    pub file_name: String,
    pub render_mode: RenderMode,
    pub frames: Option<CropMode>,
}

impl Sprites {
    pub fn parse(string: &str) -> Result<Vec<Self>, Error> {
        string
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Sprites::parse_single)
            .collect()
    }

    pub fn parse_single(string: &str) -> Result<Self, Error> {
        let mut components = string.split_whitespace();
        let invalid_data = |info| Error::InvalidData {
            info,
            context: string.to_string(),
        };
        let eof = || invalid_data(Some("eof".to_string()));

        Ok(Sprites {
            file_name: components.next().ok_or_else(eof)?.to_string(),
            sprite_type: match components.next().ok_or_else(eof)? {
                "anim_rle" => SpriteType::AnimRle,
                "anim" => SpriteType::Anim,
                "static" => SpriteType::Static,
                e => return Err(invalid_data(Some(e.to_string()))),
            },
            name: components.next().ok_or_else(eof)?.to_string(),
            render_mode: match components.next().ok_or_else(eof)? {
                "normx" => RenderMode::NormX,
                "flipx" => RenderMode::FlipX,
                e => return Err(invalid_data(Some(e.to_string()))),
            },
            frames: if let Some(c) = components.next() {
                Some(match c {
                    "nocrop" => CropMode::NoCrop,
                    x => x
                        .parse::<i32>()
                        .map(CropMode::FrameCount)
                        .map_err(|err| Error::Custom(Box::new(err)))?,
                })
            } else {
                None
            },
        })
    }
}

#[derive(Debug)]
pub enum CropMode {
    FrameCount(i32),
    NoCrop,
}

#[derive(Debug)]
pub enum RenderMode {
    NormX,
    FlipX,
}

#[derive(Debug)]
pub enum SpriteType {
    Static,
    Anim,
    AnimRle,
}
