use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct DaphnePlugin {
    daphne_path: Option<PathBuf>,
}

impl DaphnePlugin {
    pub fn new() -> Self {
        Self { daphne_path: Self::find_daphne() }
    }

    fn find_daphne() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\Daphne",
            r"C:\Program Files (x86)\Daphne",
            r"C:\Emulators\Arcade\Daphne",
            r"C:\retro\emulators\daphne",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\Daphne",
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

        let exe_name = if cfg!(windows) { "daphne.exe" } else { "daphne" };
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

impl EmulatorPlugin for DaphnePlugin {
    fn name(&self) -> &'static str {
        "DAPHNE"
    }

    fn detect_installed(&self) -> bool {
        self.daphne_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let daphne_exe = self.daphne_path.as_ref()
            .ok_or("Daphne non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        Command::new(daphne_exe)
            .arg("--dvd")
            .arg(&game.launch_target)
            .spawn()
            .map(|_| format!("Avviato Daphne: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.daphne_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".dlg", ".dat"]
    }

    fn get_platform_name(&self) -> &'static str {
        "LASERDISC"
    }
}