use godot::classes::file_access::ModeFlags;
use godot::classes::FileAccess;
use godot::classes::IResourceFormatLoader;
use godot::classes::ResourceFormatLoader;
use godot::prelude::*;

use super::rust_script::Rust;
#[derive(GodotClass)]
#[class(base=ResourceFormatLoader,tool,init)]
pub struct RustResourceLoader {
    base: Base<ResourceFormatLoader>,
}
#[godot_api]
impl IResourceFormatLoader for RustResourceLoader {
    fn load(
        &self,
        path: GString,
        _original_path: GString,
        _use_sub_threads: bool,
        _cache_mode: i32,
    ) -> Variant {
        let mut rust_script = Rust::new_gd();
        let handle = match FileAccess::open(path, ModeFlags::READ) {
            Some(handle) => handle,
            None => {
                return rust_script.to_variant();
            }
        };
        rust_script.set_source_code(handle.get_as_text());
        rust_script.to_variant()
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut array = PackedStringArray::new();
        array.push("rs".into());
        array
    }

    fn handles_type(&self, type_: StringName) -> bool {
        type_ == "Rust".into() || type_ == "Script".into()
    }

    fn get_resource_type(&self, path: GString) -> GString {
        if path.to_string().ends_with(".rs") {
            return GString::from("Rust");
        }
        GString::default()
    }
}
