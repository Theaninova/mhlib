use crate::pro::PowerRenderObject;
use binrw::BinRead;
use std::fs::File;

pub mod pro;

fn main() {
    let mut file = File::open(r#"E:\Games\Moorhuhn Kart\data\alk.pro"#).unwrap();
    let result = PowerRenderObject::read(&mut file).unwrap();
    println!("{:#?}", result);
}
