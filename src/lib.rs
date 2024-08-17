use godot::classes::Engine;
use godot::classes::ResourceFormatLoader;
use godot::classes::ResourceFormatSaver;
use godot::classes::ResourceLoader;
use godot::classes::ResourceSaver;
use godot::classes::ScriptLanguage;
use godot::prelude::*;
use syntax::rust_language::RustLanguage;
use syntax::rust_project_settings::RustProjectSettings;
use syntax::rust_resource_loader::RustResourceLoader;
use syntax::rust_resource_saver::RustResourceSaver;
mod syntax;
#[derive(GodotClass)]
#[class(base=Object,init,tool)]
pub struct RustSyntaxExtensionLibrary {}
#[gdextension]
unsafe impl ExtensionLibrary for RustSyntaxExtensionLibrary {
    fn min_level() -> InitLevel {
        InitLevel::Servers
    }

    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let res_loader = RustResourceLoader::new_gd();
            let res_saver = RustResourceSaver::new_gd();
            let mut engine = Engine::singleton();
            let lang: Gd<RustLanguage> = RustLanguage::new_alloc();
            engine.register_script_language(lang.clone().upcast::<ScriptLanguage>());
            ResourceSaver::singleton()
                .add_resource_format_saver(res_saver.clone().upcast::<ResourceFormatSaver>());
            ResourceLoader::singleton()
                .add_resource_format_loader(res_loader.clone().upcast::<ResourceFormatLoader>());
            RustProjectSettings::register_settings();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {}
    }

    fn override_hot_reload() -> Option<bool> {
        Some(true)
    }
}
