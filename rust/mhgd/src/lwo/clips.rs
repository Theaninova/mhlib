use godot::engine::{load, AnimatedTexture, Image, ImageTexture, Texture2D};
use godot::log::godot_error;
use godot::obj::Gd;
use lightwave_3d::lwo2::tags::image_clip::{ImageClip, ImageClipSubChunk};
use std::collections::HashMap;

fn convert_path(path: &str) -> String {
    path.replace('\\', "/").replace(':', ":/")
}

fn load_texture(path: &str, name: &str) -> Gd<ImageTexture> {
    let mut image: Gd<Image> = load(path);
    image.set_name(name.into());
    let mut texture = ImageTexture::new();
    texture.set_name(name.into());
    texture.set_image(image);
    texture
}

pub fn collect_clip(target: &mut HashMap<u32, Gd<Texture2D>>, clip: ImageClip) {
    let mut attributes = clip.attributes.iter();

    match attributes.next().unwrap() {
        ImageClipSubChunk::StillImage(still) => {
            let path = format!("sar://{}", convert_path(&still.name));
            for meta in attributes {
                godot_error!("TODO: {:?}", meta)
            }
            target.insert(clip.index, load_texture(&path, &still.name).upcast());
        }
        ImageClipSubChunk::ImageSequence(sequence) => {
            let mut texture = AnimatedTexture::new();
            texture.set_frames(sequence.data.end as i64 - sequence.data.start as i64);
            if sequence.data.flags & 0x1 != 1 {
                godot_error!("Non-looping animated textures are not supported!")
            }
            let mut frame_duration = 15.0 / 60.0;

            for meta in attributes {
                match meta {
                    ImageClipSubChunk::Time(time) => {
                        frame_duration = time.frame_rate as f64 / 60.0;
                    }
                    x => godot_error!("TODO: {:?}", x),
                }
            }

            for i in sequence.data.start..sequence.data.end {
                let path = format!(
                    "sar://{}{:0width$}{}",
                    convert_path(&sequence.data.prefix),
                    i,
                    sequence.data.suffix,
                    width = sequence.data.num_digits as usize
                );
                let frame = i as i64 - sequence.data.start as i64;

                texture.set_frame_texture(frame, load_texture(&path, &i.to_string()).upcast());
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
