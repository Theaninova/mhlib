use binrw::binread;

/// Describes special properties of VMAPs.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct VertexMapParameter {
    pub uv_subdivision_type: UvSubdivisionType,
    pub sketch_color: i32,
}

#[binread]
#[br(repr = i32)]
#[derive(Debug)]
pub enum UvSubdivisionType {
    Linear = 0,
    Subpatch = 1,
    SubpatchLinearCorners = 2,
    SubpatchLinearEdges = 3,
    SubpatchDiscoEdges = 4,
}
