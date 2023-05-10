use godot::builtin::{Rect2, Vector2, Vector2i};
use godot::engine::{FontFile, Image};
use godot::prelude::utilities::prints;
use godot::prelude::{Gd, Share, ToVariant};

pub fn load_bitmap_font(image: Gd<Image>) -> Gd<FontFile> {
    let mut font_chars = CHARSET.iter();

    let mut font_file = FontFile::new();

    let mut was_empty_column = true;
    let mut char_x = 0;
    let mut char_width = 0;
    let char_height = image.get_height();
    let char_y = 0;

    let base_size = Vector2i { x: 16, y: 0 };

    font_file.set_texture_image(0, base_size, 0, image.share());

    for x in 0..image.get_width() {
        let is_empty_column = (0..image.get_height()).all(|y| image.get_pixel(x, y).a == 0.0);

        if !was_empty_column && is_empty_column {
            let char = font_chars.next().expect("Font has too many characters!");
            let mut glyph = 0i64;
            for (i, c) in WINDOWS_1252
                .decode(&[*char])
                .0
                .as_bytes()
                .iter()
                .enumerate()
            {
                glyph |= (*c as i64) << (i * 8);
            }

            let glyph_offset = Vector2 {
                x: char_x as f32,
                y: char_y as f32,
            };
            let glyph_size = Vector2 {
                x: char_width as f32,
                y: char_height as f32,
            };

            prints(
                "Glyph".to_variant(),
                &[
                    (*char as char).to_string().to_variant(),
                    glyph_offset.to_variant(),
                    glyph_size.to_variant(),
                ],
            );

            // font_file.set_glyph_offset(0, base_size, glyph, glyph_offset);
            font_file.set_glyph_size(0, base_size, glyph, glyph_size);
            font_file.set_glyph_uv_rect(
                0,
                base_size,
                glyph,
                Rect2 {
                    position: glyph_offset,
                    size: glyph_size,
                },
            );
            font_file.set_glyph_texture_idx(0, base_size, glyph, 0);
        } else if was_empty_column && !is_empty_column {
            char_x = x;
            char_width = 0;
        }

        char_width += 1;
        was_empty_column = is_empty_column;
    }

    font_file.set_font_name("menufont".into());
    // font_file.set_cache_ascent(0, base_size.x, )

    font_file
}
