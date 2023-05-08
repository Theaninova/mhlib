use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DecryptError {
    FromUtf8Error(FromUtf8Error),
    ParseIntError(ParseIntError),
}

impl Display for DecryptError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecryptError::FromUtf8Error(error) => write!(f, "{}", error),
            DecryptError::ParseIntError(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for DecryptError {}

impl From<FromUtf8Error> for DecryptError {
    fn from(e: FromUtf8Error) -> DecryptError {
        DecryptError::FromUtf8Error(e)
    }
}

impl From<ParseIntError> for DecryptError {
    fn from(e: ParseIntError) -> DecryptError {
        DecryptError::ParseIntError(e)
    }
}

/// Decrypts txt files contained inside the dat file
pub fn decrypt_txt<I>(buffer: I) -> Result<String, DecryptError>
where
    I: Iterator<Item = u8>,
{
    let mut key = 0x1234u16;

    String::from_utf8(
        buffer
            .map(|char| {
                let decr = char ^ key as u8;
                key = key.wrapping_mul(3).wrapping_add(2);
                decr
            })
            .map(|char| (((char >> 1) ^ (char << 1)) & 0x55) ^ (char << 1))
            .collect(),
    )
    .map_err(DecryptError::from)
}

/// Parses a hex string to a Vec<u8>
fn from_hex(line: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..line.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(line.get(i..=i + 1).unwrap_or(""), 16))
        .collect()
}

/// This function is applied to *exposed* txt files,
/// such as the player profile or high scores
///
/// If the file is contained in the datafile, it has
/// to first be decrypted normally and then again
/// with this function.
pub fn decrypt_exposed_txt(contents: String) -> Result<String, DecryptError> {
    contents
        .split_terminator("\r\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(from_hex)
        .map(|line| decrypt_txt(line.map_err(DecryptError::from)?.into_iter()))
        .collect::<Result<Vec<String>, _>>()
        .map(|l| l.join("\r\n"))
}

#[cfg(test)]
mod tests {
    use crate::media::txt::{decrypt_exposed_txt, decrypt_txt, from_hex};

    #[test]
    fn it_should_parse_hex() {
        assert_eq!(from_hex("abcdef").unwrap(), vec![0xab, 0xcd, 0xef]);
    }

    #[test]
    fn it_should_decrypt() {
        let data: Vec<u8> = vec![0x3a, 0x9b, 0x6f, 0x09, 0x7e, 0xd3, 0x74, 0xd6];
        assert_eq!(decrypt_txt(data.into_iter()).unwrap(), "\r\nsound ",)
    }

    #[test]
    fn it_should_decrypt_exposed() {
        assert_eq!(
            decrypt_exposed_txt("83\r\n248ecc86d5d85f6fc6626a6ef5be3e".to_string()).unwrap(),
            "{\r\n    \"isValid\" 1"
        )
    }
}
