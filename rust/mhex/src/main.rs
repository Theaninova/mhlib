use starforcelib::sarc::SarcArchive;

fn main() {
    let path = "E:\\Games\\Moorhuhn Kart 3\\data.sar";
    SarcArchive::extract_all(path).unwrap();
}
