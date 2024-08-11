use godot::classes::file_access::ModeFlags;
use godot::classes::resource_saver::SaverFlags;
use godot::classes::FileAccess;
use godot::classes::IResourceFormatSaver;
use godot::classes::ResourceFormatSaver;
use godot::global::Error;
use godot::prelude::*;

use crate::syntax::rust_script::Rust;
#[derive(GodotClass)]
#[class(base=ResourceFormatSaver,tool,init)]
pub struct RustResourceSaver {
    base: Base<ResourceFormatSaver>,
}
#[godot_api]
impl IResourceFormatSaver for RustResourceSaver {
    fn save(&mut self, resource: Gd<Resource>, path: GString, flags: u32) -> Error {
        let mut script: Gd<Rust> = resource.cast();
        if flags as u64 & SaverFlags::CHANGE_PATH.ord() > 0 {
            script.set_path(path.clone());
        }
        let handle = FileAccess::open(path, ModeFlags::WRITE);
        let mut handle = match handle {
            Some(handle) => handle,
            None => {
                return Error::FAILED;
            }
        };
        handle.store_string(script.get_source_code());
        handle.close();
        Error::OK
    }

    fn recognize(&self, resource: Gd<Resource>) -> bool {
        resource.try_cast::<Rust>().is_ok()
    }

    fn get_recognized_extensions(&self, _resource: Gd<Resource>) -> PackedStringArray {
        let mut extensions = PackedStringArray::new();
        extensions.push("rs".into());
        extensions
    }
}
