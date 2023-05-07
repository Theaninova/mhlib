use crate::media::ui::vec::deserialize_vec2;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct UiToggleButton {
    pub name: Option<String>,
    pub text: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    pub value: String,
    #[serde(rename = "minValue")]
    pub min_value: i32,
    #[serde(rename = "maxValue")]
    pub max_value: i32,
    #[serde(rename = "valueStep")]
    pub value_step: i32,
    pub target: String,
    #[serde(rename = "targetLOffset", deserialize_with = "deserialize_vec2")]
    pub target_l_offset: [i32; 2],
    #[serde(rename = "targetROffset", deserialize_with = "deserialize_vec2")]
    pub target_r_offset: [i32; 2],
    #[serde(rename = "noSound", default)]
    pub no_sound: bool,
    #[serde(rename = "OnChange")]
    pub on_change: String,
    #[serde(rename = "OnSelect")]
    pub on_select: String,
}

#[cfg(test)]
mod tests {
    use crate::media::ui::toggle_button::UiToggleButton;

    // language=xml
    const TOGGLE_BUTTON: &str = "<ToggleButton \
                                    name='test' \
                                    text='abc' \
                                    position='1,2' \
                                    value='val' \
                                    minValue='0' \
                                    maxValue='10' \
                                    valueStep='1' \
                                    target='target' \
                                    targetLOffset='3,4' \
                                    targetROffset='5,6' \
                                    noSound='false' \
                                    OnChange='change' \
                                    OnSelect='select' />";

    #[test]
    fn it_should_read() {
        let toggle_button: UiToggleButton = serde_xml_rs::from_str(TOGGLE_BUTTON).unwrap();
        assert_eq!(
            toggle_button,
            UiToggleButton {
                name: Some("test".to_string()),
                text: "abc".to_string(),
                position: [1, 2],
                value: "val".to_string(),
                min_value: 0,
                max_value: 10,
                value_step: 1,
                target: "target".to_string(),
                target_l_offset: [3, 4],
                target_r_offset: [5, 6],
                no_sound: false,
                on_change: "change".to_string(),
                on_select: "select".to_string(),
            }
        )
    }
}
