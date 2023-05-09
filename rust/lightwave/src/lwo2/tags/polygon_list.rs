use crate::binrw_helpers::{count_with_vx, until_size_limit};
use binrw::binread;

/// A list of polygons for the current layer. Possible polygon types include:
///
/// * **FACE**
///     <ul>"Regular" polygons, the most common.</ul>
/// * **CURV**
///     <ul>Catmull-Rom splines. These are used during modeling and are currently ignored by the
///     renderer.</ul>
/// * **PTCH**
///     <ul>Subdivision patches. The POLS chunk contains the definition of the control cage
///     polygons, and the patch is created by subdividing these polygons. The renderable geometry
///     that results from subdivision is determined interactively by the user through settings
///     within LightWave®. The subdivision method is undocumented.</ul>
/// * **MBAL**
///     <ul>Metaballs. These are single-point polygons. The points are associated with a VMAP of
///     type MBAL that contains the radius of influence of each metaball. The renderable polygonal
///     surface constructed from a set of metaballs is inferred as an isosurface on a scalar field
///     derived from the sum of the influences of all of the metaball points.</ul>
/// * **BONE**
///     <ul>Line segments representing the object's skeleton. These are converted to bones for
///     deformation during rendering.</ul>
///
/// Each polygon is defined by a vertex count followed by a list of indexes into the most recent
/// PNTS chunk. The maximum number of vertices is 1023. The 6 high-order bits of the vertex count
/// are flag bits with different meanings for each polygon type. When reading POLS, remember to mask
/// out the flags to obtain numverts. (For CURV polygon: The two low order flags are for continuity
/// control point toggles. The four remaining high order flag bits are additional vertex count bits;
/// this brings the maximum number of vertices for CURV polygons to 2^14 = 16383.)
///
/// When writing POLS, the vertex list for each polygon should begin at a convex vertex and proceed
/// clockwise as seen from the visible side of the polygon. LightWave® polygons are single-sided
/// (although double-sidedness is a possible surface property), and the normal is defined as the
/// cross product of the first and last edges.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct PolygonLists {
    pub kind: [u8; 4],
    #[br(parse_with = until_size_limit(length as u64 - 4))]
    pub polygons: Vec<PolygonList>,
}

#[binread]
#[derive(Debug)]
pub struct PolygonList {
    #[br(temp)]
    pub numvert_and_flags: u16,
    #[br(calc = (numvert_and_flags >> 10) as u8)]
    pub flags: u8,
    #[br(parse_with = count_with_vx((numvert_and_flags & 0x3ff) as usize))]
    pub vert: Vec<u32>,
}
