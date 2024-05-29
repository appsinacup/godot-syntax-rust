use godot::{engine::{global::Error, IResourceFormatSaver, ResourceFormatSaver}, prelude::*};

use super::rust_script::RustScript;

#[derive(GodotClass)]
#[class(base=ResourceFormatSaver,tool,init)]
pub struct RustResourceFormatSaver {
    base: Base<ResourceFormatSaver>,
}

#[godot_api]
impl IResourceFormatSaver for RustResourceFormatSaver {
    fn save(&mut self, resource: Gd < Resource >, path: GString, flags: u32,) -> Error {
        unimplemented !()
    }
    fn set_uid(&mut self, path: GString, uid: i64,) -> Error {
        Error::OK
    }
    fn recognize(&self, resource: Gd < Resource >,) -> bool {
        if resource.try_cast::<RustScript>().is_ok() {
            return true;
        }
        false
    }
    fn get_recognized_extensions(&self, resource: Gd < Resource >,) -> PackedStringArray {
        let mut extensions = PackedStringArray::new();
        extensions.push("rs".into());
        extensions
    }
    fn recognize_path(&self, resource: Gd < Resource >, path: GString,) -> bool {
        true
    }
}
