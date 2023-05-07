use serde::Deserialize;
use crate::media::ui::UiTag;

#[derive(Debug, Clone, Deserialize)]
pub struct UiMenu {
    pub selected: String,
    #[serde(rename = "OnBack")]
    pub on_back: Option<String>,
    #[serde(rename = "$value", default)]
    pub children: Vec<UiTag>,
}
