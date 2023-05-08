use binrw::prelude::*;

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
