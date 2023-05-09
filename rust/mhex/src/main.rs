use lightwave::LightWaveObject;

fn main() {
    let obj = LightWaveObject::read_file("E:\\Games\\Moorhuhn Kart 3\\extract\\D\\Moorhuhnkart\\3dobjects_tracks\\track04_robinhood\\colreset.lwo").unwrap();
    println!("{:#?}", obj);
}
