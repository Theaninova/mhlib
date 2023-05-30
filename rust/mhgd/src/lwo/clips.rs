use crate::starforce::sar_archive::sarc_path_to_gd;
use godot::builtin::{GodotString, Variant};
use godot::engine::{load, AnimatedTexture, Image, ImageTexture, Texture2D};
use godot::log::{godot_error, godot_print};
use godot::obj::Gd;
use lightwave_3d::lwo2::tags::image_clip::{ImageClip, ImageClipSubChunk};
use std::collections::HashMap;

fn load_texture(path: &str) -> Gd<ImageTexture> {
    let mut image: Gd<ImageTexture> = load(path);
    godot_print!("ResPath: {}", image.get_path());
    let target_path = image
        .get_meta("target_path".into(), Variant::nil())
        .to::<GodotString>();
    image.set_path(target_path);
    godot_print!(
        "NowResPath: {} - {}",
        image.is_local_to_scene(),
        image.get_path()
    );
    image
}

pub fn collect_clip(target: &mut HashMap<u32, Gd<Texture2D>>, clip: ImageClip) {
    let mut attributes = clip.attributes.iter();

    match attributes.next().unwrap() {
        ImageClipSubChunk::StillImage(still) => {
            let path = sarc_path_to_gd(&still.name);
            for meta in attributes {
                godot_error!("TODO: {:?}", meta)
            }
            target.insert(clip.index, load_texture(&path).upcast());
        }
        ImageClipSubChunk::ImageSequence(sequence) => {
            let mut texture = AnimatedTexture::new();
            texture.set_frames(sequence.data.end as i64 - sequence.data.start as i64);
            if sequence.data.flags & 0x1 != 1 {
                godot_error!("Non-looping animated textures are not supported!")
            }
            let mut frame_duration = 1.0 / 15.0;

            for meta in attributes {
                match meta {
                    ImageClipSubChunk::Time(time) => {
                        frame_duration = 1.0 / time.frame_rate as f64;
                    }
                    x => godot_error!("TODO: {:?}", x),
                }
            }

            for i in sequence.data.start..sequence.data.end {
                let path = format!(
                    "{}{:0width$}{}.res",
                    sarc_path_to_gd(&sequence.data.prefix)
                        .strip_suffix(".res")
                        .unwrap(),
                    i,
                    sequence.data.suffix,
                    width = sequence.data.num_digits as usize
                );
                let frame = i as i64 - sequence.data.start as i64;

                texture.set_frame_texture(frame, load_texture(&path).upcast());
                texture.set_frame_duration(frame, frame_duration);
            }

            // texture.set_current_frame(sequence.data.offset as i64);
            target.insert(clip.index, texture.upcast());
        }
        x => {
            godot_error!("TODO: Clip chunk {:?}", x)
        }
    }
}
