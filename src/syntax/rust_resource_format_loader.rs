use godot::{engine::{file_access::ModeFlags, global::Error, resource_saver::SaverFlags, FileAccess, IResourceFormatLoader, ResourceFormatLoader}, prelude::*};

use super::rust_script::RustScript;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader,tool,init)]
pub struct RustResourceFormatLoader {
    base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl IResourceFormatLoader for RustResourceFormatLoader {
    fn load(&self, path: GString, original_path: GString, use_sub_threads: bool, cache_mode: i32,) -> Variant {
        let mut rust_script = RustScript::new_gd();
        let mut handle = match FileAccess::open(path, ModeFlags::READ) {
            Some(handle) => handle,
            None => {
                return rust_script.to_variant();
            }
        };
        //let _ = rust_script.load(&mut handle, original_path, use_sub_threads, cache_mode);
        rust_script.to_variant()
    }
    fn get_recognized_extensions(&self,) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        array.push("rs".into());
        array
    }
    fn handles_type(&self, type_: StringName,) -> bool {
        type_.to_string() == "RustScript"
    }
    fn get_resource_type(&self, path: GString) -> godot::prelude::GString {
        "RustScript".into()
    }
}
