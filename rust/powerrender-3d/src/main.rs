use crate::pro::read_power_render_object;
use binrw::{BinRead, BinResult};
use std::fs::File;

pub mod pro;

fn main() {
    let mut file = File::open(r#"E:\Games\Moorhuhn Kart\data\alk.pro"#).unwrap();
    let result = read_power_render_object(&mut file).unwrap();
    println!("{:#?}", result);
}
