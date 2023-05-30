use crate::sproing::datafile::DatafileLoader;
use crate::starforce::sar_archive::{GAMES_PATH, INSTALL_PATH};
use ::godot::engine::class_macros::auto_register_classes;
use ::godot::engine::{ResourceFormatLoaderVirtual, ResourceLoader};
use ::godot::init::{gdextension, ExtensionLayer};
use ::godot::prelude::{ExtensionLibrary, Gd, InitHandle, InitLevel, Share};
use godot::bind::godot_api;
use godot::builtin::{GodotString, StringName, ToVariant, Variant};
use godot::engine::global::Error;
use godot::engine::resource_loader::CacheMode;
use godot::engine::{Engine, ResourceFormatLoader};
use godot::log::godot_error;
use godot::obj::EngineEnum;
use godot::prelude::{Base, GodotClass};

pub mod data_installer;
pub mod lwo;
pub mod pr3d;
pub mod sproing;
pub mod starforce;

struct Main {}

#[gdextension]
unsafe impl ExtensionLibrary for Main {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(
            InitLevel::Editor,
            ResourceLoaderLayer {
                datafile: None,
                editor_pck: None,
            },
        );
        true
    }
}

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader)]
struct EditorPck {
    #[base]
    pub base: Base<ResourceFormatLoader>,
}
#[godot_api]
impl ResourceFormatLoaderVirtual for EditorPck {
    fn recognize_path(&self, path: GodotString, type_: StringName) -> bool {
        path.to_string().starts_with(GAMES_PATH)
    }

    fn load(
        &self,
        path: GodotString,
        original_path: GodotString,
        use_sub_threads: bool,
        cache_mode: i64,
    ) -> Variant {
        let original_path = format!(
            "{}{}",
            INSTALL_PATH,
            original_path.to_string().strip_prefix(GAMES_PATH).unwrap()
        );
        if let Some(resource) = ResourceLoader::singleton().load(
            original_path.clone().into(),
            "".into(),
            CacheMode::from_ord(cache_mode as i32),
        ) {
            resource.to_variant()
        } else {
            godot_error!("Could not find {}", original_path);
            Error::ERR_FILE_NOT_FOUND.to_variant()
        }
    }
}

struct ResourceLoaderLayer {
    pub datafile: Option<Gd<DatafileLoader>>,
    pub editor_pck: Option<Gd<EditorPck>>,
}

impl ExtensionLayer for ResourceLoaderLayer {
    fn initialize(&mut self) {
        auto_register_classes();

        self.datafile = Some(Gd::<DatafileLoader>::with_base(DatafileLoader::init));
        self.editor_pck = Some(Gd::<EditorPck>::with_base(|base| EditorPck { base }));

        if Engine::singleton().is_editor_hint() {
            ResourceLoader::singleton().add_resource_format_loader(
                self.editor_pck.as_ref().unwrap().share().upcast(),
                true,
            )
        }

        ResourceLoader::singleton()
            .add_resource_format_loader(self.datafile.as_ref().unwrap().share().upcast(), true);
    }

    fn deinitialize(&mut self) {
        if let Some(datafile) = &self.datafile {
            ResourceLoader::singleton().remove_resource_format_loader(datafile.share().upcast());
            self.datafile = None;
        }
        if let Some(editor_pck) = &self.editor_pck {
            ResourceLoader::singleton().remove_resource_format_loader(editor_pck.share().upcast());
            self.editor_pck = None;
        }
    }
}
