use godot::engine::Engine;
use godot::engine::IScriptLanguageExtension;
use godot::engine::Script;
use godot::engine::ScriptLanguageExtension;
use godot::prelude::*;

use super::rust_script::Rust;
use super::rust_validation::run_checks;
#[derive(GodotClass)]
#[class(base=ScriptLanguageExtension,tool,init)]
pub struct RustLanguage {
    current_id: u32,

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
        true
    }

    fn thread_enter(&mut self) {}

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
        let mut words = PackedStringArray::new();
        // Reserved words from Rust
        let reserved_words = [
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
            "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
            "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "async", "await", "dyn", "abstract", "become",
            "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual",
            "yield", "try",
        ];
        for word in reserved_words.iter() {
            words.push((*word).into());
        }
        words
    }

    fn is_control_flow_keyword(&self, keyword: GString) -> bool {
        let control_flow_keywords = [
            "if", "else", "loop", "while", "for", "match", "break", "continue", "return", "yield",
        ];
        control_flow_keywords.contains(&keyword.to_string().as_str())
    }

    fn get_global_class_name(&self, path: GString) -> Dictionary {
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
        path: GString,
        _validate_functions: bool,
        _validate_errors: bool,
        _validate_warnings: bool,
        _validate_safe_lines: bool,
    ) -> Dictionary {
        let mut validation = Dictionary::new();
        let errors = run_checks(path.to_string(), "error".to_string());
        if !errors.is_empty() {
            validation.insert("valid", "false");
        } else {
            validation.insert("valid", "true");
        }
        validation.insert("errors", errors);
        let functions = VariantArray::new();
        validation.insert("functions", functions);
        let warnings = run_checks(path.to_string(), "warning".to_string());
        validation.insert("warnings", warnings);
        validation
    }

    fn debug_get_current_stack_info(&mut self) -> Array<Dictionary> {
        Array::default()
    }

    // godot hook to trigger script reload
    fn reload_all_scripts(&mut self) {}

    fn frame(&mut self) {}
}
