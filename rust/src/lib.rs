use crate::godot::datafile::DatafileLoader;
use ::godot::engine::class_macros::auto_register_classes;
use ::godot::engine::{ResourceFormatLoaderVirtual, ResourceLoader};
use ::godot::init::{gdextension, ExtensionLayer};
use ::godot::prelude::{ExtensionLibrary, Gd, InitHandle, InitLevel, Share};

pub mod formats;
pub mod godot;

struct Main {}

#[gdextension]
unsafe impl ExtensionLibrary for Main {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(InitLevel::Editor, ResourceLoaderLayer { datafile: None });
        true
    }
}

struct ResourceLoaderLayer {
    pub datafile: Option<Gd<DatafileLoader>>,
}

impl ExtensionLayer for ResourceLoaderLayer {
    fn initialize(&mut self) {
        auto_register_classes();

        self.datafile = Some(Gd::<DatafileLoader>::with_base(DatafileLoader::init));

        ResourceLoader::singleton()
            .add_resource_format_loader(self.datafile.as_ref().unwrap().share().upcast(), true);
    }

    fn deinitialize(&mut self) {
        if let Some(datafile) = &self.datafile {
            ResourceLoader::singleton().remove_resource_format_loader(datafile.share().upcast());
            self.datafile = None;
        }
    }
}
