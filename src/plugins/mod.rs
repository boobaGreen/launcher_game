pub use emulator_plugin::EmulatorPlugin;
pub use plugin_manager::PluginManager;
mod emulator_plugin;
mod plugin_manager;
mod mame;
mod gluiz;

pub fn load_plugins() -> PluginManager {
    let mut manager = PluginManager::new();
    manager.register(Box::new(mame::MamePlugin::new()));
    manager.register(Box::new(gluiz::GluizPlugin::new()));
    manager
}