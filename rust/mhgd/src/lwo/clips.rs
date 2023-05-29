use godot::engine::{load, Image};
use godot::log::godot_error;
use godot::obj::Gd;
use lightwave_3d::lwo2::tags::image_clip::{ImageClip, ImageClipSubChunk};
use std::collections::HashMap;

pub fn collect_clip(target: &mut HashMap<u32, Gd<Image>>, clip: ImageClip) {
    for img in clip.attributes.iter() {
        match img {
            ImageClipSubChunk::StillImage(still) => {
                let path = format!("sar://{}", still.name.replace('\\', "/").replace(':', ":/"));
                let mut image: Gd<Image> = load(path);
                image.set_name(still.name.clone().into());
                target.insert(clip.index, image);
            }
            x => {
                godot_error!("TODO: Clip chunk {:?}", x)
            }
        }
    }
}
