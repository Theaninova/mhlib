use crate::formats::rle::RleImage;
use godot::builtin::{Color, PackedByteArray};
use godot::engine::global::Error;
use godot::engine::image::Format;
use godot::engine::{Image, ImageTexture, SpriteFrames};
use godot::obj::Gd;

const FPS: f64 = 15.0;

pub fn load_rle_as_sprite_frames(rle: RleImage) -> Gd<SpriteFrames> {
    let mut frames = SpriteFrames::new();

    frames.set_animation_loop("default".into(), true);
    frames.set_animation_speed("default".into(), FPS);

    for frame in rle.frames.iter() {
        let mut image = Image::new();
        image.set_data(
            rle.width as i64,
            rle.height as i64,
            false,
            Format::FORMAT_RGBA8,
            PackedByteArray::from(rle.get_image_data(frame).as_slice()),
        );
        image.fix_alpha_edges();

        let mut texture = ImageTexture::new();
        texture.set_image(image);
        frames.add_frame("default".into(), texture.upcast(), 1.0, 0);
    }

    frames
}

pub fn load_bmp_as_image_texture(data: Vec<u8>) -> Result<Gd<Image>, Error> {
    let mut image = Image::new();

    match image.load_bmp_from_buffer(data.as_slice().into()) {
        Error::OK => {
            for x in 0..image.get_width() {
                for y in 0..image.get_height() {
                    if image.get_pixel(x, y).is_equal_approx(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    }) {
                        image.set_pixel(x, y, Color::TRANSPARENT_BLACK);
                    }
                }
            }
            image.fix_alpha_edges();
            Ok(image)
        }
        error => Err(error),
    }
}
