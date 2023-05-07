use crate::media::ui::vec::{deserialize_vec2, deserialize_vec4};
use crate::media::ui::{FadeMode, HorizontalAlign};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UiTextField {
    pub name: Option<String>,
    pub text: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(rename = "bufferVar")]
    pub buffer_var: String,
    #[serde(deserialize_with = "deserialize_vec4")]
    pub area: [i32; 4],
    #[serde(rename = "halign", default)]
    pub horizontal_align: HorizontalAlign,
    #[serde(rename = "fademode", default)]
    pub fade_mode: FadeMode,
    #[serde(rename = "OnSelect")]
    pub on_select: String,
}

#[cfg(test)]
mod tests {
    use crate::media::ui::text_field::UiTextField;

    // language=xml
    const TEXT_FIELD: &str = "<TextField name='test' text='abc' position='1,2' bufferVar='var' area='1,2,3,4' OnSelect='click' />";

    #[test]
    fn it_should_read() {
        let text_field: UiTextField = serde_xml_rs::from_str(TEXT_FIELD).unwrap();
        assert_eq!(text_field.name, Some("test".to_string()));
        assert_eq!(text_field.text, "abc".to_string());
        assert_eq!(text_field.position, [1, 2]);
        assert_eq!(text_field.buffer_var, "var".to_string());
        assert_eq!(text_field.area, [1, 2, 3, 4]);
        assert_eq!(text_field.on_select, "click".to_string());
    }
}
