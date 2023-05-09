use binrw::binread;

///Lists (x, y, z) coordinate triples for a set of points. The number of points in the chunk is
/// just the chunk size divided by 12. The PNTS chunk must precede the POLS, VMAP and VMAD chunks
/// that refer to it. These chunks list points using a 0-based index into PNTS.
///
/// The LightWave® coordinate system is left-handed, with +X to the right or east, +Y upward,
/// and +Z forward or north. Object files don't contain explicit units, but by convention the
/// unit is meters. Coordinates in PNTS are relative to the pivot point of the layer.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct PointList {
    #[br(count = length / 12, assert(length % 12 == 0))]
    pub point_location: Vec<[f32; 3]>,
}
