use godot::{engine::{file_access::ModeFlags, global::Error, resource_saver::SaverFlags, FileAccess, IResourceFormatLoader, ResourceFormatLoader}, prelude::*};

use godot::prelude::*;

use super::rust_script::Rust;

#[derive(GodotClass)]
#[class(base=ResourceFormatLoader,tool,init)]
pub struct RustResourceLoader {
    base: Base<ResourceFormatLoader>,
}

#[godot_api]
impl IResourceFormatLoader for RustResourceLoader {
    fn load(&self, path: GString, original_path: GString, use_sub_threads: bool, cache_mode: i32,) -> Variant {
        let mut rust_script = Rust::new_gd();
        let mut handle = match FileAccess::open(path, ModeFlags::READ) {
            Some(handle) => handle,
            None => {
                return rust_script.to_variant();
            }
        };
        //let _ = rust_script.load(&mut handle, original_path, use_sub_threads, cache_mode);
        rust_script.set_source_code(handle.get_as_text());
        rust_script.to_variant()
    }
    fn get_recognized_extensions(&self,) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        array.push("rs".into());
        array
    }
    fn handles_type(&self, type_: StringName,) -> bool {
        type_ == "Rust".into()
    }
    fn get_resource_type(&self, path: GString,) -> GString {
        "Rust".into()
    }
}
