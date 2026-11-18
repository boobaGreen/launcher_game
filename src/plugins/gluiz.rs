use crate::models::Game;
use crate::plugins::EmulatorPlugin;
use std::path::PathBuf;
use std::process::Command;

pub struct GluizPlugin {
    gluiz_path: Option<PathBuf>,
}

impl GluizPlugin {
    pub fn new() -> Self {
        Self { gluiz_path: Self::find_gluiz() }
    }

    fn find_gluiz() -> Option<PathBuf> {
        let common_paths = [
            r"C:\Program Files\Gluiz",
            r"C:\Emulators\Gluiz",
            r"C:\retro\suite",
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
            "gluiz.exe"
        } else {
            "gluiz"
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

impl EmulatorPlugin for GluizPlugin {
    fn name(&self) -> &'static str {
        "GLUIZ"
    }

    fn detect_installed(&self) -> bool {
        self.gluiz_path.is_some()
    }

    fn launch_game(&self, game: &Game) -> Result<String, String> {
        let gluiz_exe = self.gluiz_path.as_ref()
            .ok_or("Gluiz non installato")?;

        let rom_path = PathBuf::from(&game.launch_target);

        Command::new(gluiz_exe)
            .current_dir(rom_path.parent().unwrap_or(PathBuf::from(".").as_path()))
            .spawn()
            .map(|_| format!("Avviato Gluiz: {}", game.title))
            .map_err(|e| e.to_string())
    }

    fn get_default_path(&self) -> Option<PathBuf> {
        self.gluiz_path.clone()
    }

    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec![".exe", ".zip", ".7z"]
    }

    fn get_platform_name(&self) -> &'static str {
        "GLUIZ"
    }
}