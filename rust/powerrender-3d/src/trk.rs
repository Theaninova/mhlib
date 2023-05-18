use binrw::{binrw, NullString};

#[binrw]
#[br(magic = b"TRK.V07\0")]
#[derive(Debug)]
pub struct Trk {
    #[br(temp)]
    #[bw(calc = chunks.len() as u32)]
    chunk_count: u32,
    pub start_chunk: u16,
    #[br(count = chunk_count)]
    pub chunks: Vec<Chunk>,
    #[br(temp)]
    #[bw(calc = objects.len() as u32)]
    object_count: u32,
    #[br(count = object_count)]
    pub objects: Vec<Object>,
    #[br(temp)]
    #[bw(calc = clusters.len() as u32)]
    cluster_count: u32,
    #[br(count = cluster_count)]
    pub clusters: Vec<Cluster>,
}

#[binrw]
#[derive(Debug)]
pub struct Chunk {
    pub id: u16,
    pub flags: u16,
    pub bbox: [f32; 4],
    pub adjacent_chunks: [i16; 4],
    pub offset: [f32; 4],
}

#[binrw]
#[derive(Debug)]
pub struct Object {
    pub name: NullString,
    pub position: [f32; 3],
}

#[binrw]
#[derive(Debug)]
pub struct Cluster {
    pub name: NullString,
    #[br(temp)]
    #[bw(calc = values.len() as u32)]
    value_count: u32,
    #[br(count = value_count)]
    pub values: Vec<[f32; 4]>,
}
