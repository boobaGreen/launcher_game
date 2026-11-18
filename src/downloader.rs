use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Downloader;

impl Downloader {
    pub fn download_cover(game_id: i64, title: &str, platform: &str) -> Result<PathBuf, String> {
        let cache_dir = PathBuf::from("cache/covers");
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
        }

        let filename = format!("{}_{}.png", game_id, Self::slugify(title));
        let cache_path = cache_dir.join(&filename);

        if cache_path.exists() {
            return Ok(cache_path);
        }

        let url = Self::get_cover_url(title, platform);
        if let Ok(mut resp) = reqwest::blocking::get(&url) {
            if resp.status().is_success() {
                let mut file = fs::File::create(&cache_path).map_err(|e| e.to_string())?;
                let bytes = resp.bytes().map_err(|e| e.to_string())?;
                file.write_all(&bytes).map_err(|e| e.to_string())?;
                return Ok(cache_path);
            }
        }

        Err(format!("Cover non disponibile per: {}", title))
    }

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
        if let Ok(mut resp) = reqwest::blocking::get(&url) {
            if resp.status().is_success() {
                let mut file = fs::File::create(&rom_path).map_err(|e| e.to_string())?;
                let bytes = resp.bytes().map_err(|e| e.to_string())?;
                file.write_all(&bytes).map_err(|e| e.to_string())?;
                return Ok(rom_path);
            }
        }

        Err(format!("ROM non disponibile per: {}", rom_name))
    }

    fn get_cover_url(title: &str, platform: &str) -> String {
        let slug = Self::slugify(title);
        format!(
            "https://raw.githubusercontent.com/libretro-thumbnails/{}/master/Named_Boxarts/{}.png",
            Self::get_libretro_platform(platform),
            slug
        )
    }

    fn get_rom_url(rom_name: &str, platform: &str) -> String {
        format!(
            "https://archive.org/download/roms-{}/{}",
            platform.to_lowercase(),
            rom_name
        )
    }

    fn get_libretro_platform(platform: &str) -> &str {
        match platform.to_uppercase().as_str() {
            "SNES" => "Nintendo_-_Super_Nintendo_Entertainment_System",
            "NES" => "Nintendo_-_Nintendo_Entertainment_System",
            "N64" => "Nintendo_-_Nintendo_64",
            "PS1" => "Sony_-_PlayStation",
            "PSX" => "Sony_-_PlayStation",
            "DREAMCAST" => "Sega_-_Dreamcast",
            "GAMECUBE" => "Nintendo_-_GameCube",
            "C64" => "Commodore_-_Commodore_64",
            "MAME" => "MAME",
            _ => "MAME",
        }
    }

    fn slugify(s: &str) -> String {
        s.to_lowercase()
            .replace([':', '-', '\'', '\"', '!', '?', ',', '.', '(', ')'], " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("_")
    }
}