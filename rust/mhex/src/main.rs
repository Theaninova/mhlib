use glob::glob;
use lightwave_3d::LightWaveObject;

fn main() {
    let mut successful = 0;
    let mut failed = 0;

    for entry in glob("E:/Games/Moorhuhn Kart 3/extract/**/*.lwo").unwrap() {
        let path = entry.unwrap();
        println!("{:?}", path.display());
        match LightWaveObject::read_file(path) {
            Ok(_) => {
                successful += 1;
                println!("...Ok")
            }
            Err(err) => {
                failed += 1;
                eprintln!("{:?}", err)
            }
        }
    }

    println!("Successful: {}\nFailed: {}", successful, failed);
}
