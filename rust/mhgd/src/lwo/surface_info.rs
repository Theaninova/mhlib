use crate::lwo::intermediate_layer::IntermediateLayer;
use crate::lwo::mapping::find_mapping;
use crate::lwo::material::MaterialUvInfo;
use crate::lwo::unique_vertex::UniqueVertex;
use godot::builtin::{
    PackedFloat32Array, PackedInt32Array, PackedVector2Array, PackedVector3Array, ToVariant,
    VariantArray, Vector2, Vector3,
};
use godot::engine::mesh::ArrayType;
use godot::log::godot_error;
use godot::obj::EngineEnum;
use itertools::Itertools;
use std::collections::HashMap;

fn triangulate(poly: &[u32]) -> Vec<i32> {
    let mut poly = poly.iter().collect_vec();

    let fan_v = *poly.pop().unwrap() as i32;
    poly.reverse();
    poly.windows(2)
        .flat_map(move |w| [*w[1] as i32, *w[0] as i32, fan_v])
        .collect()
}

#[derive(Default)]
pub struct SurfaceInfo {
    vertex_map: HashMap<UniqueVertex, i32>,
    material_incomplete: Vec<i32>,
    pub material_id: u16,
    pub vertices: PackedVector3Array,
    pub uv_sets: Vec<Option<PackedVector2Array>>,
    pub weights: PackedFloat32Array,
    pub indices: PackedInt32Array,
}

impl SurfaceInfo {
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn commit_to_arrays(self) -> VariantArray {
        let mut arrays = VariantArray::new();
        arrays.resize(ArrayType::ARRAY_MAX.ord() as usize);
        arrays.set(
            ArrayType::ARRAY_VERTEX.ord() as usize,
            self.vertices.to_variant(),
        );
        for (i, uv) in self.uv_sets.into_iter().enumerate() {
            if let Some(uv) = uv {
                arrays.set(
                    match i {
                        0 => ArrayType::ARRAY_TEX_UV,
                        1 => ArrayType::ARRAY_TEX_UV2,
                        _ => todo!(),
                    }
                    .ord() as usize,
                    uv.to_variant(),
                )
            }
        }
        /*TODO: surface.set(
          ArrayType::ARRAY_WEIGHTS.ord() as usize,
          weights.to_variant(),
        );*/
        arrays.set(
            ArrayType::ARRAY_INDEX.ord() as usize,
            self.indices.to_variant(),
        );

        #[cfg(debug_assertions)]
        for (i, inc) in self.material_incomplete.into_iter().enumerate() {
            if inc != 0 {
                godot_error!(
                    "{} ({}%) incomplete UVs",
                    inc,
                    (inc as f32 / self.vertices.len() as f32) * 100.0
                );
            }
        }

        arrays
    }

    pub fn collect_from_layer(layer: &IntermediateLayer, material: &MaterialUvInfo) -> Self {
        let material_uv_names = [
            material.diffuse_channel.as_ref(),
            material.color_channel.as_ref(),
        ];

        let materials_subset = material_uv_names
            .iter()
            .map(|it| it.and_then(|it| layer.uv_mappings.iter().find(|(name, _)| name == it)))
            .collect_vec();

        let mut surface_info = SurfaceInfo {
            uv_sets: materials_subset
                .iter()
                .map(|it| it.map(|_| PackedVector2Array::new()))
                .collect(),
            material_incomplete: material_uv_names.iter().map(|_| 0).collect_vec(),
            ..SurfaceInfo::default()
        };

        let mut surface_polygons = layer.polygons.iter().enumerate().filter(|(id, _)| {
            layer.material_mappings.get(&(*id as i32)).unwrap_or(&0) == &material.id
        });

        for (id, poly) in surface_polygons {
            for index in triangulate(&poly.vert) {
                let uv = materials_subset
                    .iter()
                    .map(|it| it.and_then(|(_, it)| find_mapping(it, id, index)))
                    .collect_vec();
                // TODO: let weight = find_mapping(weight_mappings, id, vert);
                let vert = layer.points.get(index as usize).unwrap();

                surface_info.push_index(vert, &uv, 0f32);
            }
        }

        surface_info
    }

    fn push_index(&mut self, vert: &Vector3, uvs: &[Option<Vector2>], weight: f32) {
        let index = *self
            .vertex_map
            .entry(UniqueVertex::from_point(vert, uvs, 0f32, self.material_id))
            .or_insert_with(|| {
                self.vertices.push(*vert);
                for (i, uv) in uvs.iter().enumerate() {
                    if let Some(uv_set) = &mut self.uv_sets[i] {
                        uv_set.push(uv.unwrap_or_else(|| {
                            self.material_incomplete[i] += 1;
                            Vector2::ZERO
                        }));
                    }
                }
                self.weights.push(weight);
                self.vertices.len() as i32 - 1
            });

        self.indices.push(index);
    }
}
