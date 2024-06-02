use godot::prelude::*;

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
        godot_print!("hello world");
        match level {
            InitLevel::Scene => {
                //servers::register_scene();
            }
            InitLevel::Servers => {
                //servers::register_server();
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
