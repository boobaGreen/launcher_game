pub use emulator_plugin::EmulatorPlugin;
pub use plugin_manager::PluginManager;
mod emulator_plugin;
mod plugin_manager;
mod mame;
mod gluiz;
mod snes9x;
mod fceux;
mod project64;
mod flycast;
mod dolphin;
mod vice;
mod duckstation;
mod daphne;

pub fn load_plugins() -> PluginManager {
    let mut manager = PluginManager::new();
    manager.register(Box::new(mame::MamePlugin::new()));
    manager.register(Box::new(gluiz::GluizPlugin::new()));
    manager.register(Box::new(snes9x::Snes9xPlugin::new()));
    manager.register(Box::new(fceux::FceuxPlugin::new()));
    manager.register(Box::new(project64::Project64Plugin::new()));
    manager.register(Box::new(flycast::FlycastPlugin::new()));
    manager.register(Box::new(dolphin::DolphinPlugin::new()));
    manager.register(Box::new(vice::VicePlugin::new()));
    manager.register(Box::new(duckstation::DuckStationPlugin::new()));
    manager.register(Box::new(daphne::DaphnePlugin::new()));
    manager
}