use crate::lwo::mapping::find_mapping;
use godot::builtin::{
    Array, Dictionary, PackedFloat32Array, PackedInt32Array, PackedVector2Array,
    PackedVector3Array, ToVariant, VariantArray, Vector2, Vector3,
};
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::ArrayMesh;
use godot::obj::EngineEnum;
use lightwave_3d::lwo2::tags::polygon_list::PolygonList;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct UniqueVertex {
    vert: i32,
    uv: [[u8; 4]; 2],
    weight: [u8; 4],
}

pub fn try_commit(
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
