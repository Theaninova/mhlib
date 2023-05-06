use binrw::{BinRead, NullString};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{AnimationDecoder, ImageFormat};
use mhjnr::formats::datafile::Datafile;
use mhjnr::formats::level::level_tile_data_to_image;
use mhjnr::formats::rle::RleImage;
use mhjnr::formats::sprites::Sprites;
use mhjnr::formats::txt::{decrypt_exposed_txt, decrypt_txt};
use mhjnr::formats::ui_xml::UiTag;
use serde_xml_rs::from_str;
use std::ffi::OsStr;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::path::Path;

fn extract(datafile: &Datafile, file: &mut File) {
    let target = "E:\\Games\\Schatzjäger\\data3";

    for entry in &datafile.files {
        let file_name = format!("{}\\{}", target, entry.name);
        fs::create_dir_all(file_name.rsplit_once('\\').unwrap().0).unwrap();

        file.seek(SeekFrom::Start(entry.pos as u64)).unwrap();
        let mut data = vec![0u8; entry.len as usize];
        file.read_exact(&mut data).unwrap();

        if entry.name.to_string().ends_with(".txt") {
            let mut contents = decrypt_txt(data.into_iter()).unwrap();
            /*if entry
                .name
                .to_string()
                .split('\\')
                .collect::<Vec<&str>>()
                .len()
                == 1
            {
                contents = decrypt_exposed_txt(contents).unwrap();
            }*/
            File::create(file_name)
                .unwrap()
                .write_all(contents.as_bytes())
                .unwrap();
        } else if entry.name.to_string().ends_with(".rle") {
            let image: RleImage = RleImage::read(&mut Cursor::new(data)).unwrap();
            let mut encoder = GifEncoder::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(format!(
                        "{}.{}",
                        file_name.strip_suffix(".rle").unwrap(),
                        ".gif"
                    ))
                    .unwrap(),
            );
            encoder.set_repeat(Repeat::Infinite).unwrap();
            encoder.try_encode_frames(image.into_frames()).unwrap();
        } else {
            File::create(file_name)
                .unwrap()
                .write_all(data.as_slice())
                .unwrap();
        }
    }
}

fn main() {
    let file_name = Some(NullString::from("data\\profile_00.txt"));
    let dat_path = "E:\\Games\\Schatzjäger\\data\\datafile.dat";

    let mut file = File::open(dat_path).unwrap();
    let dat: Datafile = Datafile::read(&mut file).unwrap();
    println!("{:#?}", dat);

    // extract(&dat, &mut file);

    if let Some(file_name) = file_name {
        let target = dat.files.iter().find(|it| it.name == file_name).unwrap();
        file.seek(SeekFrom::Start(target.pos as u64)).unwrap();
        let mut data = vec![0u8; target.len as usize];
        file.read_exact(&mut data).unwrap();

        match Path::new(&file_name.to_string())
            .extension()
            .and_then(OsStr::to_str)
        {
            Some("xml") => {
                let mut data =
                    from_str::<UiTag>(String::from_utf8(data).unwrap().as_str()).unwrap();
                data.post_process();
                println!("{:#?}", data)
            }
            Some("txt") => {
                if false {
                    /*let decr = decrypt_txt(&mut data);
                    let entries: String = decrypt_exposed_txt(decr);*/
                    let decr = decrypt_txt(data.into_iter()).unwrap();
                    println!("{}", &decr);
                    let sprites = Sprites::parse(decr.as_str()).unwrap();
                    println!("{:#?}", sprites);
                } else {
                    println!(
                        "{}",
                        decrypt_exposed_txt(decrypt_txt(data.into_iter()).unwrap()).unwrap()
                    )
                }
            }
            Some("rle") => {
                let image: RleImage = RleImage::read(&mut Cursor::new(data)).unwrap();
                let path = Path::new(dat_path).with_file_name("res.gif");
                println!("{:?}", path);
                let mut encoder = GifEncoder::new(
                    OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(path)
                        .unwrap(),
                );
                encoder.set_repeat(Repeat::Infinite).unwrap();
                encoder.try_encode_frames(image.into_frames()).unwrap();
            }
            Some("dat") => {
                let image = level_tile_data_to_image(&data).unwrap();
                let path = Path::new(dat_path).with_file_name("res.png");
                println!("{:?}", path);
                image.save_with_format(path, ImageFormat::Png).unwrap();
            }
            Some(ext) => eprintln!("Unknown file extension <{}>", ext),
            None => eprintln!("Failed to read"),
        }
    }
}

// pub fn decr2()
