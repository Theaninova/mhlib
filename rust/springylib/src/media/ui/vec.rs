use serde::{Deserialize, Deserializer};
use serde::de::Error;

pub fn deserialize_vec2_opt<'de, D>(deserializer: D) -> Result<Option<[i32; 2]>, D::Error>
    where
        D: Deserializer<'de>,
{
    if let Some(buf) = Option::<String>::deserialize(deserializer)? {
        to_vec2::<D>(buf).map(Some)
    } else {
        Ok(None)
    }
}

pub fn deserialize_vec2<'de, D>(deserializer: D) -> Result<[i32; 2], D::Error>
    where
        D: Deserializer<'de>,
{
    to_vec2::<D>(String::deserialize(deserializer)?)
}

pub fn deserialize_vec4<'de, D>(deserializer: D) -> Result<[i32; 4], D::Error>
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
