use crate::lwo::clips::collect_clip;
use crate::lwo::intermediate_layer::IntermediateLayer;
use crate::lwo::mapping::{collect_discontinuous_mappings, collect_mappings};
use crate::lwo::material::MaterialUvInfo;
use godot::builtin::{Vector2, Vector3};
use godot::engine::node::InternalMode;
use godot::engine::{MeshInstance3D, Node3D, PackedScene, Texture2D};
use godot::log::{godot_error, godot_print, godot_warn};
use godot::obj::{Gd, Share};
use itertools::Itertools;
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;
use std::collections::HashMap;

pub fn lightwave_to_gd(lightwave: LightWaveObject) -> Gd<PackedScene> {
    let mut materials = HashMap::<u16, MaterialUvInfo>::new();
    let mut textures = HashMap::<u32, Gd<Texture2D>>::new();
    let mut layers = vec![];
    let mut tag_strings = vec![];

    for tag in lightwave.data {
        match tag {
            Tag::TagStrings(it) => {
                tag_strings = it.data.tag_strings.into_iter().collect();
                godot_print!("{:?}", tag_strings);
            }
            Tag::Layer(layer_tag) => {
                layers.push(IntermediateLayer {
                    name: if layer_tag.name.is_empty() {
                        format!("layer_{}", layer_tag.number)
                    } else {
                        layer_tag.name.clone()
                    },
                    parent: layer_tag.parent,
                    id: layer_tag.number,
                    pivot: Vector3 {
                        z: layer_tag.pivot[0],
                        y: layer_tag.pivot[1],
                        x: layer_tag.pivot[2],
                    },
                    ..IntermediateLayer::default()
                });
            }
            Tag::PointList(points_chunk) => {
                debug_assert_eq!(layers.last().unwrap().points.len(), 0);
                layers.last_mut().unwrap().points = points_chunk
                    .data
                    .point_location
                    .into_iter()
                    .map(|p| Vector3 {
                        z: p[0],
                        y: p[1],
                        x: p[2],
                    })
                    .collect();
            }
            Tag::DiscontinuousVertexMapping(vmad) => match &vmad.kind {
                b"TXUV" => {
                    debug_assert!(vmad.data.mappings[0].values.len() == 2);
                    let name = vmad.name.clone();

                    let layer = layers.last_mut().unwrap();
                    let map = if let Some(mappings) =
                        layer.uv_mappings.iter_mut().find(|(n, _)| n == &name)
                    {
                        mappings
                    } else {
                        layer.uv_mappings.push((name, HashMap::new()));
                        layer.uv_mappings.last_mut().unwrap()
                    };

                    collect_discontinuous_mappings(&mut map.1, vmad, |uv| Vector2 {
                        x: uv[0],
                        y: uv[1],
                    });
                }
                b"WGHT" => collect_discontinuous_mappings(
                    &mut layers.last_mut().unwrap().weight_mappings,
                    vmad,
                    |it| it[0],
                ),
                x => godot_error!(
                    "Not Implemented: Discontinuous Vertex Mapping: {}",
                    String::from_utf8(x.to_vec()).unwrap()
                ),
            },
            Tag::VertexMapping(vmap) => match &vmap.kind {
                b"TXUV" => {
                    debug_assert!(vmap.data.mapping[0].value.len() == 2);
                    let name = vmap.name.clone();

                    let layer = layers.last_mut().unwrap();
                    let map = if let Some(mappings) =
                        layer.uv_mappings.iter_mut().find(|(n, _)| n == &name)
                    {
                        mappings
                    } else {
                        layer.uv_mappings.push((name, HashMap::new()));
                        layer.uv_mappings.last_mut().unwrap()
                    };

                    collect_mappings(&mut map.1, vmap, |uv| Vector2 { x: uv[0], y: uv[1] });
                }
                b"WGHT" => collect_mappings(
                    &mut layers.last_mut().unwrap().weight_mappings,
                    vmap,
                    |it| it[0],
                ),
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
                    for surf in ptag.data.mappings {
                        layers
                            .last_mut()
                            .unwrap()
                            .material_mappings
                            .insert(surf.poly as i32, surf.tag);
                    }
                }
                x => godot_warn!(
                    "Polygon Tag Mapping: {}",
                    String::from_utf8(x.to_vec()).unwrap()
                ),
            },
            Tag::PolygonList(polygon_lists) => match &polygon_lists.kind {
                b"FACE" => {
                    debug_assert_eq!(layers.last().unwrap().polygons.len(), 0);
                    layers.last_mut().unwrap().polygons = polygon_lists.data.polygons;
                }
                x => godot_warn!("{}", String::from_utf8(x.to_vec()).unwrap()),
            },
            Tag::ImageClip(clip) => collect_clip(&mut textures, clip.data),
            Tag::SurfaceDefinition(surf) => {
                let surf_name = surf.name.clone();
                let (tag_index, _) = tag_strings
                    .iter()
                    .find_position(|name| name == &&surf_name)
                    .expect("Invalid File");
                godot_print!("'{}': {}", surf_name, tag_index);

                materials.insert(
                    tag_index as u16,
                    MaterialUvInfo::collect(surf.data, &textures, tag_index as u16),
                );
            }
            Tag::BoundingBox(_) => (),
            x => {
                godot_error!("Invalid chunk {:?}", x);
            }
        }
    }

    /*godot_print!(
        "{:?}",
        surfaces
            .iter()
            .map(|(k, v)| (
                k,
                tag_strings[*k as usize].clone(),
                v.material.get_shader_parameter("tex_diffuse".into()),
                v.material.get_shader_parameter("tex_color".into())
            ))
            .collect_vec()
    );*/

    let mut root_node = Node3D::new_alloc();

    for layer in layers {
        let mut instance = MeshInstance3D::new_alloc();
        instance.set_name(layer.name.clone().into());
        instance.set_mesh(layer.commit(&materials).upcast());

        root_node.add_child(
            instance.share().upcast(),
            false,
            InternalMode::INTERNAL_MODE_DISABLED,
        );
        instance.set_owner(root_node.share().upcast());
    }

    let mut scene = PackedScene::new();
    scene.pack(root_node.share().upcast());
    root_node.queue_free();
    scene
}
