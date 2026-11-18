use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct DuckStationPlugin {
    duckstation_path: Option<PathBuf>,
}

impl DuckStationPlugin {
    pub fn new() -> Self {
        Self { duckstation_path: Self::find_duckstation() }
    }

    fn find_duckstation() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\DuckStation",
            r"C:\Program Files (x86)\DuckStation",
            r"C:\Emulators\PS1\DuckStation",
            r"C:\retro\emulators\duckstation",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\PSX\DuckStation",
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

        let exe_names = ["duckstation-qt-cfg-gl.exe", "duckstation-qt-cfg-vulkan.exe", "DuckStation.exe"];
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

impl EmulatorPlugin for DuckStationPlugin {
    fn name(&self) -> &'static str {
        "DUCKSTATION"
    }

    fn detect_installed(&self) -> bool {
        self.duckstation_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let duckstation_exe = self.duckstation_path.as_ref()
            .ok_or("DuckStation non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        // DuckStation netplay with --host flag for hosting
        Command::new(duckstation_exe)
            .arg("--fullscreen")
            .arg(&game.launch_target)
            .spawn()
            .map(|_| format!("Avviato DuckStation: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.duckstation_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".bin", ".cue", ".cdi", ".mdf", ".mds", ".gz", ".zip"]
    }

    fn get_platform_name(&self) -> &'static str {
        "PSX"
    }
}