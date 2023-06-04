use crate::starforce::sar_archive::{sarc_path_to_gd, SarLoader, INSTALL_PATH};
use godot::bind::godot_api;
use godot::builtin::{GodotString, ToVariant, Variant};
use godot::engine::file_access::ModeFlags;
use godot::engine::global::Error;
use godot::engine::{try_load, FileAccess, PckPacker, Resource, ResourceLoader};
use godot::log::godot_print;
use godot::obj::{Gd, Share};
use godot::prelude::GodotClass;
use itertools::Itertools;
use starforcelib::sarc::SarcArchive;
use std::collections::HashSet;
use std::io::Cursor;
use std::iter::Enumerate;
use std::vec::IntoIter;

/// This is supposedly to be the default.
const KEY: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(GodotClass)]
#[class(init)]
pub struct Mhk3Map {
    #[export]
    pub progress: f32,
    pub total_files: i32,
    files_to_convert: Option<Enumerate<IntoIter<String>>>,
    loader: Option<Gd<SarLoader>>,
    game: GodotString,
}

#[godot_api]
impl Mhk3Map {
    #[func]
    pub fn tick_install(&mut self) -> Variant {
        self.files_to_convert
            .as_mut()
            .map(|it| {
                it.next()
                    .map(|(i, file)| {
                        godot_print!("Loading {}", file);
                        try_load::<Resource>(&file);
                        self.progress = i as f32 / self.total_files as f32;
                        file.strip_prefix(INSTALL_PATH)
                            .unwrap()
                            .rsplit_once('.')
                            .unwrap()
                            .0
                            .to_variant()
                    })
                    .unwrap_or_else(Variant::nil)
            })
            .unwrap_or_else(|| Error::FAILED.to_variant())
    }

    #[func]
    pub fn start_install(&mut self, path: GodotString, game: GodotString) -> Error {
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

        self.total_files = files_to_convert.len() as i32;
        self.files_to_convert = Some(files_to_convert.into_iter().sorted().enumerate());

        ResourceLoader::singleton().add_resource_format_loader(sar_loader.share().upcast(), true);
        self.loader = Some(sar_loader);
        self.game = game;
        Error::OK
    }

    #[func]
    pub fn end_install(&mut self) -> Error {
        let sar_loader = if let Some(loader) = self.loader.take() {
            loader
        } else {
            return Error::FAILED;
        };
        ResourceLoader::singleton().remove_resource_format_loader(sar_loader.upcast());

        let mut packer = PckPacker::new();
        packer.pck_start(
            format!("user://{}.pck", self.game).into(),
            32,
            KEY.into(),
            false,
        );
        for file in SarLoader::list_installed_files() {
            packer.add_file(
                SarLoader::resource_path_at(file.clone(), &self.game),
                file.clone().into(),
                false,
            );
        }
        packer.flush(true);

        Error::OK
    }
}
