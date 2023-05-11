use godot::bind::godot_api;
use godot::builtin::{
    Dictionary, GodotString, PackedInt32Array, PackedVector2Array, PackedVector3Array,
    VariantArray, Vector2, Vector3,
};
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::{ArrayMesh, PackedScene};
use godot::obj::{EngineEnum, Gd};
use godot::prelude::{Array, GodotClass, Share, ToVariant};
use itertools::Itertools;
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;
use std::fs::File;

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

    for tag in lightwave.data {
        match tag {
            Tag::PointList(points) => {
                if let Some(arrays) = &arrays {
                    mesh.add_surface_from_arrays(
                        PrimitiveType::PRIMITIVE_TRIANGLES,
                        arrays.share(),
                        Array::new(),
                        Dictionary::new(),
                        ArrayFormat::ARRAY_FORMAT_NORMAL,
                    );
                }
                let mut ars = Array::new();
                ars.resize(ArrayType::ARRAY_MAX.ord() as usize);
                let mut norm = PackedVector3Array::new();
                norm.resize(points.point_location.len());
                ars.set(ArrayType::ARRAY_NORMAL.ord() as usize, norm.to_variant());
                ars.set(
                    ArrayType::ARRAY_VERTEX.ord() as usize,
                    PackedVector3Array::from(
                        points
                            .point_location
                            .iter()
                            .map(|[x, y, z]| Vector3 {
                                x: *x,
                                y: *y,
                                z: *z,
                            })
                            .collect::<Vec<Vector3>>()
                            .as_slice(),
                    )
                    .to_variant(),
                );
                arrays = Some(ars);
            }
            Tag::VertexMapping(vmap) => {
                if let b"TXUV" = &vmap.kind {
                    if let Some(arrays) = &mut arrays {
                        arrays.set(
                            ArrayType::ARRAY_TEX_UV.ord() as usize,
                            PackedVector2Array::from(
                                vmap.mapping
                                    .iter()
                                    .map(|uv| Vector2 {
                                        x: uv.value[0],
                                        y: uv.value[1],
                                    })
                                    .collect::<Vec<Vector2>>()
                                    .as_slice(),
                            )
                            .to_variant(),
                        );
                    }
                }
            }
            Tag::PolygonList(polygons) => match &polygons.kind {
                b"FACE" => {
                    if let Some(arrays) = &mut arrays {
                        arrays.set(
                            ArrayType::ARRAY_INDEX.ord() as usize,
                            PackedInt32Array::from(
                                get_rendering_vertex_indices(
                                    polygons
                                        .polygons
                                        .iter()
                                        .flat_map(|it| it.vert.iter().map(|it| *it as i32))
                                        .collect::<Vec<i32>>(),
                                )
                                .as_slice(),
                            )
                            .to_variant(),
                        );
                    }
                }
                _ => panic!(),
            },
            _ => (),
        }
    }

    if let Some(arrays) = &arrays {
        mesh.add_surface_from_arrays(
            PrimitiveType::PRIMITIVE_TRIANGLES,
            arrays.share(),
            Array::new(),
            Dictionary::new(),
            ArrayFormat::ARRAY_FORMAT_VERTEX,
        );
    }

    mesh.regen_normal_maps();

    mesh
}

fn get_rendering_vertex_indices(strip: Vec<i32>) -> Vec<i32> {
    if strip.len() == 2 {
        return vec![strip[0], strip[1], strip[0]];
    }

    let mut p = strip.into_iter();
    let mut vertex_indices = vec![];

    let mut f1 = p.next().unwrap();
    let mut f2 = p.next().unwrap();
    let mut face_direction = 1;
    for f3 in p {
        // face_direction *= -1;
        if f1 != f2 && f2 != f3 && f3 != f1 {
            if face_direction > 0 {
                vertex_indices.push(f3);
                vertex_indices.push(f2);
                vertex_indices.push(f1);
            } else {
                vertex_indices.push(f2);
                vertex_indices.push(f3);
                vertex_indices.push(f1);
            }
        }

        f1 = f2;
        f2 = f3;
    }

    vertex_indices
}
