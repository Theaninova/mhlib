use godot::bind::godot_api;
use godot::builtin::{
    Dictionary, GodotString, PackedInt32Array, PackedVector2Array, PackedVector3Array,
    VariantArray, Vector2, Vector3,
};
use godot::engine::base_material_3d::TextureParam;
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::{load, ArrayMesh, Image, ImageTexture, StandardMaterial3D, SurfaceTool};
use godot::log::godot_print;
use godot::obj::{EngineEnum, Gd};
use godot::prelude::{godot_warn, Array, GodotClass, Share, ToVariant};
use lightwave_3d::iff::Chunk;
use lightwave_3d::lwo2::sub_tags::blocks::image_texture::SurfaceBlockImageTextureSubChunk;
use lightwave_3d::lwo2::sub_tags::blocks::{
    SurfaceBlockHeaderSubChunk, SurfaceBlocks, TextureChannel,
};
use lightwave_3d::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use lightwave_3d::lwo2::tags::image_clip::{ImageClip, ImageClipSubChunk};
use lightwave_3d::lwo2::tags::point_list::PointList;
use lightwave_3d::lwo2::tags::polygon_list::PolygonLists;
use lightwave_3d::lwo2::tags::surface_definition::SurfaceDefinition;
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;

#[derive(GodotClass)]
#[class(init)]
struct Lwo {}

#[godot_api]
impl Lwo {
    #[func]
    pub fn get_mesh(path: GodotString) -> Gd<ArrayMesh> {
        lightwave_to_gd(LightWaveObject::read_file(path.to_string()).unwrap())
    }
}

pub fn lightwave_to_gd(lightwave: LightWaveObject) -> Gd<ArrayMesh> {
    let mut mesh = ArrayMesh::new();

    let mut arrays: Option<VariantArray> = None;
    let mut materials = vec![];
    let mut images: Option<ImageClip> = None;

    let mut vert_count = 0;

    for tag in lightwave.data {
        match tag {
            Tag::PointList(points) => {
                try_commit(&mut mesh, &arrays);
                vert_count = points.point_location.len();

                let mut ars = Array::new();
                ars.resize(ArrayType::ARRAY_MAX.ord() as usize);

                ars.set(
                    ArrayType::ARRAY_VERTEX.ord() as usize,
                    collect_points(points).to_variant(),
                );
                arrays = Some(ars);
            }
            Tag::DiscontinuousVertexMapping(vmad) => match &vmad.kind {
                b"TXUV" => {
                    collect_uvs(
                        vmad.data.mappings.into_iter().map(|it| {
                            (
                                it.vert as usize,
                                Vector2 {
                                    x: it.values[0],
                                    y: it.values[1],
                                },
                            )
                        }),
                        vert_count,
                        arrays.as_mut().unwrap(),
                    );
                }
                x => godot_warn!(
                    "Discontinuous Vertex Mapping: {}",
                    String::from_utf8(x.to_vec()).unwrap()
                ),
            },
            Tag::VertexMapping(vmap) => match &vmap.kind {
                b"TXUV" => {
                    /*collect_uvs(
                        vmap.data.mapping.into_iter().map(|it| {
                            (
                                it.vert as usize,
                                Vector2 {
                                    x: it.value[0],
                                    y: it.value[1],
                                },
                            )
                        }),
                        vert_count,
                        arrays.as_mut().unwrap(),
                    );*/
                }
                x => godot_warn!("Vertex Mapping: {}", String::from_utf8(x.to_vec()).unwrap()),
            },
            Tag::PolygonList(polygons) => match &polygons.kind {
                b"FACE" => {
                    if let Some(arrays) = &mut arrays {
                        let indices = collect_polygons(polygons);
                        arrays.set(ArrayType::ARRAY_INDEX.ord() as usize, indices.to_variant());
                    }
                }
                x => godot_warn!("{}", String::from_utf8(x.to_vec()).unwrap()),
            },
            Tag::ImageClip(clip) => {
                images = Some(clip.data);
            }
            Tag::SurfaceDefinition(surf) => {
                let mat = collect_material(surf.data, images.unwrap());
                images = None;
                materials.push(mat);
            }
            x => {
                godot_warn!("Invalid chunk {:?}", x);
            }
        }
    }

    try_commit(&mut mesh, &arrays);
    let mut out_mesh = ArrayMesh::new();
    let mut mats = materials.into_iter();
    for i in 0..mesh.get_surface_count() {
        let mut tool = SurfaceTool::new();
        tool.create_from(mesh.share().upcast(), i);
        tool.generate_normals(false);
        tool.generate_tangents();
        try_commit(&mut out_mesh, &Some(tool.commit_to_arrays()));
        if let Some(mat) = mats.next() {
            out_mesh.surface_set_material(i, mat.upcast())
        }
    }
    out_mesh
}

fn try_commit(mesh: &mut ArrayMesh, arrays: &Option<VariantArray>) {
    if let Some(arrays) = arrays {
        mesh.add_surface_from_arrays(
            PrimitiveType::PRIMITIVE_TRIANGLES,
            arrays.share(),
            Array::new(),
            Dictionary::new(),
            ArrayFormat::ARRAY_FORMAT_NORMAL,
        );
    }
}

fn collect_uvs<I: Iterator<Item = (usize, Vector2)>>(
    mapping: I,
    vert_count: usize,
    arrays: &mut VariantArray,
) {
    let mut arr = arrays
        .get(ArrayType::ARRAY_TEX_UV.ord() as usize)
        .try_to::<PackedVector2Array>()
        .unwrap_or_else(|_| {
            let mut new_uvs = PackedVector2Array::new();
            new_uvs.resize(vert_count);
            new_uvs
        });

    for (index, uv) in mapping {
        arr.set(index, uv);
    }

    arrays.set(ArrayType::ARRAY_TEX_UV.ord() as usize, arr.to_variant());
}

fn collect_points(chunk: Chunk<PointList>) -> PackedVector3Array {
    PackedVector3Array::from(
        chunk
            .data
            .point_location
            .into_iter()
            .map(|[x, y, z]| Vector3 { x, y, z })
            .collect::<Vec<Vector3>>()
            .as_slice(),
    )
}

fn collect_material(surface: SurfaceDefinition, clip: ImageClip) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new();
    material.set_name(surface.name.to_string().into());

    let mut i: Option<Gd<Image>> = None;
    for img in clip.attributes {
        match img {
            ImageClipSubChunk::StillImage(still) => {
                let path = format!(
                    "sar://{}",
                    still.name.to_string().replace('\\', "/").replace(':', ":/")
                );
                godot_print!("Loading {}", &path);
                i = Some(load(path));
            }
            x => {
                godot_warn!("Invalid clip chunk {:?}", x)
            }
        }
    }

    for attr in surface.attributes {
        match attr {
            SurfaceParameterSubChunk::Blocks(blocks) => {
                if let SurfaceBlocks::ImageMapTexture { header, attributes } = blocks.data {
                    let mut texture = ImageTexture::new();
                    let mut chan = TextureParam::TEXTURE_ALBEDO;
                    for attr in header.data.block_attributes {
                        match attr {
                            SurfaceBlockHeaderSubChunk::Channel(c) => {
                                chan = match c.data.texture_channel {
                                    TextureChannel::Color => TextureParam::TEXTURE_ALBEDO,
                                    TextureChannel::Diffuse => TextureParam::TEXTURE_ALBEDO,
                                    TextureChannel::Bump => TextureParam::TEXTURE_HEIGHTMAP,
                                    TextureChannel::RefractiveIndex => {
                                        TextureParam::TEXTURE_REFRACTION
                                    }
                                    TextureChannel::Specular => TextureParam::TEXTURE_METALLIC,
                                    TextureChannel::Glossy => TextureParam::TEXTURE_ROUGHNESS,
                                    x => {
                                        godot_warn!("Invalid channel {:?}", x);
                                        TextureParam::TEXTURE_ORM
                                    }
                                }
                            }
                            x => {
                                godot_warn!("Invalid surface header chunk {:?}", x)
                            }
                        }
                    }
                    for attr in attributes {
                        match attr {
                            SurfaceBlockImageTextureSubChunk::ImageMap(r) => {
                                texture.set_image(i.unwrap());
                                i = None;
                            }
                            x => {
                                godot_warn!("Invalid image texture chunk {:?}", x)
                            }
                        }
                    }
                    material.set_texture(chan, texture.upcast());
                }
            }
            x => {
                godot_warn!("Invalid Surface Chunk {:?}", x)
            }
        }
    }

    material
}

fn collect_polygons(chunk: Chunk<PolygonLists>) -> PackedInt32Array {
    debug_assert!(chunk.polygons.len() >= 3, "{:?}", chunk);
    PackedInt32Array::from(
        chunk
            .data
            .polygons
            .into_iter()
            .flat_map(|mut it| {
                let fan_v = it.vert.remove(0) as i32;
                it.vert
                    .windows(2)
                    .flat_map(|w| [w[1] as i32, w[0] as i32, fan_v])
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<i32>>()
            .as_slice(),
    )
}
