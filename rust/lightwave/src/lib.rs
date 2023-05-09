use crate::lwo2::tags::Tag;
use binrw::{binread, until_eof, BinRead, BinResult};
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

mod binrw_helpers;
pub mod iff;
pub mod lwo2;

/// The data in LightWave 3D® object files comprise the points, polygons and surfaces that describe
/// the geometry and appearance of an object. "Polygons" here means any of several geometric
/// elements (faces, curves or patches, for example) defined by an ordered list of points, and
/// "surfaces" refers to the collection of attributes, sometimes called materials, that define
/// the visual surface properties of polygons.
///
/// Object files can contain multiple layers, or parts, and each part can be a single connected mesh
/// or several disjoint meshes. They may also contain one or more surface definitions with no points
/// or polygons at all. Surface definitions can include references to other files (images, for
/// example), plug-ins, and envelopes containing parameter values that vary over time.
///
/// This document outlines the object file format and provides a detailed reference for each of the
/// components. The component descriptions include both a regular expression defining the syntax and
/// a discussion of the contents. See also the Examples supplement, a more conversational
/// introduction to the format that includes annotated listings of file contents as well as
/// several sample files.
/// Informally, object files start with the four bytes "FORM" followed by a four-byte integer giving
/// the length of the file (minus 8) and the four byte ID "LWO2". The remainder of the data is a
/// collection of chunks, some of which will contain subchunks.
///
/// To be read, IFF files must be parsed. The order in which chunks can occur in a file isn't fixed.
/// Some chunks, however, contain data that depends on the contents of other chunks, and this fixes
/// a relative order for the chunks involved. Chunks and subchunks also depend on context for their
/// meaning. The CHAN subchunk in an envelope chunk isn't the same thing as the CHAN subchunk in a
/// surface block. And you may encounter chunks that aren't defined here, which you should be
/// prepared to skip gracefully if you don't understand them. You can do this by using the chunk
/// size to seek to the next chunk.
#[binread]
#[br(big, magic(b"FORM"))]
#[derive(Debug)]
pub struct LightWaveObject {
    pub file_size: u32,
    #[br(magic(b"LWO2"), parse_with = until_eof)]
    pub data: Vec<Tag>,
}

impl LightWaveObject {
    pub fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<LightWaveObject> {
        let mut reader = File::open(path)?;
        Self::read(&mut reader)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
    }

    pub fn read<R>(reader: &mut R) -> BinResult<LightWaveObject>
    where
        R: Read + Seek,
    {
        BinRead::read(reader)
    }
}
