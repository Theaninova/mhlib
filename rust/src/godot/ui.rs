use crate::formats::ui_xml::{HorizontalAlign, UiTag};
use godot::builtin::{Array, Dictionary, GodotString, Signal, ToVariant, Vector2};
use godot::engine::global::HorizontalAlignment;
use godot::engine::node::InternalMode;
use godot::engine::{Button, Control, Node, TextureRect};
use godot::obj::{Gd, Share};
use godot::sys::GDEXTENSION_VARIANT_TYPE_STRING;
use itertools::Itertools;

const ACTION_META_NAME: &str = "action";

pub fn convert_ui(ui: UiTag, owner: Option<Gd<Node>>) -> Gd<Node> {
    match ui {
        UiTag::Menu(menu) => {
            let mut gd_menu = Control::new_alloc();
            let owner_node = owner.unwrap_or_else(|| gd_menu.share().upcast());

            for child in menu.children {
                let mut child = convert_ui(child, Some(owner_node.share()));
                gd_menu.add_child(child.share(), false, InternalMode::INTERNAL_MODE_FRONT);
                child.set_owner(owner_node.share());
            }
            gd_menu.upcast()
        }
        UiTag::Image(image) => {
            let mut gd_image = TextureRect::new_alloc();
            gd_image.set_name(image.texture.into());
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

            let mut call = button.on_select.split_whitespace().collect_vec();
            if let Some((name,)) = call.drain(..1).collect_tuple() {
                gd_button.set_meta(
                    ACTION_META_NAME.into(),
                    Dictionary::from([
                        (&"name".to_variant(), &name.to_variant()),
                        (
                            &"args".to_variant(),
                            &Array::from(
                                call.into_iter()
                                    .map(GodotString::from)
                                    .collect::<Vec<GodotString>>()
                                    .as_slice(),
                            )
                            .to_variant(),
                        ),
                    ])
                    .to_variant(),
                );
            }

            gd_button.upcast()
        }
    }
}
