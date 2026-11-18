use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetroHubConfig {
    pub emulator_paths: EmulatorPaths,
    pub performance: PerformanceConfig,
    pub ui: UIConfig,
    pub multi_monitor: MultiMonitorConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmulatorPaths {
    pub mame: Option<String>,
    pub gluiz: Option<String>,
    pub retroarch: Option<String>,
    pub snes9x: Option<String>,
    pub fceux: Option<String>,
    pub project64: Option<String>,
    pub vice: Option<String>,
    pub flycast: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    pub lazy_load_covers: bool,
    pub window_persistence: bool,
    pub cpu_optimization: CpuLevel,
    pub cache_size_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CpuLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UIConfig {
    pub theme: Theme,
    pub compact_mode: bool,
    pub show_previews: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiMonitorConfig {
    pub enabled: bool,
    pub layout: Layout,
    pub primary_display: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Layout {
    Auto,
    Custom,
}

impl RetroHubConfig {
    pub fn load() -> Self {
        let config_path = PathBuf::from("retrohub.json");
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                serde_json::from_str(&content).unwrap_or_else(|_| Self::default())
            } else {
                Self::default()
            }
        } else {
            // Crea config di default se non esiste
            Self::create_default_config();
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_path = PathBuf::from("retrohub.json");
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, json)
    }
    
    fn create_default_config() {
        let default_config = retrohub_json();
        let _ = std::fs::write("retrohub.json", default_config);
    }
}

fn retrohub_json() -> String {
    r#"
{
  "performance": {
    "lazy_load_covers": true,
    "window_persistence": true,
    "cpu_optimization": "low",
    "cache_size_mb": 100
  },
  "multi_monitor": {
    "enabled": false,
    "layout": "auto",
    "primary_display": 0
  },
  "ui": {
    "theme": "dark",
    "compact_mode": false,
    "show_previews": true
  }
}
"#.to_string()
}