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
        let mut words = PackedStringArray::new();
        // reserved words from rust
        // https://doc.rust-lang.org/reference/keywords.html
        // Strict keywords
        words.push("as".into());
        words.push("break".into());
        words.push("const".into());
        words.push("continue".into());
        words.push("crate".into());
        words.push("else".into());
        words.push("enum".into());
        words.push("extern".into());
        words.push("false".into());
        words.push("fn".into());
        words.push("for".into());
        words.push("if".into());
        words.push("impl".into());
        words.push("in".into());
        words.push("let".into());
        words.push("loop".into());
        words.push("match".into());
        words.push("mod".into());
        words.push("move".into());
        words.push("mut".into());
        words.push("pub".into());
        words.push("ref".into());
        words.push("return".into());
        words.push("self".into());
        words.push("Self".into());
        words.push("static".into());
        words.push("struct".into());
        words.push("super".into());
        words.push("trait".into());
        words.push("true".into());
        words.push("type".into());
        words.push("unsafe".into());
        words.push("use".into());
        words.push("where".into());
        words.push("while".into());

        words.push("async".into());
        words.push("await".into());
        words.push("dyn".into());
        // Reserved
        words.push("abstract".into());
        words.push("become".into());
        words.push("box".into());
        words.push("do".into());
        words.push("final".into());
        words.push("macro".into());
        words.push("override".into());
        words.push("priv".into());
        words.push("typeof".into());
        words.push("unsized".into());
        words.push("virtual".into());
        words.push("yield".into());

        words.push("try".into());
        return words;
    }

    fn is_control_flow_keyword(&self, keyword: GString,) -> bool {
        // List of control flow keywords in Rust
        let control_flow_keywords = [
            "if", "else", "loop", "while", "for", "match",
            "break", "continue", "return", "yield",
        ];

        // Check if the provided keyword is in the list of control flow keywords
        control_flow_keywords.contains(&keyword.to_string().as_str())
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
        let mut functions = VariantArray::new();
        functions.push("test".to_variant());
        validation.insert("functions", functions);
        let mut warnings = VariantArray::new();
        warnings.push("warning1".to_variant());
        validation.insert("warnings", warnings);

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
