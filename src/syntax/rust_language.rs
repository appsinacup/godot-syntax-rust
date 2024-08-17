use std::ffi::c_void;

use godot::classes::native::ScriptLanguageExtensionProfilingInfo;
use godot::classes::EditorInterface;
use godot::classes::Engine;
use godot::classes::FileAccess;
use godot::classes::IScriptLanguageExtension;
use godot::classes::ResourceLoader;
use godot::classes::Script;
use godot::classes::ScriptLanguageExtension;
use godot::classes::Texture2D;
use godot::global::Error;
use godot::prelude::*;

use super::rust_script::Rust;
use super::rust_validation::run_checks;
#[derive(GodotClass)]
#[class(base=ScriptLanguageExtension,tool,init)]
pub struct RustLanguage {
    icon_registered: bool,

    base: Base<ScriptLanguageExtension>,
}
#[godot_api]
impl IScriptLanguageExtension for RustLanguage {
    fn get_name(&self) -> GString {
        GString::from("Rust")
    }

    fn init_ext(&mut self) {
        self.icon_registered = false;
    }

    fn get_type(&self) -> GString {
        GString::from("Rust")
    }

    fn get_extension(&self) -> GString {
        GString::from("rs")
    }

    fn finish(&mut self) {}

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

    fn get_string_delimiters(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("\" \""), GString::from("\' \'")])
    }

    fn get_comment_delimiters(&self) -> PackedStringArray {
        PackedStringArray::from(&[GString::from("//"), GString::from("/* */")])
    }

    fn make_template(
        &self,
        template: GString,
        class_name: GString,
        base_class_name: GString,
    ) -> Option<Gd<Script>> {
        None
    }

    fn get_built_in_templates(&self, object: StringName) -> Array<Dictionary> {
        Array::default()
    }

    fn is_using_templates(&mut self) -> bool {
        false
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
            let _ = validation.insert("valid", "false");
        } else {
            let _ = validation.insert("valid", "true");
        }
        let _ = validation.insert("errors", errors);
        let functions = VariantArray::new();
        let _ = validation.insert("functions", functions);
        let warnings = run_checks(path.to_string(), "warning".to_string());
        let _ = validation.insert("warnings", warnings);
        validation
    }

    fn validate_path(&self, _path: GString) -> GString {
        GString::default()
    }

    fn create_script(&self) -> Option<Gd<Object>> {
        Some(Rust::new_gd().upcast())
    }

    fn has_named_classes(&self) -> bool {
        true
    }

    fn supports_builtin_mode(&self) -> bool {
        true
    }

    fn supports_documentation(&self) -> bool {
        true
    }

    fn can_inherit_from_file(&self) -> bool {
        false
    }

    fn find_function(&self, class_name: GString, function_name: GString) -> i32 {
        0
    }

    fn make_function(
        &self,
        class_name: GString,
        function_name: GString,
        function_args: PackedStringArray,
    ) -> GString {
        GString::default()
    }

    fn open_in_external_editor(&mut self, script: Gd<Script>, line: i32, column: i32) -> Error {
        Error::OK
    }

    fn overrides_external_editor(&mut self) -> bool {
        false
    }

    fn complete_code(&self, code: GString, path: GString, owner: Gd<Object>) -> Dictionary {
        Dictionary::default()
    }

    fn lookup_code(
        &self,
        code: GString,
        symbol: GString,
        path: GString,
        owner: Gd<Object>,
    ) -> Dictionary {
        Dictionary::default()
    }

    fn auto_indent_code(&self, code: GString, from_line: i32, to_line: i32) -> GString {
        code
    }

    fn add_global_constant(&mut self, name: StringName, value: Variant) {}

    fn add_named_global_constant(&mut self, name: StringName, value: Variant) {}

    fn remove_named_global_constant(&mut self, name: StringName) {}

    fn thread_enter(&mut self) {}

    fn thread_exit(&mut self) {}

    fn debug_get_error(&self) -> GString {
        GString::from("")
    }

    fn debug_get_stack_level_count(&self) -> i32 {
        0
    }

    fn debug_get_stack_level_line(&self, level: i32) -> i32 {
        0
    }

    fn debug_get_stack_level_function(&self, level: i32) -> GString {
        GString::from("")
    }

    fn debug_get_stack_level_locals(
        &mut self,
        level: i32,
        max_subitems: i32,
        max_depth: i32,
    ) -> Dictionary {
        Dictionary::default()
    }

    fn debug_get_stack_level_members(
        &mut self,
        level: i32,
        max_subitems: i32,
        max_depth: i32,
    ) -> Dictionary {
        Dictionary::default()
    }

    unsafe fn debug_get_stack_level_instance(&mut self, level: i32) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn debug_get_globals(&mut self, max_subitems: i32, max_depth: i32) -> Dictionary {
        Dictionary::default()
    }

    fn debug_parse_stack_level_expression(
        &mut self,
        level: i32,
        expression: GString,
        max_subitems: i32,
        max_depth: i32,
    ) -> GString {
        GString::from("")
    }

    fn debug_get_current_stack_info(&mut self) -> Array<Dictionary> {
        Array::default()
    }

    fn reload_all_scripts(&mut self) {
        // TODO
    }

    fn reload_tool_script(&mut self, script: Gd<Script>, soft_reload: bool) {
        // TODO
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        PackedStringArray::from(&[self.get_extension()])
    }

    fn get_public_functions(&self) -> Array<Dictionary> {
        Array::default()
    }

    fn get_public_constants(&self) -> Dictionary {
        Dictionary::default()
    }

    fn get_public_annotations(&self) -> Array<Dictionary> {
        Array::default()
    }

    fn profiling_start(&mut self) {}

    fn profiling_stop(&mut self) {}

    unsafe fn profiling_get_accumulated_data(
        &mut self,
        _info_array: *mut ScriptLanguageExtensionProfilingInfo,
        _info_max: i32,
    ) -> i32 {
        0
    }

    unsafe fn profiling_get_frame_data(
        &mut self,
        _info_array: *mut ScriptLanguageExtensionProfilingInfo,
        _info_max: i32,
    ) -> i32 {
        0
    }

    fn frame(&mut self) {
        let icon_path = "res://addons/godot_syntax_rust/Rust.svg";
        if !self.icon_registered {
            self.icon_registered = true;
            let guard = self.base_mut();
            if Engine::singleton().is_editor_hint() && FileAccess::file_exists(icon_path.into()) {
                let editor_interface = EditorInterface::singleton();
                let editor_theme = editor_interface.get_editor_theme();
                if let Some(mut editor_theme) = editor_theme {
                    let mut resource_loader = ResourceLoader::singleton();
                    let texture = resource_loader.load(icon_path.into());
                    if let Some(texture) = texture {
                        editor_theme.set_icon(
                            "Rust".into(),
                            "EditorIcons".into(),
                            texture.cast::<Texture2D>(),
                        );
                    }
                }
            }
            drop(guard);
        }
    }

    fn handles_global_class_type(&self, _type: GString) -> bool {
        false
    }

    fn get_global_class_name(&self, _path: GString) -> Dictionary {
        Dictionary::default()
    }
}
