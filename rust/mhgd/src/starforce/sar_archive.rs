use crate::lwo::object::lightwave_to_gd;
use dds::{Compression, PixelFormat, DDS};
use godot::bind::{godot_api, GodotClass};
use godot::builtin::{GodotString, PackedByteArray, StringName, ToVariant, Variant};
use godot::engine::global::Error;
use godot::engine::image::Format;
use godot::engine::{Image, ImageTexture, ResourceFormatLoader, ResourceFormatLoaderVirtual};
use godot::obj::{Base, Gd};
use lightwave_3d::LightWaveObject;
use starforcelib::sarc::SarcArchive;
use std::fs::File;
use std::io::Cursor;

const SAR_PATH: &str = r#"../games/Moorhuhn Kart 3/data.sar"#;

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader)]
pub struct SarLoader {
    pub archive: SarcArchive,

    #[base]
    pub base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl SarLoader {}

#[godot_api]
impl ResourceFormatLoaderVirtual for SarLoader {
    fn init(base: Base<Self::Base>) -> Self {
        let mut file = File::open(SAR_PATH).unwrap();
        let archive = SarcArchive::read(&mut file).unwrap();

        Self { base, archive }
    }

    fn recognize_path(&self, path: GodotString, type_: StringName) -> bool {
        path.to_string().starts_with("sar://")
    }

    fn load(
        &self,
        path: GodotString,
        original_path: GodotString,
        use_sub_threads: bool,
        cache_mode: i64,
    ) -> Variant {
        let internal_path = original_path
            .to_string()
            .strip_prefix("sar://")
            .unwrap()
            .replace('/', "\\");

        match path.to_string().rsplit_once('.') {
            Some((_, "lwo")) => {
                let mut f = File::open(SAR_PATH).unwrap();
                let data = self
                    .archive
                    .extract(&mut f, internal_path.as_str())
                    .unwrap();
                let obj = LightWaveObject::read(&mut Cursor::new(data)).unwrap();
                lightwave_to_gd(obj).to_variant()
            }
            Some((_, "bmp" | "dds")) => {
                let mut f = File::open(SAR_PATH).unwrap();

                let mut image = Image::new();
                if let Ok(bmp) = self.archive.extract(&mut f, internal_path.as_str()) {
                    image.load_bmp_from_buffer(PackedByteArray::from(bmp.as_slice()));
                } else if let Ok(dds) = self
                    .archive
                    .extract(&mut f, format!("{}.dds", internal_path).as_str())
                {
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
                    )
                }
                image.to_variant()
            }
            None => Error::ERR_FILE_UNRECOGNIZED.to_variant(),
            _ => Error::ERR_BUG.to_variant(),
        }
    }
}
