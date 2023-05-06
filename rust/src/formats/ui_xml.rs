use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub enum UiTag {
    Menu(UiMenu),
    Image(UiImage),
    TextButton(UiTextButton),
    TextArea(UiTextArea),
    TextField(UiTextField),
    StaticText(UiStaticText),
    ToggleButton(UiToggleButton),
}

#[derive(Debug, Clone, Deserialize)]
pub struct UiMenu {
    pub selected: String,
    #[serde(rename = "OnBack")]
    pub on_back: Option<String>,
    #[serde(rename = "$value", default)]
    pub children: Vec<UiTag>,
}

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
    #[serde(rename = "fademode")]
    pub fade_mode: FadeMode,
    #[serde(rename = "OnSelect")]
    pub on_select: String,
}

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

#[derive(Debug, Clone, Deserialize)]
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

impl UiTag {
    pub fn post_process(&mut self) {
        if let UiTag::Menu(menu) = self {
            let children: Vec<UiTag> = menu.children.drain(..).collect();
            let mut area_stack: Vec<Vec<UiTag>> = vec![vec![]];

            for mut child in children {
                child.post_process();
                if let UiTag::TextArea(mut area) = child {
                    let children = area_stack.pop().unwrap();
                    let opening_tag = area_stack.last_mut().map(|it| it.last_mut());

                    if let Some(Some(UiTag::TextArea(opening_tag))) = opening_tag {
                        opening_tag.children = children;
                    } else {
                        area_stack.push(children);
                    }

                    if area.position.is_some() && area.size.is_some() {
                        let children = area.children.drain(..).collect();
                        area_stack.last_mut().unwrap().push(UiTag::TextArea(area));
                        area_stack.push(children);
                    }
                } else {
                    area_stack.last_mut().unwrap().push(child);
                }
            }

            menu.children = area_stack.pop().unwrap();
            debug_assert!(area_stack.is_empty());
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl Default for HorizontalAlign {
    fn default() -> HorizontalAlign {
        HorizontalAlign::Left
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FadeMode {
    None,
}

impl Default for FadeMode {
    fn default() -> Self {
        FadeMode::None
    }
}

fn deserialize_vec2_opt<'de, D>(deserializer: D) -> Result<Option<[i32; 2]>, D::Error>
where
    D: Deserializer<'de>,
{
    if let Some(buf) = Option::<String>::deserialize(deserializer)? {
        to_vec2::<D>(buf).map(Some)
    } else {
        Ok(None)
    }
}

fn deserialize_vec2<'de, D>(deserializer: D) -> Result<[i32; 2], D::Error>
where
    D: Deserializer<'de>,
{
    to_vec2::<D>(String::deserialize(deserializer)?)
}

fn deserialize_vec4<'de, D>(deserializer: D) -> Result<[i32; 4], D::Error>
where
    D: Deserializer<'de>,
{
    to_vec4::<D>(String::deserialize(deserializer)?)
}

fn to_vec<'de, D>(buf: String) -> Result<Vec<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    buf.split(',')
        .into_iter()
        .map(|value| {
            // there's some typos so we have to cover that...
            value.split_ascii_whitespace().collect::<Vec<&str>>()[0]
                .trim()
                .parse::<i32>()
                .map_err(|err| Error::custom(err.to_string()))
        })
        .collect()
}

fn to_vec4<'de, D>(buf: String) -> Result<[i32; 4], D::Error>
where
    D: Deserializer<'de>,
{
    let mut values = to_vec::<D>(buf)?;
    let w = values.pop().ok_or(Error::custom("InvalidField"))?;
    let z = values.pop().ok_or(Error::custom("InvalidField"))?;
    let y = values.pop().ok_or(Error::custom("InvalidField"))?;
    let x = values.pop().ok_or(Error::custom("InvalidField"))?;

    Ok([x, y, z, w])
}

fn to_vec2<'de, D>(buf: String) -> Result<[i32; 2], D::Error>
where
    D: Deserializer<'de>,
{
    let mut values = to_vec::<D>(buf)?;
    let y = values.pop().ok_or(Error::custom("InvalidField"))?;
    let x = values.pop().ok_or(Error::custom("InvalidField"))?;

    Ok([x, y])
}
