use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct DownloadManager;

impl DownloadManager {
    /// Scarica la ROM al volo e la posiziona in downloads/<piattaforma>/
    pub fn download_rom(_game_id: i64, rom_name: &str, platform: &str) -> Result<PathBuf, String> {
        let clean_name = rom_name.trim();
        if clean_name.is_empty() {
            return Err("Nome ROM non specificato".to_string());
        }

        let download_dir = PathBuf::from(format!("downloads/{}", platform.to_lowercase()));
        if !download_dir.exists() {
            fs::create_dir_all(&download_dir).map_err(|e| e.to_string())?;
        }

        let rom_path = download_dir.join(clean_name);
        if rom_path.exists() {
            return Ok(rom_path);
        }

        // Verifica anche se esiste già in roms/ o emu/
        let local_candidates = [
            PathBuf::from(format!("roms/{}/{}", platform.to_lowercase(), clean_name)),
            PathBuf::from(format!("emu/{}/{}", platform.to_lowercase(), clean_name)),
            PathBuf::from(format!(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\{}\{}", platform.to_lowercase(), clean_name)),
        ];
        for cand in &local_candidates {
            if cand.exists() {
                return Ok(cand.clone());
            }
        }

        let url = Self::get_rom_url(clean_name, platform);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(6))
            .build()
            .map_err(|e| e.to_string())?;

        match client.get(&url).send() {
            Ok(resp) => {
                if resp.status().is_success() {
                    let mut file = fs::File::create(&rom_path).map_err(|e| e.to_string())?;
                    let bytes = resp.bytes().map_err(|e| e.to_string())?;
                    file.write_all(&bytes).map_err(|e| e.to_string())?;
                    Ok(rom_path)
                } else {
                    Err(format!("File '{}' al momento non presente nell'archivio cloud. Puoi copiare la ROM direttamente in downloads/{}/{}.", clean_name, platform.to_lowercase(), clean_name))
                }
            }
            Err(_e) => Err(format!("Download cloud momentaneamente non disponibile per '{}'. Colloca il file in downloads/{}/{}.", clean_name, platform.to_lowercase(), clean_name)),
        }
    }

    /// Rimuove una ROM scaricata per liberare spazio su disco
    pub fn delete_rom(rom_path: &Path) -> Result<(), String> {
        if rom_path.exists() {
            fs::remove_file(rom_path).map_err(|e| format!("Impossibile eliminare il file: {}", e))?;
        }
        Ok(())
    }

    /// Genera il nome file pulito della ROM a partire dal launch target o dal titolo
    pub fn get_rom_filename(title: &str, target: &str, platform: &str) -> String {
        let t = target.trim();
        if let Some(rom) = t.strip_prefix("MAME:") {
            return format!("{}.zip", rom.trim());
        }
        if t.ends_with(".lnk") || t.ends_with(".bat") {
            let base = Path::new(t).file_stem().and_then(|s| s.to_str()).unwrap_or(title);
            let ext = match platform.to_uppercase().as_str() {
                "SNES" => "sfc",
                "NES" => "nes",
                "MEGADRIVE" | "GENESIS" => "md",
                "GB" => "gb",
                "GBC" => "gbc",
                "GBA" => "gba",
                "N64" => "z64",
                "C64" => "d64",
                "MAME" => "zip",
                _ => "zip",
            };
            return format!("{}.{}", base, ext);
        }
        if !t.is_empty() {
            return Path::new(t).file_name().and_then(|s| s.to_str()).unwrap_or(t).to_string();
        }
        let clean_title = title.replace([':', '/', '\\', '?', '*', '"', '<', '>', '|'], "");
        format!("{}.zip", clean_title)
    }

    /// URL CDN / Mirror di riferimento per il download
    pub fn get_rom_url(rom_name: &str, platform: &str) -> String {
        format!(
            "https://archive.org/download/roms-{}/{}",
            platform.to_lowercase(),
            rom_name
        )
    }
}
