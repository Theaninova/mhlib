use crate::lwo::intermediate_layer::IntermediateLayer;
use crate::lwo::mapping::find_mapping;
use crate::lwo::material::{MaterialProjectionMode, MaterialUvInfo};
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
        for inc in self.material_incomplete.into_iter().filter(|it| it != &0) {
            let percentage = (inc as f32 / self.vertices.len() as f32) * 100.0;
            godot_error!("{} ({}%) incomplete UVs", inc, percentage);
        }

        arrays
    }

    pub fn collect_from_layer(layer: &IntermediateLayer, material: &MaterialUvInfo) -> Self {
        let uv_names = [
            material.color_projection.as_ref(),
            material.diffuse_projection.as_ref(),
        ];

        let uv_subset = uv_names
            .iter()
            .map(|it| {
                it.and_then(|it| {
                    layer.uv_mappings.iter().find(|(name, _)|
                matches!(it, MaterialProjectionMode::UvChannelName(it) if name == it))
                })
            })
            .collect_vec();

        let mut surface_info = SurfaceInfo {
            uv_sets: uv_subset
                .iter()
                .map(|it| it.map(|_| PackedVector2Array::new()))
                .collect(),
            material_incomplete: uv_names.iter().map(|_| 0).collect_vec(),
            ..SurfaceInfo::default()
        };

        let surface_polygons =
            layer.polygons.iter().enumerate().filter(|(id, _)| {
                layer.material_mappings.get(&(*id as i32)).unwrap() == &material.id
            });

        for (id, poly) in surface_polygons {
            let tri = triangulate(&poly.vert);
            /*let edge0 = layer.points[tri[1] as usize] - layer.points[tri[0] as usize];
            let edge1 = layer.points[tri[tri.len() - 2] as usize]
                - layer.points[tri[tri.len() - 1] as usize];
            let normal = edge0.cross(edge1);*/

            for index in tri {
                let uv = uv_subset
                    .iter()
                    .map(|it| it.and_then(|(_, it)| find_mapping(it, id, index)))
                    .collect_vec();
                // TODO: let weight = find_mapping(weight_mappings, id, vert);
                let vert = layer.points[index as usize];

                surface_info.push_index(&vert, &uv, 0f32);
            }
        }

        surface_info
    }

    fn surface_normals(&mut self) {
        let normals: HashMap<i32, Vector3> = HashMap::with_capacity(self.vertices.len());
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
