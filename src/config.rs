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
        // Modalità Vergine per nuovo utente: nessun percorso emulatore agganciato di default
        Self {
            port: 39281,
            emu_root: String::new(),
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
    pub vpx: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub id: String,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiMonitorConfig {
    pub enabled: bool,
    pub layout: Layout,
    pub primary_display: usize,
    #[serde(default = "default_target_display")]
    pub target_display: String,
}

fn default_target_display() -> String {
    "auto".to_string()
}

impl Default for MultiMonitorConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            layout: Layout::Auto,
            primary_display: 0,
            target_display: "auto".to_string(),
        }
    }
}

#[cfg(windows)]
pub fn detect_system_displays() -> Vec<DisplayInfo> {
    use std::ffi::c_void;

    type BOOL = i32;
    type HMONITOR = *mut c_void;
    type HDC = *mut c_void;
    type LPARAM = isize;

    #[repr(C)]
    struct RECT {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[repr(C, align(4))]
    struct MONITORINFOEXW {
        cb_size: u32,
        rc_monitor: RECT,
        rc_work: RECT,
        dw_flags: u32,
        sz_device: [u16; 32],
    }

    type MonitorEnumProc = unsafe extern "system" fn(HMONITOR, HDC, *mut RECT, LPARAM) -> BOOL;

    #[link(name = "user32")]
    extern "system" {
        fn EnumDisplayMonitors(
            hdc: HDC,
            lprc_clip: *const RECT,
            lpfn_enum: MonitorEnumProc,
            dw_data: LPARAM,
        ) -> BOOL;

        fn GetMonitorInfoW(
            h_monitor: HMONITOR,
            lp_mi: *mut MONITORINFOEXW,
        ) -> BOOL;
    }

    unsafe extern "system" fn callback(
        h_monitor: HMONITOR,
        _hdc: HDC,
        _lprc: *mut RECT,
        dw_data: LPARAM,
    ) -> BOOL {
        let list = &mut *(dw_data as *mut Vec<DisplayInfo>);
        let mut info: MONITORINFOEXW = std::mem::zeroed();
        info.cb_size = std::mem::size_of::<MONITORINFOEXW>() as u32;
        if GetMonitorInfoW(h_monitor, &mut info) != 0 {
            let len = info.sz_device.iter().position(|&c| c == 0).unwrap_or(info.sz_device.len());
            let id = String::from_utf16_lossy(&info.sz_device[..len]);
            let width = info.rc_monitor.right - info.rc_monitor.left;
            let height = info.rc_monitor.bottom - info.rc_monitor.top;
            let x = info.rc_monitor.left;
            let y = info.rc_monitor.top;
            let is_primary = (info.dw_flags & 1) != 0;
            let idx = list.len() + 1;
            let name = if is_primary {
                format!("Schermo {} (Principale / Laptop)", idx)
            } else {
                format!("Monitor Esterno {}", idx)
            };
            list.push(DisplayInfo {
                id,
                name,
                width,
                height,
                x,
                y,
                is_primary,
            });
        }
        1
    }

    let mut list: Vec<DisplayInfo> = Vec::new();
    unsafe {
        EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null(),
            callback,
            &mut list as *mut _ as LPARAM,
        );
    }

    // Ordina per coordinata X da sinistra a destra (disposizione fisica naturale)
    list.sort_by_key(|m: &DisplayInfo| m.x);
    for (i, d) in list.iter_mut().enumerate() {
        let idx = i + 1;
        d.name = if d.is_primary {
            format!("Schermo {} (Laptop / Principale)", idx)
        } else {
            format!("Monitor Esterno {}", idx)
        };
    }
    list
}

#[cfg(not(windows))]
pub fn detect_system_displays() -> Vec<DisplayInfo> {
    vec![
        DisplayInfo {
            id: "auto".to_string(),
            name: "Schermo Principale (Predefinito)".to_string(),
            width: 1920,
            height: 1080,
            x: 0,
            y: 0,
            is_primary: true,
        }
    ]
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
            "VPX" => self.emulator_paths.vpx.as_deref(),
            _ => None,
        }
    }

    /// Cerca l'eseguibile di RetroArch sul sistema
    pub fn find_retroarch(&self) -> Option<PathBuf> {
        if let Some(ref p) = self.emulator_paths.retroarch {
            let pb = PathBuf::from(p);
            if pb.exists() {
                return Some(pb);
            }
        }

        let mut candidates = vec![
            PathBuf::from(r"C:\RetroArch-Win64\retroarch.exe"),
            PathBuf::from(r"C:\RetroArch\retroarch.exe"),
            PathBuf::from(r"C:\Program Files\RetroArch\retroarch.exe"),
            PathBuf::from(r"C:\Program Files (x86)\RetroArch\retroarch.exe"),
            PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\RetroArch\retroarch.exe"),
            PathBuf::from(r"emu\RetroArch\retroarch.exe"),
            PathBuf::from(r"emulators\RetroArch\retroarch.exe"),
        ];

        if let Ok(appdata) = std::env::var("APPDATA") {
            candidates.push(PathBuf::from(&appdata).join(r"RetroArch\retroarch.exe"));
        }
        if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
            candidates.push(PathBuf::from(&localappdata).join(r"RetroArch\retroarch.exe"));
        }

        candidates.into_iter().find(|p| p.exists())
    }

    /// Cerca l'eseguibile di Visual Pinball (VPX) sul sistema
    pub fn find_vpx(&self) -> Option<PathBuf> {
        if let Some(ref p) = self.emulator_paths.vpx {
            let pb = PathBuf::from(p);
            if pb.exists() {
                return Some(pb);
            }
        }

        let candidates = [
            PathBuf::from(r"C:\Visual Pinball\VPinballX64.exe"),
            PathBuf::from(r"C:\Visual Pinball\VPinballX.exe"),
            PathBuf::from(r"C:\Visual Pinball\VPinballX_GL64.exe"),
            PathBuf::from(r"C:\vPinball\VPinballX.exe"),
            PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\Visual Pinball\VPinballX.exe"),
            PathBuf::from(r"emu\Visual Pinball\VPinballX.exe"),
            PathBuf::from(r"emulators\VPX\VPinballX.exe"),
        ];

        candidates.into_iter().find(|p| p.exists())
    }

    /// Cerca l'eseguibile di MAME sul sistema
    pub fn find_mame(&self) -> Option<PathBuf> {
        if let Some(ref p) = self.emulator_paths.mame {
            let pb = PathBuf::from(p);
            if pb.exists() {
                return Some(pb);
            }
        }

        let candidates = [
            PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame\mame.exe"),
            PathBuf::from(r"emu\mame\mame.exe"),
            PathBuf::from(r"emulators\mame\mame.exe"),
            PathBuf::from(r"C:\mame\mame.exe"),
        ];

        candidates.into_iter().find(|p| p.exists())
    }

    /// Auto-rileva tutti gli emulatori installati e salva i percorsi rilevati
    pub fn auto_detect_all(&mut self) -> (usize, Vec<String>) {
        let mut detected = Vec::new();

        if let Some(p) = self.find_mame() {
            let s = p.to_string_lossy().to_string();
            detected.push(format!("MAME: {}", s));
            self.emulator_paths.mame = Some(s);
        }
        if let Some(p) = self.find_retroarch() {
            let s = p.to_string_lossy().to_string();
            detected.push(format!("RetroArch: {}", s));
            self.emulator_paths.retroarch = Some(s);
        }
        if let Some(p) = self.find_vpx() {
            let s = p.to_string_lossy().to_string();
            detected.push(format!("Visual Pinball X: {}", s));
            self.emulator_paths.vpx = Some(s);
        }

        let count = detected.len();
        let _ = self.save();
        (count, detected)
    }
}