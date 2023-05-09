use binrw::binread;

/// The type subchunk records the format in which the envelope is displayed to the user and a type
/// code that identifies the components of certain predefined envelope triples. The user format has
/// no effect on the actual values, only the way they're presented in LightWave®'s interface.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct EnvelopeType {
    pub user_format: UserFormat,
    pub kind: EnvelopeKind,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
pub enum UserFormat {
    Float = 2,
    Distance = 3,
    Percent = 4,
    Angle = 5,
}

#[binread]
#[br(repr = u8)]
#[derive(Debug)]
pub enum EnvelopeKind {
    PositionX = 0x1,
    PositionY = 0x2,
    PositionZ = 0x3,
    RotHeading = 0x4,
    RotPitch = 0x5,
    RotBank = 0x6,
    ScaleX = 0x7,
    ScaleY = 0x8,
    ScaleZ = 0x9,
    ColorR = 0xa,
    ColorG = 0xb,
    ColorB = 0xc,
    FalloffX = 0xd,
    FalloffY = 0xe,
    FalloffZ = 0xf,
}
