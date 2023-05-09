use crate::binrw_helpers::until_size_limit;
use crate::lwo2::vx;
use binrw::{binread, NullString, PosValue};

/// Associates a set of floating-point vectors with a set of points. VMAPs begin with a type,
/// a dimension (vector length) and a name. These are followed by a list of vertex/vector pairs.
/// The vertex is given as an index into the most recent PNTS chunk, in VX format. The vector
/// contains dimension floating-point values. There can be any number of these chunks, but they
/// should all have different types or names.
///
/// Some common type codes are
///
/// * **PICK**
///      <ul>Selection set. This is a VMAP of dimension 0 that marks points for quick selection by
///     name during modeling. It has no effect on the geometry of the object.</ul>
/// * **WGHT**
///     <ul>Weight maps have a dimension of 1 and are generally used to alter the influence of
///     deformers such as bones. Weights can be positive or negative, and the default weight for
///     unmapped vertices is 0.0.</ul>
/// * **MNVW**
///     <ul>Subpatch weight maps affect the shape of geometry created by subdivision patching.</ul>
/// * **TXUV**
///     <ul>UV texture maps have a dimension of 2.</ul>
/// * **RGB**, **RGBA**
///      <ul>Color maps, with a dimension of 3 or 4.</ul>
/// * **MORF**
///      <ul>These contain vertex displacement deltas.</ul>
/// * **SPOT**
///     <ul>These contain absolute vertex displacements (alternative vertex positions).</ul>
///
/// Other widely used map types will almost certainly appear in the future.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct VertexMappings {
    #[br(temp)]
    pub begin_pos: PosValue<()>,
    pub kind: [u8; 4],
    #[br(temp)]
    pub dimension: u16,
    #[br(align_after = 2)]
    pub name: NullString,
    #[br(temp)]
    pub end_pos: PosValue<()>,
    #[br(parse_with = |reader, endian, _: ()| until_size_limit(length as u64 - (end_pos.pos - begin_pos.pos))(reader, endian, (dimension,)))]
    pub mapping: Vec<VertexMapping>,
}

#[binread]
#[br(import(dimension: u16))]
#[derive(Debug)]
pub struct VertexMapping {
    #[br(parse_with = vx)]
    pub vert: u32,
    #[br(count = dimension)]
    pub value: Vec<f32>,
}
