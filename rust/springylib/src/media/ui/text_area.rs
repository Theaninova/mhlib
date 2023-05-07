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
