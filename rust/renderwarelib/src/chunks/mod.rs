use binrw::{binread, NullString};

pub mod clump;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct RwStream {
    pub kind: u32,
    pub size: u32,
    pub library_id_stamp: u32,
}

#[binread]
#[br(little)]
#[derive(Debug)]
pub enum RwSection {
    #[br(magic(0x1u32))]
    Struct {},
    #[br(magic(0x2u32))]
    String(#[br(align_after = 4)] NullString),
    // TODO: extension
    // TODO: camera
    // TODO: texture
    // TODO: material
    // TODO: material list
    // TODO: atomic section
    // TODO: plane section
    // TODO: world
    // TODO: spline
    // TODO: matrix
    // TODO: frame list
    // TODO: geometry
    #[br(magic(0x10u32))]
    Clump {
        num_atomics: u32,
        num_lights: u32,
        num_cameras: u32,
    },
}
