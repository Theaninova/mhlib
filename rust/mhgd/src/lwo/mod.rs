use crate::lwo::object::lightwave_to_gd;
use godot::bind::{godot_api, GodotClass};
use godot::builtin::GodotString;
use godot::engine::ArrayMesh;
use godot::obj::Gd;
use lightwave_3d::LightWaveObject;

pub(crate) mod mapping;
pub(crate) mod material;
pub(crate) mod object;
pub(crate) mod surface;

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
