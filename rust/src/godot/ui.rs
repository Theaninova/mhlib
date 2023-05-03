use crate::formats::ui_xml::{HorizontalAlign, UiTag};
use godot::builtin::{GodotString, Vector2};
use godot::engine::global::HorizontalAlignment;
use godot::engine::node::InternalMode;
use godot::engine::{Button, Container, Control, TextureRect};
use godot::prelude::*;

pub fn convert_ui(ui: UiTag) -> Gd<Control> {
    match ui {
        UiTag::Menu(menu) => {
            let mut gd_menu = Container::new_alloc();
            for child in menu.children {
                gd_menu.add_child(
                    convert_ui(child).upcast(),
                    false,
                    InternalMode::INTERNAL_MODE_FRONT,
                );
            }
            gd_menu.upcast()
        }
        UiTag::Image(image) => {
            let mut gd_image = TextureRect::new_alloc();
            gd_image.set_position(
                Vector2 {
                    x: image.position[0] as f32,
                    y: image.position[1] as f32,
                },
                false,
            );
            gd_image.set_size(
                Vector2 {
                    x: image.size[0] as f32,
                    y: image.size[1] as f32,
                },
                false,
            );
            gd_image.upcast()
        }
        UiTag::TextButton(button) => {
            let mut gd_button = Button::new_alloc();
            gd_button.set_position(
                Vector2 {
                    x: button.position[0] as f32,
                    y: button.position[1] as f32,
                },
                false,
            );
            gd_button.set_text_alignment(match button.horizontal_align {
                HorizontalAlign::Center => HorizontalAlignment::HORIZONTAL_ALIGNMENT_CENTER,
            });
            if let Some(name) = button.name {
                gd_button.set_name(GodotString::from(name));
            }
            gd_button.set_text(GodotString::from(button.text));
            gd_button.upcast()
        }
    }
}
