use crate::models::Game;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct CatalogLoader;

impl CatalogLoader {
    /// Loads all games from the Master HTML Catalog using a pre-cached directory index for ultra-fast startup (<100ms).
    pub fn load_from_master_catalog(emu_root: &Path) -> Result<Vec<Game>, String> {
        let cat_path = emu_root.join("CATALOGO GENERALE RETROGAMING.html");
        if !cat_path.exists() {
            return Err(format!("File catalogo non trovato in: {:?}", cat_path));
        }

        let content = fs::read_to_string(&cat_path)
            .map_err(|e| format!("Impossibile leggere il catalogo master: {}", e))?;

        // Pre-build directory shortcut caches once
        let dir_cache = PreloadedDirCache::build(emu_root);

        let tbody_start = content.find("<tbody>").unwrap_or(0);
        let tbody_end = content.rfind("</tbody>").unwrap_or(content.len());
        let tbody = &content[tbody_start..tbody_end];

        let row_re = Regex::new(r#"<tr\s+data-system="([^"]+)">\s*<td>(\d+)</td>\s*<td><span[^>]*>([^<]+)</span></td>\s*<td><span class="title-text">([^<]+)</span></td>\s*<td><span[^>]*>([^<]+)</span></td>\s*<td><span[^>]*>([^<]+)</span></td>\s*<td>([^<]*)</td>\s*<td><div[^>]*>(.*?)</div></td>\s*</tr>"#).map_err(|e| e.to_string())?;

        let mut games = Vec::new();

        for cap in row_re.captures_iter(tbody) {
            let sys_code = cap.get(1).map_or("", |m| m.as_str()).trim();
            let id: i64 = cap.get(2).map_or("0", |m| m.as_str()).parse().unwrap_or(0);
            let platform = cap.get(3).map_or("", |m| m.as_str()).trim();
            let title = html_decode(cap.get(4).map_or("", |m| m.as_str()).trim());
            let year: u16 = cap.get(5).map_or("1990", |m| m.as_str()).trim().parse().unwrap_or(1990);
            let genre = html_decode(cap.get(6).map_or("", |m| m.as_str()).trim());
            let dev = html_decode(cap.get(7).map_or("", |m| m.as_str()).trim());
            let raw_controls = cap.get(8).map_or("", |m| m.as_str());
            let controls = clean_controls_text(raw_controls);

            let launch_target = dir_cache.resolve(emu_root, sys_code, &title);
            let cover_url = Self::build_cover_url(sys_code, &title);

            games.push(Game {
                id,
                title,
                platform: platform.to_string(),
                system_code: sys_code.to_string(),
                year,
                genre,
                developer: dev.clone(),
                publisher: dev,
                controls,
                launch_target,
                cover_url: Some(cover_url),
                is_favorite: false,
                play_count: 0,
                last_played_at: None,
            });
        }

        Ok(games)
    }

    fn build_cover_url(sys_code: &str, title: &str) -> String {
        // Strip trailing arcade ROM codes like " ( demofist )" or " ( mslug6 )"
        let mut clean = title.trim();
        if let Some(open_p) = clean.rfind('(') {
            if let Some(close_p) = clean[open_p..].find(')') {
                let inside = clean[open_p + 1..open_p + close_p].trim();
                // If it's a short alphanumeric token or romcode without spaces, or starts with known rom code
                if inside.len() <= 12 && !inside.contains("19") && !inside.contains("20") {
                    clean = clean[..open_p].trim();
                }
            }
        }

        // Libretro thumbnail standard invalid character replacements:
        let clean_title = clean
            .replace(':', "_")
            .replace('/', "_")
            .replace('*', "_")
            .replace('?', "_")
            .replace('"', "_")
            .replace('<', "_")
            .replace('>', "_")
            .replace('|', "_");
        let clean_encoded = clean_title.replace(' ', "%20");
        
        let (repo_system, folder) = match sys_code.to_uppercase().as_str() {
            "MAME" => ("MAME", "Named_Snaps"),
            "C64" => ("Commodore_-_64", "Named_Boxarts"),
            "SNES" => ("Nintendo_-_Super_Nintendo_Entertainment_System", "Named_Boxarts"),
            "NES" => ("Nintendo_-_Nintendo_Entertainment_System", "Named_Boxarts"),
            "N64" => ("Nintendo_-_Nintendo_64", "Named_Boxarts"),
            "GAMECUBE" => ("Nintendo_-_GameCube", "Named_Boxarts"),
            "DREAMCAST" => ("Sega_-_Dreamcast", "Named_Boxarts"),
            "NAOMI" => ("Sega_-_Naomi", "Named_Boxarts"),
            "ATOMISWAVE" => ("Atomiswave", "Named_Boxarts"),
            "PSX" => ("Sony_-_PlayStation", "Named_Boxarts"),
            "DOS" => ("DOS", "Named_Boxarts"),
            "SCUMMVM" => ("ScummVM", "Named_Boxarts"),
            _ => ("MAME", "Named_Snaps"),
        };

        format!(
            "https://raw.githubusercontent.com/libretro-thumbnails/{}/master/{}/{}.png",
            repo_system, folder, clean_encoded
        )
    }
}

struct PreloadedDirCache {
    cache_by_sys: HashMap<String, Vec<(String, String)>>,
}

impl PreloadedDirCache {
    fn build(emu_root: &Path) -> Self {
        let mut map = HashMap::new();
        let dirs = [
            ("C64", emu_root.join("C64").join("roms")),
            ("MAME", emu_root.join("mame")),
            ("NAOMI", emu_root.join("NAOMI - Flycast")),
            ("ATOMISWAVE", emu_root.join("NAOMI - Flycast")),
            ("HIKARU", emu_root.join("Hikaru - DEmul")),
            ("HYPSEUS", emu_root.join("Hypseus Singe")),
            ("DOS", emu_root.join("DOS").join("games")),
            ("SCUMMVM", emu_root.join("ScummVM").join("games")),
            ("SNES", emu_root.join("SNES")),
            ("INDIE", emu_root.join("LocoMalito & Indie")),
            ("RECOMP", emu_root.join("Recomp Library")),
            ("N64", emu_root.join("Nintendo64")),
            ("GAMECUBE", emu_root.join("GameCube")),
            ("DREAMCAST", emu_root.join("Dreamcast")),
            ("PSX", emu_root.join("PSX")),
            ("NES", emu_root.join("NES")),
        ];

        for (code, path) in dirs {
            if path.exists() {
                let mut list = Vec::new();
                if let Ok(entries) = fs::read_dir(&path) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        if p.is_file() && p.extension().map_or(false, |e| e == "lnk") {
                            let stem = p.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
                            list.push((stem, p.to_string_lossy().to_string()));
                        } else if p.is_dir() {
                            let sub_lnk = p.join("Gioca.lnk");
                            if sub_lnk.exists() {
                                let dir_name = p.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                                list.push((dir_name, sub_lnk.to_string_lossy().to_string()));
                            }
                        }
                    }
                }
                map.insert(code.to_string(), list);
            }
        }

        Self { cache_by_sys: map }
    }

    fn resolve(&self, emu_root: &Path, sys_code: &str, title: &str) -> String {
        let title_clean = title.to_lowercase().replace([':', '-', '\'', '\"'], " ");
        let words: Vec<&str> = title_clean.split_whitespace().collect();

        if let Some(list) = self.cache_by_sys.get(sys_code) {
            for (name, lnk_path) in list {
                if words.iter().all(|w| name.contains(w)) {
                    return lnk_path.clone();
                }
            }
        }

        // Fallback default system launcher
        match sys_code.to_uppercase().as_str() {
            "C64" => emu_root.join("Avvia Commodore 64 - VICE.lnk").to_string_lossy().to_string(),
            "MAME" => emu_root.join("Avvia MAME - Arcade.lnk").to_string_lossy().to_string(),
            "NAOMI" | "ATOMISWAVE" => emu_root.join("Avvia Sega NAOMI Arcade - Flycast.lnk").to_string_lossy().to_string(),
            "HIKARU" => emu_root.join("Hikaru - DEmul").join("demul.exe").to_string_lossy().to_string(),
            "HYPSEUS" => emu_root.join("Avvia LaserDisc - Hypseus Singe.lnk").to_string_lossy().to_string(),
            "DOS" => emu_root.join("Avvia DOS - DOSBox.lnk").to_string_lossy().to_string(),
            "SCUMMVM" => emu_root.join("Avvia ScummVM - Avventure Grafiche.lnk").to_string_lossy().to_string(),
            "SNES" => emu_root.join("Avvia Super Nintendo - Snes9x.lnk").to_string_lossy().to_string(),
            "INDIE" => emu_root.join("LocoMalito & Indie").to_string_lossy().to_string(),
            "RECOMP" => emu_root.join("Recomp Library").to_string_lossy().to_string(),
            "N64" => emu_root.join("Avvia Nintendo 64.lnk").to_string_lossy().to_string(),
            "GAMECUBE" => emu_root.join("Avvia GameCube - Dolphin.lnk").to_string_lossy().to_string(),
            "DREAMCAST" => emu_root.join("Avvia Dreamcast - Flycast.lnk").to_string_lossy().to_string(),
            "PSX" => emu_root.join("Avvia PlayStation - DuckStation.lnk").to_string_lossy().to_string(),
            "NES" => emu_root.join("Avvia NES - Mesen.lnk").to_string_lossy().to_string(),
            _ => emu_root.to_string_lossy().to_string(),
        }
    }
}

fn html_decode(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

fn clean_controls_text(raw: &str) -> String {
    let without_tags = Regex::new(r"<[^>]+>").unwrap().replace_all(raw, " ").to_string();
    let decoded = html_decode(&without_tags);
    decoded.replace("🎮", "").replace("&#127918;", "").split_whitespace().collect::<Vec<_>>().join(" ")
}
