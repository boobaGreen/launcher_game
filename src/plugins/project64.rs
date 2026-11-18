use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Project64Plugin {
    project64_path: Option<PathBuf>,
}

impl Project64Plugin {
    pub fn new() -> Self {
        Self { project64_path: Self::find_project64() }
    }

    fn find_project64() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\Project64",
            r"C:\Program Files (x86)\Project64",
            r"C:\Emulators\N64\Project64",
            r"C:\retro\emulators\project64",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\N64\Project64",
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

        let exe_name = if cfg!(windows) { "Project64.exe" } else { "Project64" };
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

impl EmulatorPlugin for Project64Plugin {
    fn name(&self) -> &'static str {
        "PROJECT64"
    }

    fn detect_installed(&self) -> bool {
        self.project64_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let project64_exe = self.project64_path.as_ref()
            .ok_or("Project64 non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        Command::new(project64_exe)
            .arg(&game.launch_target)
            .spawn()
            .map(|_| format!("Avviato Project64: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.project64_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".n64", ".z64", ".v64", ".zip"]
    }

    fn get_platform_name(&self) -> &'static str {
        "N64"
    }
}