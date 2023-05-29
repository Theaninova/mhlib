use crate::lwo::mapping::find_mapping;
use crate::lwo::material::MaterialUvInfo;
use godot::builtin::{
    Array, Dictionary, PackedFloat32Array, PackedInt32Array, PackedVector2Array,
    PackedVector3Array, ToVariant, VariantArray, Vector2, Vector3,
};
use godot::engine::mesh::{ArrayFormat, ArrayType, PrimitiveType};
use godot::engine::{ArrayMesh, SurfaceTool};
use godot::obj::{EngineEnum, Gd, Share};
use godot::prelude::godot_error;
use itertools::Itertools;
use lightwave_3d::lwo2::tags::polygon_list::PolygonList;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct UniqueVertex {
    vert: i32,
    uv: Vec<Option<[[u8; 4]; 2]>>,
    weight: [u8; 4],
}

pub type SurfaceMapping<T> = HashMap<i32, HashMap<i32, T>>;

#[derive(Default)]
pub struct IntermediateLayer {
    pub name: String,
    pub pivot: Vector3,
    pub parent: Option<u16>,
    pub id: u16,
    pub points: Vec<Vector3>,
    pub uv_mappings: Vec<(String, SurfaceMapping<Vector2>)>,
    pub weight_mappings: SurfaceMapping<f32>,
    pub polygons: Vec<PolygonList>,
    pub surfaces: HashMap<i32, u16>,
}

impl IntermediateLayer {
    pub fn commit(mut self, materials: &[MaterialUvInfo]) -> Gd<ArrayMesh> {
        let mut mesh = ArrayMesh::new();
        mesh.set_name(self.name.clone().into());
        let mut surface_materials = Vec::<u16>::new();

        self.uv_mappings.sort_by(|a, b| a.0.cmp(&b.0));

        for surface_id in self.surfaces.values().unique() {
            let material = &materials[*surface_id as usize];
            let material_uv_names = [
                material.diffuse_channel.as_ref(),
                material.color_channel.as_ref(),
            ];
            let mut material_incomplete = material_uv_names.iter().map(|_| 0).collect_vec();
            let materials_subset = material_uv_names
                .iter()
                .map(|it| it.and_then(|it| self.uv_mappings.iter().find(|(name, _)| name == it)))
                .collect_vec();

            let mut vertex_map = HashMap::<UniqueVertex, i32>::new();
            let mut vertices = PackedVector3Array::new();
            let mut uvs = materials_subset
                .iter()
                .map(|it| it.map(|_| PackedVector2Array::new()))
                .collect_vec();
            let mut weights = PackedFloat32Array::new();
            let mut indices = PackedInt32Array::new();

            for (id, poly) in self
                .polygons
                .iter_mut()
                .enumerate()
                .filter(|(id, _)| self.surfaces.get(&(*id as i32)).unwrap_or(&0) == surface_id)
            {
                let fan_v = poly.vert.remove(0) as i32;
                let fan = poly
                    .vert
                    .windows(2)
                    .flat_map(|w| [w[1] as i32, w[0] as i32, fan_v]);

                for vert in fan {
                    let uv = materials_subset
                        .iter()
                        .map(|it| {
                            it.and_then(|(name, it)| {
                                find_mapping(it, id, vert).map(|it| (name, it))
                            })
                        })
                        .collect_vec();
                    // let weight = find_mapping(weight_mappings, id, vert);

                    indices.push(
                        *vertex_map
                            .entry(UniqueVertex {
                                vert,
                                uv: uv
                                    .iter()
                                    .map(|it| {
                                        it.map(|(_, uv)| [uv.x.to_ne_bytes(), uv.y.to_ne_bytes()])
                                    })
                                    .collect(),
                                weight: (0.0f32).to_ne_bytes(),
                            })
                            .or_insert_with(|| {
                                vertices.push(*self.points.get(vert as usize).unwrap());
                                for (i, uv) in uv.iter().enumerate() {
                                    if let Some(uvs) = &mut uvs[i] {
                                        uvs.push(uv.map(|(_, it)| it).unwrap_or_else(|| {
                                            material_incomplete[i] += 1;
                                            Vector2::ZERO
                                        }));
                                    }
                                }
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
            for (i, uv) in uvs.iter().enumerate() {
                if let Some(uv) = uv {
                    surface.set(
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
            surface.set(ArrayType::ARRAY_INDEX.ord() as usize, indices.to_variant());

            for (i, inc) in material_incomplete.into_iter().enumerate() {
                if inc != 0 {
                    godot_error!(
                        "{}:{}'s UV map '{}' has {} ({}%) incomplete UVs",
                        self.name,
                        surface_id,
                        material_uv_names[i].unwrap(),
                        inc,
                        (inc as f32 / vertices.len() as f32) * 100.0
                    );
                }
            }

            if vertices.is_empty() {
                continue;
            }
            mesh.add_surface_from_arrays(
                PrimitiveType::PRIMITIVE_TRIANGLES,
                surface,
                Array::new(),
                Dictionary::new(),
                ArrayFormat::ARRAY_FORMAT_NORMAL,
            );
            surface_materials.push(*surface_id);
        }

        let mut out_mesh = ArrayMesh::new();
        out_mesh.set_name(self.name.into());
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

            if let Some(mat) = materials.get(self.surfaces[&(i as i32)] as usize) {
                out_mesh.surface_set_material(i, mat.material.share().upcast())
            }
        }

        out_mesh
    }
}
