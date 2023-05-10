use godot::bind::godot_api;
use godot::builtin::{GodotString, PackedInt32Array, PackedVector3Array, Vector3};
use godot::engine::mesh::ArrayType;
use godot::engine::{ArrayMesh, PackedScene};
use godot::obj::{EngineEnum, Gd};
use godot::prelude::{Array, GodotClass, ToVariant};
use lightwave_3d::lwo2::tags::Tag;
use lightwave_3d::LightWaveObject;
use std::fs::File;

#[derive(GodotClass)]
struct Lwo {}

#[godot_api]
impl Lwo {
    pub fn get_mesh(path: GodotString) -> Gd<ArrayMesh> {
        lightwave_to_gd(LightWaveObject::read_file(path.to_string()).unwrap())
    }
}

pub fn lightwave_to_gd(lightwave: LightWaveObject) -> Gd<ArrayMesh> {
    let mesh = ArrayMesh::new();
    let mut arrays = Array::new();
    arrays.resize(ArrayType::ARRAY_MAX.ord() as usize);

    for tag in lightwave.data {
        match tag {
            Tag::PointList(points) => {
                arrays.set(
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
            }
            Tag::PolygonList(polygons) => match &polygons.kind {
                b"FACE" => {
                    arrays.set(
                        ArrayType::ARRAY_INDEX.ord() as usize,
                        PackedInt32Array::from(
                            polygons
                                .polygons
                                .iter()
                                .flat_map(|it| it.vert.iter().map(|it| *it as i32))
                                .collect::<Vec<i32>>()
                                .as_slice(),
                        )
                        .to_variant(),
                    );
                }
                _ => panic!(),
            },
            _ => (),
        }
    }

    mesh
}
