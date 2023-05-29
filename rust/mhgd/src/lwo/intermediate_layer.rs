use crate::lwo::material::MaterialUvInfo;
use crate::lwo::surface_info::SurfaceInfo;
use godot::builtin::{Array, Dictionary, Vector2, Vector3};
use godot::engine::mesh::{ArrayFormat, PrimitiveType};
use godot::engine::{ArrayMesh, SurfaceTool};
use godot::log::godot_print;
use godot::obj::{Gd, Share};
use itertools::Itertools;
use lightwave_3d::lwo2::tags::polygon_list::PolygonList;
use std::collections::HashMap;

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
    pub material_mappings: HashMap<i32, u16>,
}

impl IntermediateLayer {
    pub fn commit(self, materials: &HashMap<u16, MaterialUvInfo>) -> Gd<ArrayMesh> {
        let mut mesh = ArrayMesh::new();
        mesh.set_name(self.name.clone().into());
        let surface_material_ids = self.material_mappings.values().unique().collect_vec();

        for material_id in surface_material_ids.iter() {
            let surface_info = SurfaceInfo::collect_from_layer(&self, &materials[material_id]);

            debug_assert!(!surface_info.is_empty());

            mesh.add_surface_from_arrays(
                PrimitiveType::PRIMITIVE_TRIANGLES,
                surface_info.commit_to_arrays(),
                Array::new(),
                Dictionary::new(),
                ArrayFormat::ARRAY_FORMAT_NORMAL,
            );
        }

        godot_print!(
            "{}: {:?}",
            &self.name,
            surface_material_ids
                .iter()
                .unique()
                .map(|id| { (*id, materials[id].material.get_name().to_string()) })
                .collect_vec()
        );

        let mut final_mesh = post_process_mesh(mesh, materials, surface_material_ids);
        final_mesh.set_name(self.name.into());
        final_mesh
    }
}

fn post_process_mesh(
    mesh: Gd<ArrayMesh>,
    materials: &HashMap<u16, MaterialUvInfo>,
    material_ids: Vec<&u16>,
) -> Gd<ArrayMesh> {
    let mut out_mesh = ArrayMesh::new();

    debug_assert_eq!(mesh.get_surface_count() as usize, material_ids.len());

    for (surface_idx, surface_id) in material_ids.into_iter().enumerate() {
        let mut tool = SurfaceTool::new();

        tool.create_from(mesh.share().upcast(), surface_idx as i64);
        tool.generate_normals(false);
        tool.generate_tangents();

        out_mesh.add_surface_from_arrays(
            PrimitiveType::PRIMITIVE_TRIANGLES,
            tool.commit_to_arrays(),
            Array::new(),
            Dictionary::new(),
            ArrayFormat::ARRAY_FORMAT_NORMAL,
        );

        let mat = &materials[surface_id];
        out_mesh.surface_set_material(surface_idx as i64, mat.material.share().upcast())
    }

    out_mesh
}
