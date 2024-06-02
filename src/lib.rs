use std::sync::{Mutex, Once};

use godot::{engine::{Engine, ResourceLoader, ResourceSaver}, prelude::*};
use syntax::{rust_language::RustLanguage, rust_resource_loader::RustResourceLoader, rust_resource_saver::RustResourceSaver};

mod syntax;

#[derive(GodotClass)]
#[class(base=Object,init,tool)]
pub struct RustSyntaxExtensionLibrary {}


fn get_instance() -> &'static Gd<RustResourceLoader> {
    static mut INSTANCE: Option<Gd<RustResourceLoader>> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let singleton = RustResourceLoader::new_gd();
            INSTANCE = Some(singleton);
        });
        INSTANCE.as_ref().unwrap()
    }
}

#[gdextension]
unsafe impl ExtensionLibrary for RustSyntaxExtensionLibrary {
    fn min_level() -> InitLevel {
        InitLevel::Servers
    }
    fn on_level_init(level: InitLevel) {
        godot_print!("hello world");
        match level {
            InitLevel::Scene => {
        
                let res_loader = get_instance();
                let res_saver = RustResourceSaver::new_gd();
        
                let mut engine = Engine::singleton();
        
                let lang: Gd<RustLanguage> = RustLanguage::new_alloc();
        
                engine.register_script_language(lang.clone().upcast());
                engine.register_singleton(
                    RustLanguage::class_name().to_string_name(),
                    lang.upcast(),
                );
        
                ResourceSaver::singleton().add_resource_format_saver(res_saver.clone().upcast());
                engine.register_singleton(
                    RustResourceSaver::class_name().to_string_name(),
                    res_saver.upcast(),
                );
        
                ResourceLoader::singleton().add_resource_format_loader(res_loader.clone().upcast());
                engine.register_singleton(
                    RustResourceLoader::class_name().to_string_name(),
                    res_loader.clone().upcast(),
                );
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                //servers::unregister_scene();
            }
            InitLevel::Servers => {
                //servers::unregister_server();
            }
            _ => (),
        }
    }

    fn override_hot_reload() -> Option<bool> {
        Some(true)
    }
}
