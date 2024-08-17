use godot::classes::*;
use godot::prelude::*;
const CARGO_PATH: &str = "interface/external/cargo_path";
#[derive(Debug)]
pub struct RustProjectSettings;
impl RustProjectSettings {
    pub fn register_settings() {
        let mut editor_settings = EditorInterface::singleton().get_editor_settings().unwrap();
        editor_settings.set_setting(CARGO_PATH.to_godot(), "cargo".to_variant());
    }

    fn get_setting_string(p_setting: &str) -> String {
        let editor_settings = EditorInterface::singleton().get_editor_settings().unwrap();
        let setting_value = editor_settings.get_setting(p_setting.into());
        setting_value.to::<String>()
    }

    pub fn get_cargo_path() -> String {
        RustProjectSettings::get_setting_string(CARGO_PATH)
    }
}
