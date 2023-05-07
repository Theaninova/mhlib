use crate::media::ui::image::UiImage;
use crate::media::ui::menu::UiMenu;
use crate::media::ui::static_text::UiStaticText;
use crate::media::ui::text_area::UiTextArea;
use crate::media::ui::text_button::UiTextButton;
use crate::media::ui::text_field::UiTextField;
use crate::media::ui::toggle_button::UiToggleButton;
use serde::Deserialize;

pub mod image;
pub mod menu;
pub mod static_text;
pub mod text_area;
pub mod text_button;
pub mod text_field;
pub mod toggle_button;
pub mod vec;

#[derive(Debug, Clone, Deserialize)]
pub enum UiTag {
    Menu(UiMenu),
    Image(UiImage),
    TextButton(UiTextButton),
    TextArea(UiTextArea),
    TextField(UiTextField),
    StaticText(UiStaticText),
    ToggleButton(UiToggleButton),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlign {
    fn default() -> HorizontalAlign {
        HorizontalAlign::Left
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FadeMode {
    None,
}

impl Default for FadeMode {
    fn default() -> Self {
        FadeMode::None
    }
}

impl UiTag {
    pub fn post_process(&mut self) {
        if let UiTag::Menu(menu) = self {
            let children: Vec<UiTag> = menu.children.drain(..).collect();
            let mut area_stack: Vec<Vec<UiTag>> = vec![vec![]];

            for mut child in children {
                child.post_process();
                if let UiTag::TextArea(mut area) = child {
                    let children = area_stack.pop().unwrap();
                    let opening_tag = area_stack.last_mut().map(|it| it.last_mut());

                    if let Some(Some(UiTag::TextArea(opening_tag))) = opening_tag {
                        opening_tag.children = children;
                    } else {
                        area_stack.push(children);
                    }

                    if area.position.is_some() && area.size.is_some() {
                        let children = area.children.drain(..).collect();
                        area_stack.last_mut().unwrap().push(UiTag::TextArea(area));
                        area_stack.push(children);
                    }
                } else {
                    area_stack.last_mut().unwrap().push(child);
                }
            }

            menu.children = area_stack.pop().unwrap();
            debug_assert!(area_stack.is_empty());
        }
    }
}
