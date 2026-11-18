use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct FlycastPlugin {
    flycast_path: Option<PathBuf>,
}

impl FlycastPlugin {
    pub fn new() -> Self {
        Self { flycast_path: Self::find_flycast() }
    }

    fn find_flycast() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\Flycast",
            r"C:\Program Files (x86)\Flycast",
            r"C:\Emulators\Flycast",
            r"C:\retro\emulators\flycast",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\Flycast",
        ];

        for path in common_paths.iter() {
            let p = PathBuf::from(path);
            if let Some(exe) = Self::find_executable_in_dir(&p) {
                return Some(exe);
            }
        }
        None
    }

    fn find_executable_in_dir(dir: &PathBuf) -> Option<PathBuf> {
        if !dir.exists() { return None; }

        // Try flycast.exe first, then re Dreamcast
        let exe_names = ["flycast.exe", "re.exe", "retroarch.exe"];
        for exe_name in exe_names {
            let exe_path = dir.join(exe_name);
            if exe_path.exists() {
                return Some(exe_path);
            }
        }

        let subdirs: Vec<_> = std::fs::read_dir(dir)
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

        for subdir in subdirs {
            if let Some(exe) = Self::find_executable_in_dir(&subdir) {
                return Some(exe);
            }
        }
        None
    }
}

impl EmulatorPlugin for FlycastPlugin {
    fn name(&self) -> &'static str {
        "FLYCAST"
    }

    fn detect_installed(&self) -> bool {
        self.flycast_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let flycast_exe = self.flycast_path.as_ref()
            .ok_or("Flycast non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        // Flycast/RetroArch netplay args for Dreamcast/NAOMI/Atomiswave
        let args = vec![
            "-config", "netplay_enable = true",
            &game.launch_target,
        ];

        Command::new(flycast_exe)
            .args(&args)
            .spawn()
            .map(|_| format!("Avviato Flycast: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.flycast_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".cdi", ".gdi", ".lst", ".zip"]
    }

    fn get_platform_name(&self) -> &'static str {
        "DREAMCAST"
    }
}