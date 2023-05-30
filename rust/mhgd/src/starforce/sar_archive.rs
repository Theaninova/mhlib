use crate::lwo::object::lightwave_to_gd;
use dds::DDS;
use godot::bind::{godot_api, GodotClass};
use godot::builtin::{GodotString, PackedByteArray, StringName, ToVariant, Variant};
use godot::engine::global::Error;
use godot::engine::image::Format;
use godot::engine::resource_loader::CacheMode;
use godot::engine::resource_saver::SaverFlags;
use godot::engine::{
    DirAccess, FileAccess, Image, ImageTexture, Resource, ResourceFormatLoader,
    ResourceFormatLoaderVirtual, ResourceLoader, ResourceSaver, Texture2D,
};
use godot::log::godot_error;
use godot::obj::{Base, Share};
use godot::prelude::godot_print;
use lightwave_3d::LightWaveObject;
use starforcelib::sarc::SarcArchive;
use std::io::Cursor;

pub const GAMES_PATH: &str = "res://games/";
pub const INSTALL_PATH: &str = "user://.install/";

pub fn sarc_path_to_gd<T: ToString>(path: &T) -> String {
    format!(
        "{}{}.res",
        INSTALL_PATH,
        path.to_string()
            .replace(['\\', ':'], "/")
            .replace("//", "/")
    )
}

fn files_recursive(path: String) -> Vec<String> {
    DirAccess::get_directories_at(path.clone().into())
        .to_vec()
        .into_iter()
        .map(|it| format!("{}/{}", path, it))
        .flat_map(files_recursive)
        .chain(
            DirAccess::get_files_at(path.clone().into())
                .to_vec()
                .into_iter()
                .map(|it| format!("{}/{}", path, it)),
        )
        .collect()
}

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader)]
pub struct SarLoader {
    pub archive: SarcArchive,
    pub data: Vec<u8>,
    pub target_path: String,

    #[base]
    pub base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl SarLoader {
    pub fn list_installed_files() -> Vec<String> {
        files_recursive(INSTALL_PATH.strip_suffix('/').unwrap().into())
    }

    pub fn resource_path_at(path: String, game: &GodotString) -> GodotString {
        format!(
            "{}/{}",
            GAMES_PATH,
            path.strip_prefix(INSTALL_PATH).unwrap()
        )
        .into()
    }
}

#[godot_api]
impl ResourceFormatLoaderVirtual for SarLoader {
    fn recognize_path(&self, path: GodotString, type_: StringName) -> bool {
        path.to_string().starts_with(INSTALL_PATH) /*|| path.to_string().starts_with(GAMES_PATH)*/
    }

    fn load(
        &self,
        path: GodotString,
        original_path: GodotString,
        use_sub_threads: bool,
        cache_mode: i64,
    ) -> Variant {
        let path = original_path.to_string();
        let original_path = if path.starts_with(GAMES_PATH) {
            path.strip_prefix(GAMES_PATH)
                .unwrap()
                .strip_prefix(&self.target_path)
                .unwrap()
                .strip_prefix('/')
                .unwrap()
        } else {
            path.strip_prefix(INSTALL_PATH).unwrap()
        };
        let internal_path: GodotString = format!("{}{}", INSTALL_PATH, original_path).into();
        let original_path: GodotString = format!(
            "{}{}/{}",
            INSTALL_PATH,
            self.target_path,
            original_path
                .strip_prefix(&format!("{}/", self.target_path))
                .unwrap_or(original_path)
        )
        .into();

        match internal_path
            .to_string()
            .strip_suffix(".res")
            .unwrap_or(&internal_path.to_string())
            .rsplit_once('.')
        {
            Some((_, "lwo")) => {
                let data = self
                    .archive
                    .extract(&mut Cursor::new(&self.data), &internal_path.to_string())
                    .unwrap();
                let obj = LightWaveObject::read(&mut Cursor::new(data)).unwrap();

                let directory = original_path
                    .to_string()
                    .strip_suffix(".res")
                    .unwrap()
                    .to_string();

                DirAccess::make_dir_recursive_absolute(directory.clone().into());
                for mut mesh in lightwave_to_gd(obj) {
                    let name = mesh.get_name();
                    let path = format!("{}/{}.res", directory, name);

                    mesh.set_path(Self::resource_path_at(
                        path.to_string(),
                        &self.target_path.clone().into(),
                    ));
                    println!("{}", path);
                    ResourceSaver::singleton().save(
                        mesh.upcast(),
                        path.into(),
                        SaverFlags::FLAG_CHANGE_PATH,
                    );
                }

                Resource::new().to_variant()
            }
            Some((base, "bmp" | "dds")) => {
                if FileAccess::file_exists(original_path.clone()) {
                    godot_print!("Reusing {}", original_path);
                    return ResourceLoader::singleton()
                        .load(original_path, "".into(), CacheMode::CACHE_MODE_REUSE)
                        .to_variant();
                }

                let mut image = Image::new();
                if let Ok(bmp) = self
                    .archive
                    .extract(&mut Cursor::new(&self.data), &internal_path.to_string())
                {
                    image.load_bmp_from_buffer(PackedByteArray::from(bmp.as_slice()));
                } else if let Ok(dds) = self.archive.extract(
                    &mut Cursor::new(&self.data),
                    format!("{}.bmp.dds.res", &base).as_str(),
                ) {
                    let data = DDS::decode(&mut Cursor::new(dds)).unwrap();
                    image.set_data(
                        data.header.width as i64,
                        data.header.height as i64,
                        false,
                        Format::FORMAT_RGBA8,
                        data.layers[0]
                            .iter()
                            .flat_map(|px| [px.r, px.g, px.b, px.a])
                            .collect(),
                    );
                    image.decompress();
                } else {
                    godot_error!("Could not find {} or {}.bmp.dds.res", original_path, base)
                }

                image.generate_mipmaps(true);

                image.set_name(
                    original_path
                        .to_string()
                        .rsplit_once('/')
                        .unwrap()
                        .1
                        .strip_suffix(".res")
                        .unwrap()
                        .into(),
                );

                println!("{}", original_path);
                // image.set_path(cache_path.to_string().into());
                DirAccess::make_dir_recursive_absolute(
                    original_path.to_string().rsplit_once('/').unwrap().0.into(),
                );

                let mut texture = ImageTexture::new();
                texture.set_name(image.get_name());
                texture.set_image(image);

                let target_path = Self::resource_path_at(
                    original_path.clone().into(),
                    &self.target_path.clone().into(),
                );
                texture.set_meta("target_path".into(), target_path.to_variant());
                texture.set_path(target_path);
                ResourceSaver::singleton().save(
                    texture.share().upcast(),
                    original_path,
                    SaverFlags::FLAG_COMPRESS,
                );

                texture.to_variant()
            }
            None => Error::ERR_FILE_UNRECOGNIZED.to_variant(),
            _ => Error::ERR_BUG.to_variant(),
        }
    }
}
