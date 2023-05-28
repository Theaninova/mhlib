use godot::bind::godot_api;
use godot::builtin::{
    Dictionary, GodotString, PackedInt32Array, PackedVector2Array, PackedVector3Array,
    VariantArray, Vector2, Vector3,
};
use godot::engine::base_material_3d::TextureParam;
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::{load, ArrayMesh, Image, ImageTexture, StandardMaterial3D, SurfaceTool};
use godot::log::{godot_error, godot_print};
use godot::obj::{EngineEnum, Gd};
use godot::prelude::{godot_warn, Array, GodotClass, PackedFloat32Array, Share, ToVariant};
use lightwave_3d::iff::Chunk;
use lightwave_3d::lwo2::sub_tags::blocks::image_texture::SurfaceBlockImageTextureSubChunk;
use lightwave_3d::lwo2::sub_tags::blocks::{
    SurfaceBlockHeaderSubChunk, SurfaceBlocks, TextureChannel,
};
use lightwave_3d::lwo2::sub_tags::surface_parameters::SurfaceParameterSubChunk;
use lightwave_3d::lwo2::tags::discontinuous_vertex_mapping::DiscontinuousVertexMappings;
use lightwave_3d::lwo2::tags::image_clip::{ImageClip, ImageClipSubChunk};
use lightwave_3d::lwo2::tags::polygon_list::PolygonList;
use lightwave_3d::lwo2::tags::surface_definition::SurfaceDefinition;
use lightwave_3d::lwo2::tags::vertex_mapping::VertexMappings;
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;
use std::collections::{HashMap, HashSet};

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

    let mut materials = vec![];
    let mut images: Option<ImageClip> = None;

    let mut points = Vec::<Vector3>::new();
    let mut uv_mappings = HashMap::<i32, HashMap<i32, Vector2>>::new();
    let mut weight_mappings = HashMap::<i32, HashMap<i32, f32>>::new();
    let mut polygons = Vec::<PolygonList>::new();

    for tag in lightwave.data {
        match tag {
            Tag::Layer(layer) => {
                try_commit(
                    &mut mesh,
                    &mut points,
                    &mut uv_mappings,
                    &mut weight_mappings,
                    &mut polygons,
                );
            }
            Tag::PointList(points_chunk) => {
                points = points_chunk
                    .data
                    .point_location
                    .into_iter()
                    .map(|p| Vector3 {
                        x: p[0],
                        y: p[1],
                        z: p[2],
                    })
                    .collect();
            }
            Tag::DiscontinuousVertexMapping(vmad) => match &vmad.kind {
                b"TXUV" => {
                    debug_assert!(vmad.data.mappings[0].values.len() == 2);
                    collect_discontinuous_mappings(&mut uv_mappings, vmad, |uv| Vector2 {
                        x: uv[0],
                        y: uv[1],
                    })
                }
                b"WGHT" => collect_discontinuous_mappings(&mut weight_mappings, vmad, |it| it[0]),
                x => godot_error!(
                    "Not Implemented: Discontinuous Vertex Mapping: {}",
                    String::from_utf8(x.to_vec()).unwrap()
                ),
            },
            Tag::VertexMapping(vmap) => match &vmap.kind {
                b"TXUV" => {
                    debug_assert!(vmap.data.mapping[0].value.len() == 2);
                    collect_mappings(&mut uv_mappings, vmap, |uv| Vector2 { x: uv[0], y: uv[1] })
                }
                b"WGHT" => collect_mappings(&mut weight_mappings, vmap, |it| it[0]),
                x => godot_error!(
                    "Not Implemented: Vertex Mapping: {}",
                    String::from_utf8(x.to_vec()).unwrap()
                ),
            },
            Tag::PolygonTagMapping(ptag) => match &ptag.kind {
                /*b"COLR" => {
                    todo!();
                },*/
                b"SURF" => {
                    let surfaces = ptag
                        .data
                        .mappings
                        .iter()
                        .map(|map| map.tag)
                        .collect::<HashSet<u16>>();
                    if surfaces.len() > 1 {
                        godot_error!("Too many surfaces: {:?}", surfaces)
                    }
                }
                x => godot_warn!(
                    "Polygon Tag Mapping: {}",
                    String::from_utf8(x.to_vec()).unwrap()
                ),
            },
            Tag::PolygonList(polygon_lists) => match &polygon_lists.kind {
                b"FACE" => {
                    polygons = polygon_lists.data.polygons;
                }
                x => godot_warn!("{}", String::from_utf8(x.to_vec()).unwrap()),
            },
            Tag::ImageClip(clip) => {
                images = Some(clip.data);
            }
            Tag::SurfaceDefinition(surf) => {
                if let Some(img) = images {
                    let mat = collect_material(surf.data, img);
                    images = None;
                    materials.push(mat);
                } else {
                    godot_error!("Missing images for surface {}", surf.name)
                }
            }
            x => {
                godot_error!("Invalid chunk {:?}", x);
            }
        }
    }

    try_commit(
        &mut mesh,
        &mut points,
        &mut uv_mappings,
        &mut weight_mappings,
        &mut polygons,
    );
    let mut out_mesh = ArrayMesh::new();
    let mut mats = materials.into_iter();
    for i in 0..mesh.get_surface_count() {
        let mut tool = SurfaceTool::new();

        tool.create_from(mesh.share().upcast(), i);
        tool.generate_normals(false);
        tool.generate_tangents();

        out_mesh.add_surface_from_arrays(
            PrimitiveType::PRIMITIVE_TRIANGLES,
            tool.commit_to_arrays(),
            Array::new(),
            Dictionary::new(),
            ArrayFormat::ARRAY_FORMAT_NORMAL,
        );

        if let Some(mat) = mats.next() {
            out_mesh.surface_set_material(i, mat.upcast())
        }
    }
    out_mesh
}

#[derive(Hash, Eq, PartialEq)]
struct UniqueVertex {
    vert: i32,
    uv: [[u8; 4]; 2],
    weight: [u8; 4],
}

fn try_commit(
    mesh: &mut ArrayMesh,
    points: &mut [Vector3],
    uv_mappings: &mut HashMap<i32, HashMap<i32, Vector2>>,
    weight_mappings: &mut HashMap<i32, HashMap<i32, f32>>,
    polygons: &mut Vec<PolygonList>,
) {
    if polygons.is_empty() {
        return;
    }

    let mut vertex_map = HashMap::<UniqueVertex, i32>::new();
    let mut vertices = PackedVector3Array::new();
    let mut uvs = PackedVector2Array::new();
    let mut weights = PackedFloat32Array::new();
    let mut indices = PackedInt32Array::new();

    for (id, poly) in polygons.iter_mut().enumerate() {
        let fan_v = poly.vert.remove(0) as i32;
        let fan = poly
            .vert
            .windows(2)
            .flat_map(|w| [w[1] as i32, w[0] as i32, fan_v]);

        for vert in fan {
            let uv = find_mapping(uv_mappings, id, vert);
            // let weight = find_mapping(weight_mappings, id, vert);

            indices.push(
                *vertex_map
                    .entry(UniqueVertex {
                        vert,
                        uv: [uv.x.to_ne_bytes(), uv.y.to_ne_bytes()],
                        weight: (0.0f32).to_ne_bytes(),
                    })
                    .or_insert_with(|| {
                        vertices.push(*points.get(vert as usize).unwrap());
                        uvs.push(uv);
                        weights.push(0.0);
                        vertices.len() as i32 - 1
                    }),
            );
        }
    }

    let mut surface = VariantArray::new();
    surface.resize(ArrayType::ARRAY_MAX.ord() as usize);
    surface.set(
        ArrayType::ARRAY_VERTEX.ord() as usize,
        vertices.to_variant(),
    );
    surface.set(ArrayType::ARRAY_TEX_UV.ord() as usize, uvs.to_variant());
    /*TODO: surface.set(
        ArrayType::ARRAY_WEIGHTS.ord() as usize,
        weights.to_variant(),
    );*/
    surface.set(ArrayType::ARRAY_INDEX.ord() as usize, indices.to_variant());

    mesh.add_surface_from_arrays(
        PrimitiveType::PRIMITIVE_TRIANGLES,
        surface,
        Array::new(),
        Dictionary::new(),
        ArrayFormat::ARRAY_FORMAT_NORMAL,
    );
}

fn find_mapping<T: Default + Copy + std::fmt::Debug>(
    target: &HashMap<i32, HashMap<i32, T>>,
    poly: usize,
    vert: i32,
) -> T {
    target
        .get(&(poly as i32))
        .and_then(|mapping| mapping.get(&vert).copied())
        .or_else(|| {
            target
                .get(&-1)
                .and_then(|mapping| mapping.get(&vert).copied())
        })
        .unwrap_or_else(|| {
            godot_error!(
                "Missing VX Mapping for {{vert: {}, poly: {}}}; {:?}",
                vert,
                poly,
                target
                    .get(&(poly as i32))
                    .map(|p| p.keys().collect::<Vec<&i32>>())
            );
            T::default()
        })
}

fn collect_discontinuous_mappings<T>(
    target: &mut HashMap<i32, HashMap<i32, T>>,
    vmap: Chunk<DiscontinuousVertexMappings>,
    map_fn: fn(Vec<f32>) -> T,
) {
    for mapping in vmap.data.mappings {
        target
            .entry(mapping.poly as i32)
            .or_insert_with(|| HashMap::new())
            .insert(mapping.vert as i32, map_fn(mapping.values));
    }
}

fn collect_mappings<T>(
    target: &mut HashMap<i32, HashMap<i32, T>>,
    vmap: Chunk<VertexMappings>,
    map_fn: fn(Vec<f32>) -> T,
) {
    let entry = target.entry(-1).or_insert_with(|| HashMap::new());
    for mapping in vmap.data.mapping {
        entry.insert(mapping.vert as i32, map_fn(mapping.value));
    }
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
                                if let Some(i) = i {
                                    texture.set_image(i);
                                } else {
                                    godot_error!("Missing texture {:?}", r);
                                }
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
