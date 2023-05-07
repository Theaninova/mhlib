use crate::media::ui::vec::deserialize_vec2;
use crate::media::ui::FadeMode;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UiImage {
    pub texture: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(deserialize_with = "deserialize_vec2")]
    pub size: [i32; 2],
    #[serde(rename = "fademode", default)]
    pub fade_mode: FadeMode,
}

#[cfg(test)]
mod tests {
    use crate::media::ui::image::UiImage;

    // language=xml
    const IMAGE: &str = "<Image texture='tex' position='1,2' size='3,4' />";

    #[test]
    fn it_should_read() {
        let image: UiImage = serde_xml_rs::from_str(IMAGE).unwrap();
        assert_eq!(image.texture, "tex".to_string());
        assert_eq!(image.position, [1, 2]);
        assert_eq!(image.size, [3, 4]);
    }
}
