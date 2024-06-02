use godot::{engine::{global::Error, Engine, IScriptLanguageExtension, Script, ScriptLanguageExtension}, prelude::*};

use super::rust_script::Rust;

#[derive(GodotClass)]
#[class(base=ScriptLanguageExtension,tool,init)]
pub struct RustLanguage {
    base: Base<ScriptLanguageExtension>,
}

impl RustLanguage {
    pub fn singleton() -> Option<Gd<Self>> {
        Engine::singleton()
            .get_singleton(RustLanguage::class_name().to_string_name())
            .map(|gd| gd.cast())
    }
}

#[godot_api]
impl IScriptLanguageExtension for RustLanguage {

    fn get_name(&self) -> GString {
        GString::from("Rust")
    }

    fn get_type(&self) -> GString {
        GString::from("Rust")
    }

    fn get_extension(&self) -> GString {
        GString::from("rs")
    }

    fn supports_documentation(&self) -> bool {
        false
    }

    /// thread enter hook will be called before entering a thread
    fn thread_enter(&mut self) {}

    /// thread exit hook will be called before leaving a thread
    fn thread_exit(&mut self) {}

    fn get_public_functions(&self) -> Array<Dictionary> {
        let mut array = Array::new();
        let mut dictionary = Dictionary::new();
        dictionary.insert("test".into_godot(), Variant::from(1));
        array.push(dictionary);
        array
    }

    fn get_public_constants(&self) -> Dictionary {
        Dictionary::new()
    }

    fn get_public_annotations(&self) -> Array<Dictionary> {
        Array::new()
    }

    fn handles_global_class_type(&self, type_: GString) -> bool {
        type_ == self.get_type()
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[self.get_extension()])
    }

    fn has_named_classes(&self) -> bool {
        false
    }

    fn supports_builtin_mode(&self) -> bool {
        false
    }

    fn can_inherit_from_file(&self) -> bool {
        false
    }

    fn is_using_templates(&mut self) -> bool {
        false
    }

    /// validate that the path of a new rust script is valid. Constraints for script locations can be enforced here.
    fn validate_path(&self, path: GString) -> GString {
        GString::new()
    }

    fn make_template(
        &self,
        _template: GString,
        _class_name: GString,
        _base_class_name: GString,
    ) -> Option<Gd<Script>> {
        Some(Rust::new_gd().upcast())
    }

    fn create_script(&self) -> Option<Gd<Object>> {
        Some(Rust::new_gd().upcast())
    }

    fn get_reserved_words(&self) -> PackedStringArray {
        PackedStringArray::new()
    }

    fn get_global_class_name(&self, path: GString) -> Dictionary {
        /*
        let class_name = Self::path_to_class_name(&path);

        let Some(script) = Self::script_meta_data(&class_name) else {
            return Dictionary::new();
        };

        Dictionary::new().apply(|dict| {
            dict.set("name", class_name);
            dict.set("base_type", script.base_type_name());
        })
         */

        Dictionary::new()
    }

    fn overrides_external_editor(&mut self) -> bool {
        false
    }

    fn get_string_delimiters(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("\"")])
    }

    fn get_comment_delimiters(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("//")])
    }

    fn validate(
        &self,
        _script: GString,
        _path: GString,
        _validate_functions: bool,
        _validate_errors: bool,
        _validate_warnings: bool,
        _validate_safe_lines: bool,
    ) -> Dictionary {
        let mut validation = Dictionary::new();

        validation.insert("valid", "true");
        validation.insert("errors", VariantArray::new());
        validation.insert("functions", VariantArray::new());
        validation.insert("warnings", VariantArray::new());

        validation
    }

    fn debug_get_current_stack_info(&mut self) -> Array<Dictionary> {
        Array::default()
    }

    // godot hook to trigger script reload
    fn reload_all_scripts(&mut self) {}

    fn frame(&mut self) {
    }
}
