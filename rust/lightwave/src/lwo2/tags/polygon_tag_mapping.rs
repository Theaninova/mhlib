use crate::binrw_helpers::until_size_limit;
use crate::lwo2::vx;
use binrw::binread;

/// Associates tags of a given type with polygons in the most recent POLS chunk. The most common
/// polygon tag types are
///
/// * **SURF**:
///     <ul>The surface assigned to the polygon. The actual surface attributes are found by matching
///     the name in the TAGS chunk with the name in a SURF chunk.</ul>
/// * **PART**:
///     <ul>The part the polygon belongs to. Parts are named groups of polygons analogous to point
///     selection sets (but a polygon can belong to only one part).</ul>
/// * **SMGP**
///     <ul>The smoothing group the polygon belongs to. Shading is only interpolated within a
///       smoothing group, not across groups.</ul>
///
/// The polygon is identified by an index into the previous POLS chunk, and the tag is given by an
/// index into the previous TAGS chunk. Not all polygons will have a value for every tag type. The
/// behavior for polygons lacking a given tag depends on the type.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct PolygonTagMappings {
    pub kind: [u8; 4],
    #[br(parse_with = until_size_limit(length as u64 - 4))]
    pub mappings: Vec<PolygonTagMapping>,
}

#[binread]
#[derive(Debug)]
pub struct PolygonTagMapping {
    #[br(parse_with = vx)]
    pub poly: u32,
    pub tag: u16,
}
