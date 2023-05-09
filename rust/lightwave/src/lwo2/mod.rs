use binrw::{BinReaderExt, BinResult, Endian};
use std::io::{Read, Seek};

pub mod sub_tags;
pub mod tags;

/// This is an index into an array of items (points or polygons), or a collection of items
/// each uniquely identified by an integer (clips or envelopes). A VX is written as a variable
/// length 2- or 4-byte element. If the index value is less than 65,280 (0xFF00), then the
/// index is written as an unsigned two-byte integer. Otherwise the index is written as an
/// unsigned four byte integer with bits 24-31 set. When reading an index, if the first byte
/// encountered is 255 (0xFF), then the four-byte form is being used and the first byte should
/// be discarded or masked out.
pub fn vx<R>(reader: &mut R, endian: Endian, _args: ()) -> BinResult<u32>
where
    R: Read + Seek,
{
    let kind: u16 = reader.read_type(endian)?;
    Ok(if kind < 0xff00 {
        kind as u32
    } else {
        (((kind as u32) & 0xff) << 16) | (reader.read_type::<u16>(endian)? as u32)
    })
}
