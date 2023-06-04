use binrw::binread;

#[binread]
#[derive(Debug)]
pub struct RpClump {
    pub num_atomics: i32,
    pub num_lights: i32,
    pub num_cameras: i32,
}
