use godot::builtin::{Vector2, Vector3};

#[derive(Hash, Eq, PartialEq)]
pub struct UniqueVertex {
    vert: [[u8; 4]; 3],
    material_id: u16,
    uv: Vec<Option<[[u8; 4]; 2]>>,
    weight: [u8; 4],
}

impl UniqueVertex {
    pub fn from_point(
        vert: &Vector3,
        uvs: &[Option<Vector2>],
        weight: f32,
        material_id: u16,
    ) -> Self {
        Self {
            vert: [
                vert.x.to_ne_bytes(),
                vert.y.to_ne_bytes(),
                vert.z.to_ne_bytes(),
            ],
            material_id,
            uv: uvs
                .iter()
                .map(|it| it.map(|uv| [uv.x.to_ne_bytes(), uv.y.to_ne_bytes()]))
                .collect(),
            weight: weight.to_ne_bytes(),
        }
    }
}
