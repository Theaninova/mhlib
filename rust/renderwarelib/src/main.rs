use binrw::BinRead;
use renderwarelib::chunks::RwSection;
use std::fs::File;

pub fn main() {
    let mut file = File::open("/run/media/theaninova/Heart Drive/Games/Moorhuhn Kart 2/data/data/mk2/level01/objects/collision_border.dff")
        .unwrap();
    let data = RwSection::read(&mut file).unwrap();

    println!("{:?}", data)
}
