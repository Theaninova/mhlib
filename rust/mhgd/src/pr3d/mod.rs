use crate::pr3d::pro::pro_to_gd;
use godot::bind::godot_api;
use godot::builtin::GodotString;
use godot::engine::PackedScene;
use godot::obj::Gd;
use godot::prelude::GodotClass;
use powerrender_3d::pro::PowerRenderObject;

pub mod pro;

#[derive(GodotClass)]
#[class(init)]
pub struct ProLoader {}

#[godot_api]
impl ProLoader {
    #[func]
    pub fn load(path: GodotString) -> Gd<PackedScene> {
        pro_to_gd(PowerRenderObject::from_file(path.to_string()).unwrap())
    }
}
