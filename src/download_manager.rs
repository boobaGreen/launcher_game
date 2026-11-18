use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct DownloadManager;

impl DownloadManager {
    pub fn download_rom(game_id: i64, rom_name: &str, platform: &str) -> Result<PathBuf, String> {
        let download_dir = PathBuf::from(format!("downloads/{}", platform.to_lowercase()));
        if !download_dir.exists() {
            fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;
        }

        let rom_path = download_dir.join(rom_name);
        if rom_path.exists() {
            return Ok(rom_path);
        }

        let url = Self::get_rom_url(rom_name, platform);
        if let Ok(resp) = reqwest::blocking::get(&url) {
            if resp.status().is_success() {
                let mut file = fs::File::create(&rom_path).map_err(|e| e.to_string())?;
                let bytes = resp.bytes().map_err(|e| e.to_string())?;
                file.write_all(&bytes).map_err(|e| e.to_string())?;
                return Ok(rom_path);
            }
        }

        Err(format!("ROM non disponibile per: {}", rom_name))
    }

    pub fn delete_rom(rom_path: &Path) -> Result<(), String> {
        if rom_path.exists() {
            fs::remove_file(rom_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn get_rom_url(rom_name: &str, platform: &str) -> String {
        format!(
            "https://archive.org/download/roms-{}/{}",
            platform.to_lowercase(),
            rom_name
        )
    }
}