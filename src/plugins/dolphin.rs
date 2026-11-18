use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct DolphinPlugin {
    dolphin_path: Option<PathBuf>,
}

impl DolphinPlugin {
    pub fn new() -> Self {
        Self { dolphin_path: Self::find_dolphin() }
    }

    fn find_dolphin() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\Dolphin",
            r"C:\Program Files (x86)\Dolphin",
            r"C:\Emulators\GameCube\Dolphin",
            r"C:\retro\emulators\dolphin",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\GC\Dolphin",
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

        let exe_name = if cfg!(windows) { "Dolphin.exe" } else { "Dolphin" };
        let exe_path = dir.join(exe_name);
        if exe_path.exists() {
            return Some(exe_path);
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

impl EmulatorPlugin for DolphinPlugin {
    fn name(&self) -> &'static str {
        "DOLPHIN"
    }

    fn detect_installed(&self) -> bool {
        self.dolphin_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let dolphin_exe = self.dolphin_path.as_ref()
            .ok_or("Dolphin non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        // Dolphin netplay args
        let args = vec![
            "--config", "NetPlayEnable = True",
            "--batch",
            &game.launch_target,
        ];

        Command::new(dolphin_exe)
            .args(&args)
            .spawn()
            .map(|_| format!("Avviato Dolphin: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.dolphin_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".iso", ".gcm", ".wbfs", ".ciso", ".zip"]
    }

    fn get_platform_name(&self) -> &'static str {
        "GAMECUBE"
    }
}