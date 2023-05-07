use crate::media::ui::vec::deserialize_vec2;
use crate::media::ui::{FadeMode, HorizontalAlign};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UiTextButton {
    pub name: Option<String>,
    pub text: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(rename = "halign", default)]
    pub horizontal_align: HorizontalAlign,
    #[serde(rename = "fademode", default)]
    pub fade_mode: FadeMode,
    #[serde(rename = "OnSelect")]
    pub on_select: String,
}

#[cfg(test)]
mod tests {
    use crate::media::ui::text_button::UiTextButton;

    // language=xml
    const BUTTON: &str =
        "<TextButton name='test' text='abc' position='1,2' OnSelect='StartGame' />";

    #[test]
    fn it_should_read() {
        let button: UiTextButton = serde_xml_rs::from_str(BUTTON).unwrap();
        assert_eq!(button.name, Some("test".to_string()));
        assert_eq!(button.text, "abc".to_string());
        assert_eq!(button.position, [1, 2]);
        assert_eq!(button.on_select, "StartGame".to_string());
    }
}
