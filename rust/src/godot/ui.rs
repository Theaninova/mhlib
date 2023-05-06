use crate::formats::ui_xml::{HorizontalAlign, UiTag};
use godot::builtin::{Array, Dictionary, GodotString, ToVariant, Vector2};
use godot::engine::control::{LayoutPreset, SizeFlags};
use godot::engine::global::HorizontalAlignment;
use godot::engine::node::InternalMode;
use godot::engine::{load, Button, Control, Label, Node, SpinBox, TextureRect};
use godot::obj::{Gd, Inherits, Share};
use itertools::Itertools;

const ACTION_META_NAME: &str = "action";

pub fn convert_ui(ui: UiTag, owner: Option<Gd<Node>>, base_path: &str) -> Gd<Node> {
    match ui {
        UiTag::Menu(menu) => {
            let mut gd_menu = Control::new_alloc();
            gd_menu.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
            attach_children(&mut gd_menu, owner, menu.children, base_path);
            gd_menu.upcast()
        }
        UiTag::Image(image) => {
            let mut gd_image = TextureRect::new_alloc();
            let texture = load(format!("{}/sprites/{}.bmp", base_path, image.texture));

            gd_image.set_texture(texture);
            gd_image.set_name(image.texture.into());
            gd_image.set_position(to_vec2(image.position), false);
            gd_image.set_size(to_vec2(image.size), false);
            gd_image.upcast()
        }
        UiTag::StaticText(text) => {
            let mut label = Label::new_alloc();
            label.set_anchors_preset(LayoutPreset::PRESET_TOP_WIDE, false);
            label.set_position(to_vec2(text.position), false);
            label.set_horizontal_alignment(text.horizontal_align.into());
            label.set_text(text.text.into());
            label.upcast()
        }
        UiTag::TextArea(area) => {
            let mut text_area = Control::new_alloc();
            text_area.set_anchors_preset(LayoutPreset::PRESET_FULL_RECT, false);
            text_area.set_position(to_vec2(area.position), false);
            text_area.set_size(to_vec2(area.size), false);
            attach_children(&mut text_area, owner, area.children, base_path);
            text_area.upcast()
        }
        UiTag::ToggleButton(toggle) => {
            let mut spin_box = SpinBox::new_alloc();
            spin_box.set_position(to_vec2(toggle.position), false);
            spin_box.set_min(toggle.min_value as f64);
            spin_box.set_max(toggle.max_value as f64);
            spin_box.set_step(toggle.value_step as f64);
            if let Some(name) = toggle.name {
                spin_box.set_name(GodotString::from(name));
            }
            spin_box.set_meta("text".into(), toggle.text.to_variant());
            spin_box.set_meta("target".into(), toggle.target.to_variant());
            spin_box.set_meta(
                "no_sound".into(),
                toggle.no_sound.unwrap_or(false).to_variant(),
            );
            attach_call_meta(&mut spin_box, toggle.on_change);
            spin_box.upcast()
        }
        UiTag::TextButton(button) => {
            let mut gd_button = Button::new_alloc();
            gd_button.set_anchors_preset(LayoutPreset::PRESET_TOP_WIDE, false);
            gd_button.set_flat(true);
            gd_button.set_position(to_vec2(button.position), false);
            gd_button.set_text_alignment(button.horizontal_align.into());
            if let Some(name) = button.name {
                gd_button.set_name(GodotString::from(name));
            }
            gd_button.set_text(GodotString::from(button.text));
            attach_call_meta(&mut gd_button, button.on_select);
            gd_button.upcast()
        }
    }
}

impl Into<HorizontalAlignment> for HorizontalAlign {
    fn into(self) -> HorizontalAlignment {
        match self {
            HorizontalAlign::Center => HorizontalAlignment::HORIZONTAL_ALIGNMENT_CENTER,
            HorizontalAlign::Left => HorizontalAlignment::HORIZONTAL_ALIGNMENT_LEFT,
            HorizontalAlign::Right => HorizontalAlignment::HORIZONTAL_ALIGNMENT_RIGHT,
        }
    }
}

fn attach_children<T>(
    node: &mut Gd<T>,
    owner: Option<Gd<Node>>,
    children: Vec<UiTag>,
    base_path: &str,
) where
    T: Inherits<Node>,
{
    let owner_node = owner.unwrap_or_else(|| node.share().upcast());

    for child in children {
        let mut child = convert_ui(child, Some(owner_node.share()), base_path);
        node.share()
            .upcast()
            .add_child(child.share(), false, InternalMode::INTERNAL_MODE_FRONT);
        child.set_owner(owner_node.share());
    }
}

fn to_vec2(vec: [i32; 2]) -> Vector2 {
    Vector2 {
        x: vec[0] as f32,
        y: vec[1] as f32,
    }
}

fn attach_call_meta<T>(button: &mut Gd<T>, call_string: String)
where
    T: Inherits<Node>,
{
    let mut call = call_string.split_whitespace().collect_vec();
    if call.is_empty() {
        return;
    }
    if let Some((name,)) = call.drain(..1).collect_tuple() {
        button.share().upcast().set_meta(
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
}
