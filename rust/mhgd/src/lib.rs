use crate::sproing::datafile::DatafileLoader;
use crate::starforce::sar_archive::SarLoader;
use ::godot::engine::class_macros::auto_register_classes;
use ::godot::engine::{ResourceFormatLoaderVirtual, ResourceLoader};
use ::godot::init::{gdextension, ExtensionLayer};
use ::godot::prelude::{ExtensionLibrary, Gd, InitHandle, InitLevel, Share};

pub mod data_installer;
pub mod lightwave_object;
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
                sarc: None,
            },
        );
        true
    }
}

struct ResourceLoaderLayer {
    pub datafile: Option<Gd<DatafileLoader>>,
    pub sarc: Option<Gd<SarLoader>>,
}

impl ExtensionLayer for ResourceLoaderLayer {
    fn initialize(&mut self) {
        auto_register_classes();

        self.datafile = Some(Gd::<DatafileLoader>::with_base(DatafileLoader::init));
        self.sarc = Some(Gd::<SarLoader>::with_base(SarLoader::init));

        ResourceLoader::singleton()
            .add_resource_format_loader(self.datafile.as_ref().unwrap().share().upcast(), true);
        ResourceLoader::singleton()
            .add_resource_format_loader(self.sarc.as_ref().unwrap().share().upcast(), true);
    }

    fn deinitialize(&mut self) {
        if let Some(datafile) = &self.datafile {
            ResourceLoader::singleton().remove_resource_format_loader(datafile.share().upcast());
            self.datafile = None;
        }
        if let Some(sarc) = &self.sarc {
            ResourceLoader::singleton().remove_resource_format_loader(sarc.share().upcast());
            self.sarc = None;
        }
    }
}
