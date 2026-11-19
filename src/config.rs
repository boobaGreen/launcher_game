use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetroHubConfig {
    pub emulator_paths: EmulatorPaths,
    pub performance: PerformanceConfig,
    pub ui: UIConfig,
    pub multi_monitor: MultiMonitorConfig,
    pub netplay: NetplayConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetplayConfig {
    pub enable_stun: bool,
    pub stun_servers: Vec<String>,
    pub nat_traversal: bool,
    pub max_ping_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub emu_root: String,
    pub browser_fallback: bool,
}

impl Default for NetplayConfig {
    fn default() -> Self {
        Self {
            enable_stun: true,
            stun_servers: vec![
                "stun.l.google.com:19302".to_string(),
                "stun1.l.google.com:19302".to_string(),
                "stun.cloudflare.com:3478".to_string(),
            ],
            nat_traversal: true,
            max_ping_ms: 200,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        let emu_candidate = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");
        let emu_root = if emu_candidate.exists() {
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu".to_string()
        } else if std::path::Path::new("emu").exists() {
            "emu".to_string()
        } else {
            ".".to_string()
        };

        Self {
            port: 39281,
            emu_root,
            browser_fallback: true,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum CpuLevel {
    #[default]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Theme {
    #[default]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Layout {
    #[default]
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

// Helper per ottenere il percorso emulatore
impl RetroHubConfig {
    pub fn emu_root(&self) -> PathBuf {
        PathBuf::from(&self.server.emu_root)
    }

    pub fn get_emulator_path(&self, name: &str) -> Option<&str> {
        match name {
            "MAME" => self.emulator_paths.mame.as_deref(),
            "GLUIZ" => self.emulator_paths.gluiz.as_deref(),
            "RETROARCH" => self.emulator_paths.retroarch.as_deref(),
            "SNES9X" => self.emulator_paths.snes9x.as_deref(),
            "FCEUX" => self.emulator_paths.fceux.as_deref(),
            "PROJECT64" => self.emulator_paths.project64.as_deref(),
            "VICE" => self.emulator_paths.vice.as_deref(),
            "FLYCAST" => self.emulator_paths.flycast.as_deref(),
            _ => None,
        }
    }
}