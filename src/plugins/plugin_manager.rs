use crate::plugins::EmulatorPlugin;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct PluginManager {
    plugins: Arc<Mutex<HashMap<String, Box<dyn EmulatorPlugin>>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Arc::new(Mutex::new(HashMap::new())) }
    }

    pub fn register(&mut self, plugin: Box<dyn EmulatorPlugin>) {
        let name = plugin.name().to_string();
        self.plugins.lock().unwrap().insert(name, plugin);
    }

    pub fn launch_game(&self, platform: &str, game: &crate::models::Game) -> Result<String, String> {
        let plugins = self.plugins.lock().unwrap();
        if let Some(plugin) = plugins.get(platform) {
            plugin.launch_game(game)
        } else {
            Err(format!("Piattaforma '{}' non trovata", platform))
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}