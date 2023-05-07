use crate::media::ui::vec::deserialize_vec2;
use crate::media::ui::{FadeMode, HorizontalAlign};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UiStaticText {
    pub text: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(rename = "halign", default)]
    pub horizontal_align: HorizontalAlign,
    #[serde(rename = "fademode", default)]
    pub fade_mode: FadeMode,
}

#[cfg(test)]
mod tests {
    use crate::media::ui::static_text::UiStaticText;

    // language=xml
    const STATIC_TEXT: &str = "<StaticText text='test' position='1,2' />";

    #[test]
    fn it_should_read() {
        let static_text: UiStaticText = serde_xml_rs::from_str(STATIC_TEXT).unwrap();
        assert_eq!(static_text.text, "test".to_string());
        assert_eq!(static_text.position, [1, 2]);
    }
}
