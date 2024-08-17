use std::ffi::c_void;

use godot::classes::Engine;
use godot::classes::IScriptExtension;
use godot::classes::Script;
use godot::classes::ScriptExtension;
use godot::classes::ScriptLanguage;
use godot::global::Error;
use godot::prelude::*;

use super::rust_language::RustLanguage;
#[derive(GodotClass)]
#[class(base=ScriptExtension,tool,init)]
pub struct Rust {
    source_code: GString,

    base: Base<ScriptExtension>,
}
#[godot_api]
impl IScriptExtension for Rust {
    fn editor_can_reload_from_file(&mut self) -> bool {
        true
    }

    unsafe fn placeholder_erased(&mut self, _placeholder: *mut c_void) {}

    fn can_instantiate(&self) -> bool {
        false
    }

    fn get_base_script(&self) -> Option<Gd<Script>> {
        None
    }

    fn get_global_name(&self) -> StringName {
        StringName::default()
    }

    fn inherits_script(&self, script: Gd<Script>) -> bool {
        false
    }

    fn get_instance_base_type(&self) -> StringName {
        StringName::default()
    }

    unsafe fn instance_create(&self, for_object: Gd<Object>) -> *mut c_void {
        std::ptr::null_mut()
    }

    unsafe fn placeholder_instance_create(&self, for_object: Gd<Object>) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn instance_has(&self, object: Gd<Object>) -> bool {
        false
    }

    fn has_source_code(&self) -> bool {
        true
    }

    fn get_source_code(&self) -> GString {
        self.source_code.clone()
    }

    fn set_source_code(&mut self, code: GString) {
        self.source_code = code;
    }

    fn reload(&mut self, keep_state: bool) -> Error {
        Error::OK
    }

    fn get_documentation(&self) -> Array<Dictionary> {
        Array::default()
    }

    fn get_class_icon_path(&self) -> GString {
        GString::from("res://addons/godot_syntax_rust/Rust.svg")
    }

    fn has_method(&self, method: StringName) -> bool {
        false
    }

    fn has_static_method(&self, method: StringName) -> bool {
        false
    }

    fn get_method_info(&self, method: StringName) -> Dictionary {
        Dictionary::default()
    }

    fn is_tool(&self) -> bool {
        true
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn is_abstract(&self) -> bool {
        true
    }

    fn get_language(&self) -> Option<Gd<ScriptLanguage>> {
        let mut engine = Engine::singleton();
        for language_count in 0..engine.get_script_language_count() {
            let language = engine.get_script_language(language_count);
            if let Some(language) = language {
                if language.clone().try_cast::<RustLanguage>().is_ok() {
                    return Some(language.clone());
                }
            }
        }
        None
    }

    fn has_script_signal(&self, signal: StringName) -> bool {
        false
    }

    fn get_script_signal_list(&self) -> Array<Dictionary> {
        Array::default()
    }

    fn has_property_default_value(&self, property: StringName) -> bool {
        false
    }

    fn get_property_default_value(&self, property: StringName) -> Variant {
        Variant::default()
    }

    fn update_exports(&mut self) {}

    fn get_script_method_list(&self) -> Array<Dictionary> {
        Array::default()
    }

    fn get_script_property_list(&self) -> Array<Dictionary> {
        Array::default()
    }

    fn get_member_line(&self, member: StringName) -> i32 {
        0
    }

    fn get_constants(&self) -> Dictionary {
        Dictionary::default()
    }

    fn get_members(&self) -> Array<StringName> {
        Array::default()
    }

    fn is_placeholder_fallback_enabled(&self) -> bool {
        false
    }

    fn get_rpc_config(&self) -> Variant {
        Variant::default()
    }

    fn setup_local_to_scene(&mut self) {}
}
