use crate::pro::chunk::Chunk;
use crate::pro::internal::materials::PrMaterialChunks;
use crate::pro::internal::segment::PrSegmentChunks;
use crate::pro::internal::{PrChunks, PrObject};
use binrw::__private::magic;
use binrw::meta::{EndianKind, ReadEndian};
use binrw::BinRead;
use binrw::BinResult;
use binrw::Endian;
use std::fmt::Debug;
use std::io::{Read, Seek};

pub(crate) mod chunk;
pub(crate) mod internal;

#[derive(Default, Debug)]
pub struct PowerRenderObject {
    pub version: f32,
    pub name: String,
    pub segments: Vec<PowerRenderSegment>,
    pub materials: Vec<PowerRenderMaterial>,
    pub textures: Vec<PowerRenderTexture>,
}

#[derive(Default, Debug)]
pub struct PowerRenderSegment {
    pub name: String,
    pub surfaces: Vec<PowerRenderSurface>,
}

#[derive(Default, Debug)]
pub struct PowerRenderTexture {
    pub name: String,
    pub has_alpha: bool,
    pub is_stage: bool,
    pub is_multi: bool,
    pub format: u8,
}

#[derive(Debug)]
pub struct PowerRenderSurface {
    pub material_index: usize,
    pub back_material_index: usize,
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<i32>,
}

#[derive(Default, Debug)]
pub struct PowerRenderMaterial {
    pub name: String,
    pub texture_index: usize,
    pub two_sided: bool,
    pub frag_shader: Option<String>,
    pub vert_shader: Option<String>,
}

impl ReadEndian for PowerRenderObject {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl BinRead for PowerRenderObject {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _: Self::Args<'_>,
    ) -> BinResult<Self> {
        magic(reader, 0x0303u16, endian)?;
        let chunks = Chunk::<PrObject>::read(reader)?;

        let mut obj = PowerRenderObject::default();
        let mut texture_alpha = vec![];
        let mut texture_format = vec![];
        let mut texture_stage = vec![];
        let mut texture_multi = vec![];

        for chunk in chunks.0.chunks {
            match chunk {
                PrChunks::Version(version) => obj.version = version,
                PrChunks::ObjectName(name) => obj.name = name.to_string(),
                PrChunks::TextureAlpha(alpha) => texture_alpha = alpha,
                PrChunks::TextureFormat(format) => texture_format = format,
                PrChunks::TextureMulti(multi) => texture_multi = multi,
                PrChunks::TextureStage(stage) => texture_stage = stage,
                PrChunks::TextureList(texture_list) => {
                    let mut alpha = texture_alpha.iter();
                    let mut formats = texture_format.iter();
                    let mut stages = texture_stage.iter();
                    let mut multi = texture_multi.iter();

                    obj.textures = texture_list
                        .into_iter()
                        .map(|name| PowerRenderTexture {
                            name: name.to_string(),
                            has_alpha: *alpha.next().unwrap() != 0,
                            is_stage: *stages.next().unwrap() != 0,
                            is_multi: *multi.next().unwrap() != 0,
                            format: *formats.next().unwrap(),
                        })
                        .collect();
                }
                PrChunks::MaterialList(chunks) => {
                    for chunk in chunks {
                        match chunk {
                            PrMaterialChunks::MaterialEnd(_) => (),
                            PrMaterialChunks::MaterialName(name) => {
                                obj.materials.push(PowerRenderMaterial {
                                    name: name.to_string(),
                                    ..Default::default()
                                })
                            }
                            PrMaterialChunks::MaterialTexNum(id) => {
                                obj.materials.last_mut().unwrap().texture_index = id as usize;
                            }
                            PrMaterialChunks::MaterialTwoSided(val) => {
                                obj.materials.last_mut().unwrap().two_sided = val != 0
                            }
                            PrMaterialChunks::MaterialPixelShaderName(name) => {
                                if !name.is_empty() {
                                    obj.materials.last_mut().unwrap().frag_shader =
                                        Some(name.to_string())
                                }
                            }
                            PrMaterialChunks::MaterialVertexShaderName(name) => {
                                if !name.is_empty() {
                                    obj.materials.last_mut().unwrap().vert_shader =
                                        Some(name.to_string())
                                }
                            }
                            x => eprintln!("TODO (Materials): {:?}", x),
                        }
                    }
                }
                PrChunks::Segments(segments) => {
                    let mut vertices = None;
                    for chunk in segments {
                        match chunk {
                            PrSegmentChunks::SegmentName(name) => {
                                obj.segments.push(PowerRenderSegment {
                                    name: name.to_string(),
                                    ..Default::default()
                                });
                            }
                            PrSegmentChunks::Vertices(v) => {
                                vertices = Some(v);
                            }
                            PrSegmentChunks::Faces(faces) => {
                                let surfaces = vertices.unwrap().into_surfaces(faces);
                                vertices = None;
                                obj.segments.last_mut().unwrap().surfaces = surfaces;
                            }
                            x => eprintln!("TODO: {:?}", x),
                        }
                    }
                }
                x => eprintln!("TODO: {:?}", x),
            }
        }

        Ok(obj)
    }
}
