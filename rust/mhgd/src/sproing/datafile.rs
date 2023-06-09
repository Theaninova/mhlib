use crate::sproing::game_object::parse_game_object;
use crate::sproing::image::{load_bmp_as_image_texture, load_rle_as_sprite_frames};
use crate::sproing::sprites::load_sprite_frames;
use crate::sproing::tile_map::{create_tile_map, TileCollision};
use crate::sproing::ui::convert_ui;
use godot::engine::global::Error;
use godot::engine::resource_loader::CacheMode;
use godot::engine::resource_saver::SaverFlags;
use godot::engine::utilities::printerr;
use godot::engine::ImageTexture;
use godot::engine::{AudioStreamOggVorbis, DirAccess, OggPacketSequence, Translation};
use godot::engine::{ResourceFormatLoader, ResourceSaver};
use godot::engine::{ResourceFormatLoaderVirtual, ResourceLoader};
use godot::prelude::*;
use itertools::Itertools;
use springylib::archive::Archive;
use springylib::DatafileFile;
use std::fs::File;
use std::str::FromStr;

const DAT_PATH: &str = "../games/Schatzjäger/data/datafile.dat";

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader)]
pub struct DatafileLoader {
    pub datafile_table: Archive,

    #[base]
    pub base: Base<ResourceFormatLoader>,
}

fn convert_path(path: &GodotString) -> String {
    path.to_string()
        .strip_prefix("datafile://")
        .map(|it| it.replace('/', "\\"))
        .expect("Invalid path")
}

#[godot_api]
impl DatafileLoader {
    fn save_to_cache(&self, resource: Gd<Resource>, path: String) {
        let cache_path = self.get_cache_path(path);
        match DirAccess::make_dir_recursive_absolute(cache_path.rsplit_once('/').unwrap().0.into())
        {
            Error::OK => (),
            error => printerr(error.to_variant(), &[]),
        }
        ResourceSaver::singleton().save(resource, cache_path.into(), SaverFlags::FLAG_NONE);
    }

    fn get_cache_path(&self, path: String) -> String {
        format!(
            "{}/.cache/{}",
            DAT_PATH
                .replace('\\', "/")
                .strip_suffix("datafile.dat")
                .unwrap(),
            path.replace('\\', "/")
        )
    }

    fn retrieve_cache<T>(&self, path: String) -> Option<Gd<T>>
    where
        T: GodotClass + Inherits<Resource>,
    {
        let cache_path = self.get_cache_path(path);
        let type_hint = T::CLASS_NAME;
        if !ResourceLoader::singleton().exists(cache_path.clone().into(), type_hint.into()) {
            return None;
        }
        ResourceLoader::singleton()
            .load(
                cache_path.into(),
                type_hint.into(),
                CacheMode::CACHE_MODE_REUSE,
            )
            .map(|it| it.cast())
    }
}

#[godot_api]
impl ResourceFormatLoaderVirtual for DatafileLoader {
    fn init(base: Base<Self::Base>) -> Self {
        let mut file = File::open(DAT_PATH).unwrap();
        let datafile_table = Archive::read(&mut file).unwrap();

        DatafileLoader {
            base,
            datafile_table,
        }
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[
            "xml".into(),
            "txt".into(),
            "rle".into(),
            "bmp".into(),
            "dat".into(),
        ])
    }

    fn recognize_path(&self, path: GodotString, _type: StringName) -> bool {
        path.to_string().starts_with("datafile://")
    }

    fn get_resource_type(&self, path: GodotString) -> GodotString {
        if path.to_string().ends_with(".dat") {
            "PackedScene".into()
        } else {
            "Resource".into()
        }
    }

    fn get_resource_script_class(&self, _path: GodotString) -> GodotString {
        GodotString::from_str("").unwrap()
    }

    fn exists(&self, path: GodotString) -> bool {
        self.datafile_table
            .contains_key(convert_path(&path).as_str())
    }

    fn get_classes_used(&self, _path: GodotString) -> PackedStringArray {
        PackedStringArray::from(&[])
    }

    fn load(
        &self,
        virtual_path: GodotString,
        _original_path: GodotString,
        _use_sub_threads: bool,
        _cache_mode: i64,
    ) -> Variant {
        let datafile_path = convert_path(&virtual_path);
        if let Some(resource) = self.retrieve_cache::<Resource>(format!(
            "{}.{}",
            datafile_path,
            if datafile_path.ends_with(".xml") || datafile_path.ends_with("dat") {
                "scn"
            } else {
                "res"
            }
        )) {
            return resource.to_variant();
        }

        if let Some(target) = self.datafile_table.get(datafile_path.as_str()) {
            let mut file = File::open(DAT_PATH).unwrap();
            match target.load_from(&mut file) {
                Ok(DatafileFile::Level(level)) => {
                    let level_id = datafile_path
                        .split_terminator('\\')
                        .find(|i| i.starts_with("level"))
                        .map(|lvl| u32::from_str(lvl.strip_prefix("level").unwrap()).unwrap())
                        .unwrap();
                    let tile_map = create_tile_map(level, level_id);

                    self.save_to_cache(tile_map.share().upcast(), format!("{}.scn", datafile_path));
                    tile_map.to_variant()
                }
                Ok(DatafileFile::Txt(txt)) => {
                    let game_object = parse_game_object(txt);
                    self.save_to_cache(
                        game_object.share().upcast(),
                        format!("{}.res", datafile_path),
                    );
                    game_object.to_variant()
                }
                Ok(DatafileFile::Ui(ui)) => {
                    let full_path = virtual_path.to_string();
                    let (_, _, base_path) = full_path
                        .rsplitn(3, '/')
                        .collect_tuple()
                        .expect("Illegal path for UI");
                    let mut ui = convert_ui(ui, base_path);
                    own_children(&mut ui, None);

                    let mut scene = PackedScene::new();
                    scene.pack(ui);

                    self.save_to_cache(scene.share().upcast(), format!("{}.scn", datafile_path));
                    scene.to_variant()
                }
                Ok(DatafileFile::Translations(translations)) => {
                    let mut translation = Translation::new();
                    for (key, message) in translations {
                        translation.add_message(
                            format!("%{}%", key).into(),
                            message.join("\n").into(),
                            "".into(),
                        );
                    }
                    self.save_to_cache(
                        translation.share().upcast(),
                        format!("{}.res", datafile_path),
                    );
                    translation.to_variant()
                }
                Ok(DatafileFile::Vorbis(vorbis)) => {
                    let mut audio = AudioStreamOggVorbis::new();
                    audio.set_loop(true);
                    let mut packet = OggPacketSequence::new();
                    packet.set_packet_data(Array::from(&[Array::from(&[PackedByteArray::from(
                        vorbis.as_slice(),
                    )
                    .to_variant()])]));
                    audio.set_packet_sequence(packet);
                    audio.to_variant()
                }
                Ok(DatafileFile::RleSprite(rle)) => load_rle_as_sprite_frames(*rle).to_variant(),
                Ok(DatafileFile::Sprites(sprites)) => {
                    let sprite_frames = load_sprite_frames(sprites, virtual_path);

                    self.save_to_cache(
                        sprite_frames.share().upcast(),
                        format!("{}.res", datafile_path),
                    );
                    sprite_frames.to_variant()
                }
                Ok(DatafileFile::Bitmap(data)) => {
                    let gd_image = match load_bmp_as_image_texture(data) {
                        Ok(image) => image,
                        Err(err) => return err.to_variant(),
                    };

                    if datafile_path.contains("\\fonts\\") {
                        panic!();
                        /*let font = load_bitmap_font(gd_image);

                        self.save_to_cache(
                            font.share().upcast(),
                            format!("{}.tres", datafile_path),
                        );
                        font.to_variant()*/
                    } else {
                        let mut texture = ImageTexture::new();
                        texture.set_image(gd_image);

                        self.save_to_cache(
                            texture.share().upcast(),
                            format!("{}.res", datafile_path),
                        );
                        texture.to_variant()
                    }
                }
                Ok(DatafileFile::TileCollision(collision)) => {
                    let tile_collision = Gd::new(TileCollision {
                        collision: collision
                            .chars()
                            .filter_map(|c| c.to_digit(10))
                            .map(|d| d as u8)
                            .collect(),
                    });

                    // No need to save this to cache, we only use this internally
                    /*self.save_to_cache(
                        tile_collision.share().upcast(),
                        format!("{}.res", datafile_path),
                    );*/
                    tile_collision.to_variant()
                }
                Err(springylib::error::Error::UnknownFormat(ext)) => {
                    printerr(format!("Unknown format <{}>", ext).to_variant(), &[]);
                    Error::ERR_FILE_UNRECOGNIZED.to_variant()
                }
                Err(springylib::error::Error::InvalidData { info, context }) => {
                    printerr(
                        "Failed to deserialize".to_variant(),
                        &[
                            info.unwrap_or("".to_string()).to_variant(),
                            context.to_variant(),
                        ],
                    );
                    Error::ERR_FILE_CORRUPT.to_variant()
                }
                Err(springylib::error::Error::Custom(message)) => {
                    printerr(message.to_string().to_variant(), &[]);
                    Error::ERR_BUG.to_variant()
                }
                _ => {
                    printerr("Unknown error".to_variant(), &[]);
                    Error::ERR_BUG.to_variant()
                }
            }
        } else {
            printerr("File not found".to_variant(), &[]);
            Error::ERR_FILE_NOT_FOUND.to_variant()
        }
    }
}

fn own_children(node: &mut Gd<Node>, owner: Option<&mut Gd<Node>>) {
    let iter = node.get_children(false);
    let owner = owner.unwrap_or(node);
    for mut child in iter.iter_shared() {
        println!("{:#?}", child);
        child.set_owner(owner.share());
        own_children(&mut child, Some(owner));
    }
}
