use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct RomScanner;

impl RomScanner {
    pub fn scan_paths(paths: &[String]) -> HashMap<String, Vec<PathBuf>> {
        let mut found: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for path_str in paths {
            let root = PathBuf::from(path_str);
            if !root.exists() {
                continue;
            }

            for entry in walkdir::WalkDir::new(&root).max_depth(3).into_iter().flatten() {
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if Self::is_rom_extension(&ext_str) {
                            let platform = Self::detect_platform(entry.path());
                            found.entry(platform).or_default().push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }

        found
    }

    fn is_rom_extension(ext: &str) -> bool {
        matches!(
            ext,
            "zip" | "7z" | "sfc" | "smc" | "fig" | "bs" | "swc" | "nes" | "n64" | "z64" | "v64"
                | "iso" | "gcm" | "wbfs" | "ciso" | "cdi" | "gdi" | "lst" | "d64" | "t64" | "prg"
                | "crd" | "g64" | "x64" | "bin" | "cue" | "mdf" | "mds" | "dlg" | "dat"
                | "chd" | "rar" | "tar" | "gz" | "bz2"
        )
    }

    fn detect_platform(path: &Path) -> String {
        let path_str = path.to_string_lossy().to_lowercase();

        if path_str.contains("snes") || path_str.contains("super nintendo") {
            "SNES".to_string()
        } else if path_str.contains("nes") || path_str.contains("famicom") {
            "NES".to_string()
        } else if path_str.contains("n64") || path_str.contains("nintendo 64") {
            "N64".to_string()
        } else if path_str.contains("dreamcast") || path_str.contains("naomi") || path_str.contains("atomiswave") {
            "DREAMCAST".to_string()
        } else if path_str.contains("gamecube") || path_str.contains("wii") {
            "GAMECUBE".to_string()
        } else if path_str.contains("c64") || path_str.contains("commodore") {
            "C64".to_string()
        } else if path_str.contains("ps1") || path_str.contains("playstation") {
            "PS1".to_string()
        } else if path_str.contains("mame") || path_str.contains("arcade") {
            "MAME".to_string()
        } else if path_str.contains("laserdisc") || path_str.contains("daphne") {
            "LASERDISC".to_string()
        } else {
            "UNKNOWN".to_string()
        }
    }
}