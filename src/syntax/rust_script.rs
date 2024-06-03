
use godot::{engine::{Engine, IScriptExtension, Script, ScriptExtension, ScriptLanguage}, prelude::*};

use super::rust_language::RustLanguage;

#[derive(GodotClass)]
#[class(base=ScriptExtension,tool,init)]
pub struct Rust {
    class_name: GString,
    source_code: GString,

    base: Base<ScriptExtension>,
}

#[godot_api]
impl IScriptExtension for Rust {
    fn get_global_name(&self) -> StringName {
        self.class_name.clone().into()
    }

    fn get_source_code(&self) -> GString {
        self.source_code.clone()
    }
    fn set_source_code(&mut self, code: GString) {
        self.source_code = code;
    }

    fn get_language(&self) -> Option<Gd<ScriptLanguage>> {
        let singleton = Engine::singleton()
        .get_singleton(RustLanguage::class_name().to_string_name());
        if let Some(singleton) = singleton {
            return Some(singleton.cast());
        }
        None
    }

    fn can_instantiate(&self) -> bool {
        self.is_tool() || !Engine::singleton().is_editor_hint()
    }

    fn get_base_script(&self) -> Option<Gd<Script>> {
        None
    }

    fn is_tool(&self) -> bool {
        true
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn has_property_default_value(&self, _property: StringName) -> bool {
        false
    }

    fn is_abstract(&self) -> bool {
        true
    }

    fn update_exports(&mut self) {
    }

    fn get_constants(&self) -> Dictionary {
        Dictionary::new()
    }
    fn editor_can_reload_from_file(&mut self) -> bool {
        true
    }
}
