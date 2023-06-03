use crate::starforce::sar_archive::{sarc_path_to_gd, SarLoader, GAMES_PATH};
use godot::bind::godot_api;
use godot::builtin::GodotString;
use godot::engine::file_access::ModeFlags;
use godot::engine::global::Error;
use godot::engine::{try_load, DirAccess, FileAccess, PckPacker, Resource, ResourceLoader};
use godot::log::godot_print;
use godot::obj::{Gd, Share};
use godot::prelude::GodotClass;
use itertools::Itertools;
use starforcelib::sarc::SarcArchive;
use std::collections::HashSet;
use std::io::Cursor;
use std::iter::{Chain, FlatMap};
use std::path::Iter;
use std::vec::IntoIter;

/// This is supposedly to be the default.
const KEY: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(GodotClass)]
#[class(init)]
pub struct Mhk3Map {}

#[godot_api]
impl Mhk3Map {
    #[func]
    pub fn get_available_maps() {}

    #[func]
    pub fn test() {}

    #[func]
    pub fn install(path: GodotString, game: GodotString) -> Error {
        let file = if let Some(file) = FileAccess::open(path, ModeFlags::READ) {
            file
        } else {
            return Error::ERR_FILE_NOT_FOUND;
        };

        let data = file.get_buffer(file.get_length()).to_vec();
        let mut archive = match SarcArchive::read(&mut Cursor::new(&data)) {
            Ok(archive) => archive,
            Err(error) => return Error::ERR_FILE_CORRUPT,
        };
        let mut files_to_convert = HashSet::new();
        for file in archive.files.iter_mut() {
            let convert = file.path.ends_with(".lwo")
                || file.path.ends_with(".bmp")
                || file.path.ends_with(".dds");
            file.path = sarc_path_to_gd(&file.path);
            if convert {
                files_to_convert.insert(file.path.clone());
            }
        }

        files_to_convert.remove(
            "user://.install/D/Moorhuhnkart/3dobjects_tracks/track01_steinzeit/pflanzen.lwo.res",
        );
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch2.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch3.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch4.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch5.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch6.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch7.lwo.res");
        files_to_convert.remove("user://.install/D/Moorhuhnkart/3dobjects_extras/rauch8.lwo.res");
        files_to_convert
            .remove("user://.install/D/Moorhuhnkart/3dobjects_extras/schutzschild.lwo.res");
        files_to_convert.remove(
            "user://.install/D/Moorhuhnkart/menu/kart_select/menu_character_moorhuhn.lwo.res",
        );

        let sar_loader = Gd::<SarLoader>::with_base(|base| SarLoader {
            archive,
            data,
            target_path: game.to_string(),
            base,
        });

        ResourceLoader::singleton().add_resource_format_loader(sar_loader.share().upcast(), true);
        for (i, file) in files_to_convert.into_iter().sorted().enumerate() {
            godot_print!("{}x Next up: {}", i, file);
            try_load::<Resource>(file);
        }
        ResourceLoader::singleton().remove_resource_format_loader(sar_loader.upcast());

        let mut packer = PckPacker::new();
        packer.pck_start(format!("user://{}.pck", game).into(), 32, KEY.into(), false);
        for file in SarLoader::list_installed_files() {
            packer.add_file(
                SarLoader::resource_path_at(file.clone(), &game),
                file.clone().into(),
                false,
            );
        }
        packer.flush(true);

        Error::OK
    }
}
