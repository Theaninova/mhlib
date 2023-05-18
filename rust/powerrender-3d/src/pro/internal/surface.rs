use crate::pro::internal::segment::{PrFace, PrVertices};
use crate::pro::PowerRenderSurface;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct UniqueVertex {
    pub pos: [i32; 3],
    pub normal: [i16; 3],
    pub color: [i32; 3],
    pub uv: [i32; 2],
}

impl PrVertices {
    /// converts the relatively uncommonly used format into one that can be used more easily
    /// by putting it into an array of surfaces that are separated by material
    pub fn into_surfaces(self, faces: Vec<PrFace>) -> Vec<PowerRenderSurface> {
        let mut surfaces = HashMap::new();

        for face in faces.into_iter() {
            let surface_id = ((face.material as u32) << 16) | (face.back_material as u32);
            let (index_map, surface) = if let Some(surface) = surfaces.get_mut(&surface_id) {
                surface
            } else {
                let surface = PowerRenderSurface {
                    material_index: face.material as usize,
                    back_material_index: face.back_material as usize,
                    uvs: vec![],
                    indices: vec![],
                    colors: vec![],
                    normals: vec![],
                    vertices: vec![],
                };
                surfaces.insert(surface_id, (HashMap::<UniqueVertex, i32>::new(), surface));
                surfaces.get_mut(&surface_id).unwrap()
            };

            let triangle = [
                (face.i0, face.u0, face.v0),
                (face.i1, face.u1, face.v1),
                (face.i2, face.u2, face.v2),
            ];
            for (index, u, v) in triangle {
                let vertex = &self.vertices[index as usize];
                let point = UniqueVertex {
                    pos: vertex.position,
                    normal: vertex.normal,
                    color: face.color,
                    uv: [u, v],
                };
                let index = if let Some(index) = index_map.get(&point) {
                    *index
                } else {
                    surface.vertices.push([
                        f32::from_ne_bytes(point.pos[2].to_ne_bytes()),
                        f32::from_ne_bytes(point.pos[1].to_ne_bytes()),
                        f32::from_ne_bytes(point.pos[0].to_ne_bytes()),
                    ]);
                    surface.normals.push([
                        point.normal[0] as f32 / 1024.0,
                        point.normal[1] as f32 / 1024.0,
                        point.normal[2] as f32 / 1024.0,
                    ]);
                    surface.uvs.push([
                        f32::from_ne_bytes(point.uv[0].to_ne_bytes()),
                        f32::from_ne_bytes(point.uv[1].to_ne_bytes()),
                    ]);
                    surface.colors.push([
                        point.color[0] as f32 / 255.0,
                        point.color[1] as f32 / 255.0,
                        point.color[2] as f32 / 255.0,
                    ]);

                    let index = surface.vertices.len() as i32 - 1;
                    index_map.insert(point, index);
                    index
                };
                surface.indices.push(index);
            }
        }

        surfaces.into_values().map(|(_, surface)| surface).collect()
    }
}
