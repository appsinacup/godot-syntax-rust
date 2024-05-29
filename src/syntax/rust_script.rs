use std::ffi::c_void;

use godot::{engine::{global::Error, IScriptExtension, Script, ScriptExtension, ScriptLanguage}, prelude::*};

#[derive(GodotClass)]
#[class(base=ScriptExtension,tool,init)]
pub struct RustScript {
    base: Base<ScriptExtension>,
}

#[godot_api]
impl IScriptExtension for RustScript {

    fn editor_can_reload_from_file(&mut self,) -> bool {
        unimplemented !()
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
    #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
    unsafe fn placeholder_erased(&mut self, placeholder: * mut c_void,) {
        unimplemented !()
    }
    fn can_instantiate(&self,) -> bool {
        unimplemented !()
    }
    fn get_base_script(&self,) -> Option < Gd < Script > > {
        unimplemented !()
    }
    fn get_global_name(&self,) -> StringName {
        unimplemented !()
    }
    fn inherits_script(&self, script: Gd < Script >,) -> bool {
        unimplemented !()
    }
    fn get_instance_base_type(&self,) -> StringName {
        unimplemented !()
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
    #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
    unsafe fn instance_create(&self, for_object: Gd < Object >,) -> * mut c_void {
        unimplemented !()
    }
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" This method has automatically been marked `unsafe` because it accepts raw pointers as parameters."]
    #[doc = r" If Godot does not document any safety requirements, make sure you understand the underlying semantics."]
    unsafe fn placeholder_instance_create(&self, for_object: Gd < Object >,) -> * mut c_void {
        unimplemented !()
    }
    fn instance_has(&self, object: Gd < Object >,) -> bool {
        unimplemented !()
    }
    fn has_source_code(&self,) -> bool {
        unimplemented !()
    }
    fn get_source_code(&self,) -> GString {
        unimplemented !()
    }
    fn set_source_code(&mut self, code: GString,) {
        unimplemented !()
    }
    fn reload(&mut self, keep_state: bool,) -> Error {
        unimplemented !()
    }
    fn get_documentation(&self,) -> Array < Dictionary > {
        unimplemented !()
    }
    fn get_class_icon_path(&self,) -> GString {
        unimplemented !()
    }
    fn has_method(&self, method: StringName,) -> bool {
        unimplemented !()
    }
    fn has_static_method(&self, method: StringName,) -> bool {
        unimplemented !()
    }
    fn get_method_info(&self, method: StringName,) -> Dictionary {
        unimplemented !()
    }
    fn is_tool(&self,) -> bool {
        unimplemented !()
    }
    fn is_valid(&self,) -> bool {
        unimplemented !()
    }
    fn is_abstract(&self,) -> bool {
        unimplemented !()
    }
    fn get_language(&self,) -> Option < Gd < ScriptLanguage > > {
        unimplemented !()
    }
    fn has_script_signal(&self, signal: StringName,) -> bool {
        unimplemented !()
    }
    fn get_script_signal_list(&self,) -> Array < Dictionary > {
        unimplemented !()
    }
    fn has_property_default_value(&self, property: StringName,) -> bool {
        unimplemented !()
    }
    fn get_property_default_value(&self, property: StringName,) -> Variant {
        unimplemented !()
    }
    fn update_exports(&mut self,) {
        unimplemented !()
    }
    fn get_script_method_list(&self,) -> Array < Dictionary > {
        unimplemented !()
    }
    fn get_script_property_list(&self,) -> Array < Dictionary > {
        unimplemented !()
    }
    fn get_member_line(&self, member: StringName,) -> i32 {
        unimplemented !()
    }
    fn get_constants(&self,) -> Dictionary {
        unimplemented !()
    }
    fn get_members(&self,) -> Array < StringName > {
        unimplemented !()
    }
    fn is_placeholder_fallback_enabled(&self,) -> bool {
        unimplemented !()
    }
    fn get_rpc_config(&self,) -> Variant {
        unimplemented !()
    }
    fn setup_local_to_scene(&mut self,) {
        unimplemented !()
    }
}
