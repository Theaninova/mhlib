use crate::media::ui::UiTag;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct UiMenu {
    pub selected: String,
    #[serde(rename = "OnBack")]
    pub on_back: Option<String>,
    #[serde(rename = "$value", default)]
    pub children: Vec<UiTag>,
}

#[cfg(test)]
mod tests {
    use crate::media::ui::menu::UiMenu;

    // language=xml
    const MENU: &str = "<Menu selected='item' OnBack='back' />";

    #[test]
    fn it_should_read() {
        let menu: UiMenu = serde_xml_rs::from_str(MENU).unwrap();
        assert_eq!(menu.selected, "item".to_string());
        assert_eq!(menu.on_back, Some("back".to_string()));
    }
}
