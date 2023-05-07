use crate::media::ui::vec::deserialize_vec2_opt;
use crate::media::ui::UiTag;
use serde::Deserialize;

/// This is a really weird node, sometimes it has children and sometimes, don't ask me why,
/// it appears as a normal tag and then gets closed by an empty tag of this kind.
#[derive(Debug, Clone, Deserialize)]
pub struct UiTextArea {
    #[serde(deserialize_with = "deserialize_vec2_opt", default)]
    pub position: Option<[i32; 2]>,
    #[serde(deserialize_with = "deserialize_vec2_opt", default)]
    pub size: Option<[i32; 2]>,
    #[serde(rename = "$value", default)]
    pub children: Vec<UiTag>,
}

impl UiTextArea {
    pub fn is_closing_tag(&self) -> bool {
        self.position.is_none() && self.size.is_none() && self.children.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::media::ui::text_area::UiTextArea;

    // language=xml
    const TEXT_AREA: &str = "<TextArea position='1,2' size='3,4' />";
    // language=xml
    const EMPTY: &str = "<TextArea />";

    #[test]
    fn it_should_read() {
        let text_area: UiTextArea = serde_xml_rs::from_str(TEXT_AREA).unwrap();
        assert_eq!(text_area.position, Some([1, 2]));
        assert_eq!(text_area.size, Some([3, 4]));
        assert!(!text_area.is_closing_tag());
    }

    #[test]
    fn it_should_read_empty() {
        let text_area: UiTextArea = serde_xml_rs::from_str(EMPTY).unwrap();
        assert_eq!(text_area.position, None);
        assert_eq!(text_area.size, None);
        assert!(text_area.is_closing_tag());
    }
}
