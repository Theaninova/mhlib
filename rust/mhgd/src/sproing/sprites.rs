use godot::builtin::{GodotString, Rect2, StringName, ToVariant, Vector2};
use godot::engine::utilities::printerr;
use godot::engine::{
    load, AtlasTexture, ImageTexture, PlaceholderTexture2D, ResourceLoader, SpriteFrames,
};
use godot::obj::{Gd, Share};
use godot::prelude::GodotClass;
use springylib::media::sprites::{CropMode, RenderMode, Sprites};

const FPS: f64 = 15.0;
const SPRITE_EXTENSIONS: &[&str] = &["bmp", "rle"];

pub fn load_sprite_frames(sprites: Vec<Sprites>, path: GodotString) -> Gd<SpriteFrames> {
    let dir = path
        .to_string()
        .strip_suffix("/sprites.txt")
        .unwrap()
        .to_string();
    let mut sprite_frames = SpriteFrames::new();
    for sprite in sprites.into_iter() {
        if let RenderMode::FlipX = sprite.render_mode {
            continue;
        }
        sprite_frames.add_animation(StringName::from(&sprite.name));
        sprite_frames.set_animation_speed(StringName::from(&sprite.name), FPS);

        match select_from_extensions(&dir, &sprite.file_name) {
            Some((path, "rle")) => extract_rle_frames(&mut sprite_frames, &sprite, path),
            Some((path, "bmp")) => extract_bitmap_frames(&mut sprite_frames, &sprite, path),
            Some(_) | None => {
                printerr(
                    format!("Missing sprite '{}'", sprite.file_name).to_variant(),
                    &[],
                );
                let texture = PlaceholderTexture2D::new();
                sprite_frames.add_frame(
                    StringName::from(&sprite.name),
                    texture.upcast(),
                    60.0 / FPS,
                    0,
                );
            }
        }
    }

    sprite_frames
}

/// Loads an RLE file as SpriteFrames and extracts
/// its frames into `sprite_frames`
fn extract_rle_frames(sprite_frames: &mut SpriteFrames, sprite: &Sprites, path: String) {
    let frames: Gd<SpriteFrames> = load(path);
    for frame_idx in 0..frames.get_frame_count("default".into()) {
        sprite_frames.add_frame(
            StringName::from(&sprite.name),
            frames
                .get_frame_texture("default".into(), frame_idx)
                .unwrap(),
            60.0 / FPS,
            0,
        );
    }
}

/// Loads a bitmap and extracts its frames into `sprite_frames`
/// creates an atlas if there are multiple frames.
fn extract_bitmap_frames(sprite_frames: &mut SpriteFrames, sprite: &Sprites, path: String) {
    let texture: Gd<ImageTexture> = load(path);

    let frame_count = if let Some(CropMode::FrameCount(frame_count)) = sprite.frames {
        frame_count
    } else {
        1
    };

    if frame_count > 1 {
        let height = texture.get_height();
        let width = texture.get_width();
        let frame_height = height / frame_count as i64;

        for i in 0..frame_count as i64 {
            let mut atlas = AtlasTexture::new();
            atlas.set_atlas(texture.share().upcast());
            atlas.set_region(Rect2 {
                position: Vector2 {
                    x: 0.0,
                    y: (i * frame_height) as f32,
                },
                size: Vector2 {
                    x: width as f32,
                    y: frame_height as f32,
                },
            });

            sprite_frames.add_frame(
                StringName::from(&sprite.name),
                atlas.upcast(),
                60.0 / FPS,
                0,
            );
        }
    } else {
        sprite_frames.add_frame(
            StringName::from(&sprite.name),
            texture.upcast(),
            60.0 / FPS,
            0,
        );
    }
}

/// Selects the extension based on which file exists
fn select_from_extensions(dir: &str, file_name: &str) -> Option<(String, &'static str)> {
    SPRITE_EXTENSIONS
        .iter()
        .map(|ext| {
            (
                format!("{}/sprites/{}.{}", dir, file_name.to_lowercase(), ext),
                *ext,
            )
        })
        .find(|(path, ext)| {
            ResourceLoader::singleton().exists(
                path.clone().into(),
                match *ext {
                    "rle" => SpriteFrames::CLASS_NAME.to_string(),
                    "bmp" => ImageTexture::CLASS_NAME.to_string(),
                    _ => panic!(),
                }
                .into(),
            )
        })
}
