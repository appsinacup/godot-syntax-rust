use godot::{engine::{file_access::ModeFlags, global::Error, resource_saver::SaverFlags, FileAccess, IResourceFormatSaver, ResourceFormatSaver}, prelude::*};

use super::rust_script::Rust;

#[derive(GodotClass)]
#[class(base=ResourceFormatSaver,tool,init)]
pub struct RustResourceSaver {
    base: Base<ResourceFormatSaver>,
}

#[godot_api]
impl IResourceFormatSaver for RustResourceSaver {
    fn save(&mut self, resource: Gd < Resource >, path: GString, flags: u32,) -> Error {
        let mut script: Gd<Rust> = resource.cast();

        godot_print!("saving rust script resource to: {}", path);

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
    fn recognize(&self, resource: Gd < Resource >,) -> bool {
        resource.try_cast::<Rust>().is_ok()
    }
    fn get_recognized_extensions(&self, resource: Gd < Resource >,) -> PackedStringArray {
        let mut extensions = PackedStringArray::new();
        extensions.push("rs".into());
        extensions
    }
}
