#[derive(Debug)]
pub struct Sprites {
    pub name: String,
    pub sprite_type: SpriteType,
    pub file_name: String,
    pub render_mode: RenderMode,
    pub frames: Option<CropMode>,
}

#[derive(Debug)]
pub enum Error {
    InvalidData,
    UnknownEnum(String),
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

        Ok(Sprites {
            file_name: components.next().ok_or(Error::InvalidData)?.to_string(),
            sprite_type: match components.next().ok_or(Error::InvalidData)? {
                "anim_rle" => SpriteType::AnimRle,
                "anim" => SpriteType::Anim,
                "static" => SpriteType::Static,
                e => return Err(Error::UnknownEnum(e.to_string())),
            },
            name: components.next().ok_or(Error::InvalidData)?.to_string(),
            render_mode: match components.next().ok_or(Error::InvalidData)? {
                "normx" => RenderMode::NormX,
                "flipx" => RenderMode::FlipX,
                e => return Err(Error::UnknownEnum(e.to_string())),
            },
            frames: if let Some(c) = components.next() {
                Some(match c {
                    "nocrop" => CropMode::NoCrop,
                    x => x
                        .parse::<i32>()
                        .map(CropMode::FrameCount)
                        .map_err(|e| Error::UnknownEnum(e.to_string()))?,
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
