use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct VicePlugin {
    vice_path: Option<PathBuf>,
}

impl VicePlugin {
    pub fn new() -> Self {
        Self { vice_path: Self::find_vice() }
    }

    fn find_vice() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\Vice",
            r"C:\Program Files (x86)\Vice",
            r"C:\Emulators\C64\Vice",
            r"C:\retro\emulators\vice",
            r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\C64\Vice",
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

        // Try different VICE emulators
        let exe_names = ["x64.exe", "x64sc.exe", "vice.exe"];
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

impl EmulatorPlugin for VicePlugin {
    fn name(&self) -> &'static str {
        "VICE"
    }

    fn detect_installed(&self) -> bool {
        self.vice_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let vice_exe = self.vice_path.as_ref()
            .ok_or("VICE non installato")?;

        let rom_path = Path::new(&game.launch_target);
        if !rom_path.exists() {
            return Err(format!("ROM non trovata: {}", game.launch_target));
        }

        // VICE C64 emulator with drive
        Command::new(vice_exe)
            .arg("--drive8type")
            .arg("1541")
            .arg(&game.launch_target)
            .spawn()
            .map(|_| format!("Avviato VICE (C64): {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.vice_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".d64", ".t64", ".prg", ".crd", ".g64", ".x64"]
    }

    fn get_platform_name(&self) -> &'static str {
        "C64"
    }
}