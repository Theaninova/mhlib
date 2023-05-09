use binrw::binread;

///Store the bounding box for the vertex data in a layer. Optional. The min and max vectors are
/// the lower and upper corners of the bounding box.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}
