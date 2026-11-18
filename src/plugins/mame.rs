use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::PathBuf;
use std::process::Command;

pub struct MamePlugin {
    mame_path: Option<PathBuf>,
}

impl MamePlugin {
    pub fn new() -> Self {
        Self { mame_path: Self::find_mame() }
    }

    fn find_mame() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\MAME",
            r"C:\Emulators\MAME",
            r"C:\retro\emulators\mame",
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

        let exe_name = if cfg!(windows) {
            "mame.exe"
        } else {
            "mame"
        };

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

impl EmulatorPlugin for MamePlugin {
    fn name(&self) -> &'static str {
        "MAME"
    }

    fn detect_installed(&self) -> bool {
        self.mame_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let mame_exe = self.mame_path.as_ref()
            .ok_or("MAME non installato")?;

        let rom_name = game.launch_target.clone();

        let args = vec![
            "-video", "bgfx",
            &rom_name,
        ];

        Command::new(mame_exe)
            .args(&args)
            .spawn()
            .map(|_| format!("Avviato MAME: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.mame_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".zip", ".7z", ".chd"]
    }

    fn get_platform_name(&self) -> &'static str {
        "MAME"
    }
}