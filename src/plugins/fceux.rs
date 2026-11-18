use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct FceuxPlugin {
    fceux_path: Option<PathBuf>,
}

impl FceuxPlugin {
    pub fn new() -> Self {
        Self { fceux_path: Self::find_fceux() }
    }

    fn find_fceux() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\FCEUX",
            r"C:\Program Files (x86)\FCEUX",
            r"C:\Emulators\NES\fceux",
            r"C:\retro\emulators\fceux",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\NES\fceux",
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

        let exe_name = if cfg!(windows) { "fceux.exe" } else { "fceux" };
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

impl EmulatorPlugin for FceuxPlugin {
    fn name(&self) -> &'static str {
        "FCEUX"
    }

    fn detect_installed(&self) -> bool {
        self.fceux_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let fceux_exe = self.fceux_path.as_ref()
            .ok_or("FCEUX non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        Command::new(fceux_exe)
            .arg(&game.launch_target)
            .spawn()
            .map(|_| format!("Avviato FCEUX: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.fceux_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".nes", ".zip"]
    }

    fn get_platform_name(&self) -> &'static str {
        "NES"
    }
}