use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub enum UiTag {
    Menu(UiMenu),
    Image(UiImage),
    TextButton(UiTextButton),
    TextArea(UiTextArea),
    StaticText(UiStaticText),
    ToggleButton(UiToggleButton),
}

#[derive(Debug, Deserialize)]
pub struct UiMenu {
    pub selected: String,
    #[serde(rename = "OnBack")]
    pub on_back: Option<String>,
    #[serde(rename = "$value", default)]
    pub children: Vec<UiTag>,
}

#[derive(Debug, Deserialize)]
pub struct UiImage {
    pub texture: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(deserialize_with = "deserialize_vec2")]
    pub size: [i32; 2],
    #[serde(rename = "fademode")]
    pub fade_mode: FadeMode,
}

#[derive(Debug, Deserialize)]
pub struct UiTextButton {
    pub name: Option<String>,
    pub text: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(rename = "halign")]
    pub horizontal_align: HorizontalAlign,
    #[serde(rename = "fademode")]
    pub fade_mode: FadeMode,
    #[serde(rename = "OnSelect")]
    pub on_select: String,
}

/// This sometimes appears completely empty
#[derive(Debug, Deserialize)]
pub struct UiTextArea {
    #[serde(deserialize_with = "deserialize_vec2", default)]
    pub position: [i32; 2],
    #[serde(deserialize_with = "deserialize_vec2", default)]
    pub size: [i32; 2],
    #[serde(rename = "$value", default)]
    pub children: Vec<UiTag>,
}

#[derive(Debug, Deserialize)]
pub struct UiStaticText {
    pub text: String,
    #[serde(deserialize_with = "deserialize_vec2")]
    pub position: [i32; 2],
    #[serde(rename = "halign")]
    pub horizontal_align: HorizontalAlign,
    #[serde(rename = "fademode")]
    pub fade_mode: FadeMode,
}

#[derive(Debug, Deserialize)]
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
    #[serde(rename = "noSound")]
    pub no_sound: Option<bool>,
    #[serde(rename = "OnChange")]
    pub on_change: String,
    #[serde(rename = "OnSelect")]
    pub on_select: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FadeMode {
    None,
}

fn deserialize_vec2<'de, D>(deserializer: D) -> Result<[i32; 2], D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let mut values: Vec<Result<i32, D::Error>> = buf
        .split(',')
        .into_iter()
        .map(|value| {
            // there's some typos so we have to cover that...
            value.split_ascii_whitespace().collect::<Vec<&str>>()[0]
                .trim()
                .parse::<i32>()
                .map_err(|err| Error::custom(err.to_string()))
        })
        .collect();
    let y = values.pop().ok_or(Error::custom("InvalidField"))??;
    let x = values.pop().ok_or(Error::custom("InvalidField"))??;

    Ok([x, y])
}
