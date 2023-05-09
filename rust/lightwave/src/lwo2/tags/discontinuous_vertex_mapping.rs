use crate::binrw_helpers::until_size_limit;
use crate::lwo2::vx;
use binrw::{binread, NullString, PosValue};

/// (Introduced with LightWave® 6.5.) Associates a set of floating-point vectors with the vertices
/// of specific polygons. VMADs are similar to VMAPs, but they assign vectors to polygon vertices
/// rather than points. For a given mapping, a VMAP always assigns only one vector to a point, while
/// a VMAD can assign as many vectors to a point as there are polygons sharing the point.
///
/// The motivation for VMADs is the problem of seams in UV texture mapping. If a UV map is
/// topologically equivalent to a cylinder or a sphere, a seam is formed where the opposite edges of
/// the map meet. Interpolation of UV coordinates across this discontinuity is aesthetically and
/// mathematically incorrect. The VMAD substitutes an equivalent mapping that interpolates
/// correctly. It only needs to do this for polygons in which the seam lies.
///
/// VMAD chunks are paired with VMAPs of the same name, if they exist. The vector values in the VMAD
/// will then replace those in the corresponding VMAP, but only for calculations involving the
/// specified polygons. When the same points are used for calculations on polygons not specified in
/// the VMAD, the VMAP values are used.
///
/// VMADs need not be associated with a VMAP. They can also be used simply to define a
/// (discontinuous) per-polygon mapping. But not all mapping types are valid for VMADs, since for
/// some types it makes no sense for points to have more than one map value. TXUV, RGB, RGBA and
/// WGHT types are supported for VMADs, for example, while MORF and SPOT are not. VMADs of
/// unsupported types are preserved but never evaluated.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct DiscontinuousVertexMappings {
    #[br(temp)]
    pub start_pos: PosValue<()>,
    pub kind: [u8; 4],
    #[br(temp)]
    pub dimension: u16,
    #[br(align_after = 2)]
    pub name: NullString,
    #[br(temp)]
    pub end_pos: PosValue<()>,
    #[br(parse_with = |reader, endian, _: ()| until_size_limit(length as u64 - (end_pos.pos - start_pos.pos))(reader, endian, (dimension, )))]
    pub mappings: Vec<DiscontinuousVertexMapping>,
}

#[binread]
#[br(import(dimension: u16))]
#[derive(Debug)]
pub struct DiscontinuousVertexMapping {
    #[br(parse_with = vx)]
    pub vert: u32,
    #[br(parse_with = vx)]
    pub poly: u32,
    #[br(count = dimension)]
    pub values: Vec<f32>,
}
