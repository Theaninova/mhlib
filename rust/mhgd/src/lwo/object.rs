use crate::lwo::clips::collect_clip;
use crate::lwo::mapping::{collect_discontinuous_mappings, collect_mappings};
use crate::lwo::material::collect_material;
use crate::lwo::surface::try_commit;
use godot::builtin::{Array, Dictionary, Vector2, Vector3};
use godot::engine::mesh::{ArrayFormat, PrimitiveType};
use godot::engine::{ArrayMesh, Image, SurfaceTool};
use godot::log::{godot_error, godot_warn};
use godot::obj::{Gd, Share};
use lightwave_3d::lwo2::tags::polygon_list::PolygonList;
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;
use std::collections::{HashMap, HashSet};

pub fn lightwave_to_gd(lightwave: LightWaveObject) -> Gd<ArrayMesh> {
    let mut mesh = ArrayMesh::new();

    let mut materials = vec![];
    let mut images = HashMap::<u32, Gd<Image>>::new();

    let mut points = Vec::<Vector3>::new();
    let mut uv_mappings = HashMap::<i32, HashMap<i32, Vector2>>::new();
    let mut weight_mappings = HashMap::<i32, HashMap<i32, f32>>::new();
    let mut polygons = Vec::<PolygonList>::new();
    let mut surfaces = HashMap::<i32, u16>::new();

    let mut surface_materials = Vec::<u16>::new();

    for tag in lightwave.data {
        match tag {
            Tag::Layer(layer) => {
                try_commit(
                    &mut mesh,
                    &mut points,
                    &mut uv_mappings,
                    &mut weight_mappings,
                    &mut polygons,
                    &mut surfaces,
                    &mut surface_materials,
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
                    for surf in ptag.data.mappings {
                        surfaces.insert(surf.poly as i32, surf.tag);
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
            Tag::ImageClip(clip) => collect_clip(&mut images, clip.data),
            Tag::SurfaceDefinition(surf) => {
                let mat = collect_material(surf.data, &images);
                materials.push(mat);
            }
            Tag::BoundingBox(_) => (),
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
        &mut surfaces,
        &mut surface_materials,
    );
    let mut out_mesh = ArrayMesh::new();
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

        if let Some(mat) = materials.get(surfaces[&(i as i32)] as usize) {
            out_mesh.surface_set_material(i, mat.share().upcast())
        }
    }
    out_mesh
}
