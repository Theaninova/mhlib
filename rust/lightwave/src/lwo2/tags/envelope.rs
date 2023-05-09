use crate::binrw_helpers::until_size_limit;
use crate::iff::SubChunk;
use crate::lwo2::vx;
use binrw::{binread, NullString, PosValue};
use crate::lwo2::sub_tags::plugin::PluginServerNameAndData;

#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct EnvelopeDefinition {
    #[br(temp)]
    pos_start: PosValue<()>,
    #[br(parse_with = vx)]
    pub index: u32,
    #[br(temp)]
    pos_end: PosValue<()>,
    #[br(parse_with = until_size_limit(length as u64 - (pos_end.pos - pos_start.pos)))]
    pub attributes: Vec<EnvelopeSubChunk>,
}

#[binread]
#[derive(Debug)]
pub enum EnvelopeSubChunk {
    #[br(magic(b"TYPE"))]
    EnvelopeType(SubChunk<EnvelopeType>),
    #[br(magic(b"PRE"))]
    PreBehavior(SubChunk<Behavior>),
    #[br(magic(b"POST"))]
    PostBehavior(SubChunk<Behavior>),
    #[br(magic(b"KEY"))]
    KeyframeTimeAndValue(SubChunk<KeyframeTimeAndValue>),
    #[br(magic(b"SPAN"))]
    IntervalInterpolation(SubChunk<IntervalInterpolation>),
    #[br(magic(b"CHAN"))]
    PluginChannelModifiers(SubChunk<PluginServerNameAndData>),
    #[br(magic(b"NAME"))]
    ChannelName(SubChunk<PluginChannelName>),
}

/// An optional name for the envelope. LightWave® itself ignores the names of surface envelopes,
/// but plug-ins can browse the envelope database by name.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct PluginChannelName {
    #[br(align_after = 2)]
    pub channel_name: NullString,
}

/// Defines the interpolation between the most recent KEY chunk and the KEY immediately before it in
/// time. The type identifies the interpolation algorithm and can be STEP, LINE, TCB
/// (Kochanek-Bartels), HERM (Hermite), BEZI (1D Bezier) or BEZ2 (2D Bezier).
/// Different parameters are stored for each of these.
#[binread]
#[br(import(length: u32))]
#[derive(Debug)]
pub struct IntervalInterpolation {
    pub kind: IntervalInterpolationType,
    #[br(count = (length - 4) / 4)]
    pub parameters: Vec<f32>,
}

#[binread]
#[derive(Debug)]
pub enum IntervalInterpolationType {
    #[br(magic(b"STEP"))]
    Step,
    #[br(magic(b"LINE"))]
    Line,
    #[br(magic(b"TCB\0"))]
    KochanekBartels,
    #[br(magic(b"HERM"))]
    Hermite,
    #[br(magic(b"BEZI"))]
    Bezier1D,
    #[br(magic(b"BEZ2"))]
    Bezier2D,
}

/// The value of the envelope at the specified time in seconds. The signal value between keyframes
/// is interpolated. The time of a keyframe isn't restricted to integer frames.
#[binread]
#[br(import(_length: u32))]
#[derive(Debug)]
pub struct KeyframeTimeAndValue {
    pub time: f32,
    pub value: f32,
}

#[binread]
#[br(repr = u16, import(_length: u32))]
#[derive(Debug)]
pub enum Behavior {
    /// Sets the value to 0.0.
    Reset = 0,
    /// Sets the value to the value at the nearest key.
    Constant = 1,
    /// Repeats the interval between the first and last keys (the primary interval).
    Repeat = 2,
    /// Like Repeat, but alternating copies of the primary interval are time-reversed.
    Oscillate = 3,
    /// Like Repeat, but offset by the difference between the values of the first and last keys.
    OffsetRepeat = 4,
    /// Linearly extrapolates the value based on the tangent at the nearest key.
    Linear = 5,
}

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
