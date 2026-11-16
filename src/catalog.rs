use crate::models::Game;
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct CatalogLoader;

impl CatalogLoader {
    /// Loads all games from the Master HTML Catalog and resolves disk launch targets.
    pub fn load_from_master_catalog(emu_root: &Path) -> Result<Vec<Game>, String> {
        let cat_path = emu_root.join("CATALOGO GENERALE RETROGAMING.html");
        if !cat_path.exists() {
            return Err(format!("File catalogo non trovato in: {:?}", cat_path));
        }

        let content = fs::read_to_string(&cat_path)
            .map_err(|e| format!("Impossibile leggere il catalogo master: {}", e))?;

        // Extract tbody
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

            let launch_target = Self::resolve_launch_target(emu_root, sys_code, &title);
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

    fn resolve_launch_target(emu_root: &Path, sys_code: &str, title: &str) -> String {
        // Find existing shortcut or launcher on disk
        match sys_code.to_uppercase().as_str() {
            "C64" => {
                let c64_dir = emu_root.join("C64").join("roms");
                find_matching_lnk(&c64_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia Commodore 64 - VICE.lnk").to_string_lossy().to_string())
            }
            "MAME" => {
                // If specific shortcut exists in mame/
                let mame_dir = emu_root.join("mame");
                find_matching_lnk(&mame_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia MAME - Arcade.lnk").to_string_lossy().to_string())
            }
            "NAOMI" | "ATOMISWAVE" => {
                let flycast_dir = emu_root.join("NAOMI - Flycast");
                find_matching_lnk(&flycast_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia Sega NAOMI Arcade - Flycast.lnk").to_string_lossy().to_string())
            }
            "HIKARU" => {
                let hikaru_dir = emu_root.join("Hikaru - DEmul");
                find_matching_lnk(&hikaru_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia Sega Hikaru - DEmul.lnk").to_string_lossy().to_string())
            }
            "HYPSEUS" => {
                let hyp_dir = emu_root.join("Hypseus Singe");
                find_matching_lnk(&hyp_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia LaserDisc - Hypseus Singe.lnk").to_string_lossy().to_string())
            }
            "DOS" => {
                let dos_dir = emu_root.join("DOS").join("games");
                find_matching_lnk(&dos_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia DOSBox Staging.lnk").to_string_lossy().to_string())
            }
            "SCUMMVM" => {
                let scumm_dir = emu_root.join("ScummVM").join("games");
                find_matching_lnk(&scumm_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia ScummVM.lnk").to_string_lossy().to_string())
            }
            "SNES" => {
                let snes_dir = emu_root.join("SNES");
                find_matching_lnk(&snes_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia Super Nintendo - Snes9x.lnk").to_string_lossy().to_string())
            }
            "INDIE" => {
                let indie_dir = emu_root.join("LocoMalito & Indie");
                find_matching_lnk(&indie_dir, title)
                    .unwrap_or_else(|| indie_dir.to_string_lossy().to_string())
            }
            "RECOMP" => {
                let recomp_dir = emu_root.join("Recomp Library");
                find_matching_lnk(&recomp_dir, title)
                    .unwrap_or_else(|| recomp_dir.to_string_lossy().to_string())
            }
            "N64" => {
                let n64_dir = emu_root.join("Nintendo64");
                find_matching_lnk(&n64_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia Nintendo 64.lnk").to_string_lossy().to_string())
            }
            "GAMECUBE" => {
                let gc_dir = emu_root.join("GameCube");
                find_matching_lnk(&gc_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia GameCube - Dolphin.lnk").to_string_lossy().to_string())
            }
            "DREAMCAST" => {
                let dc_dir = emu_root.join("Dreamcast");
                find_matching_lnk(&dc_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia Dreamcast - Flycast.lnk").to_string_lossy().to_string())
            }
            "PSX" => {
                let psx_dir = emu_root.join("PSX");
                find_matching_lnk(&psx_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia PlayStation - DuckStation.lnk").to_string_lossy().to_string())
            }
            "NES" => {
                let nes_dir = emu_root.join("NES");
                find_matching_lnk(&nes_dir, title)
                    .unwrap_or_else(|| emu_root.join("Avvia NES - Mesen.lnk").to_string_lossy().to_string())
            }
            _ => emu_root.to_string_lossy().to_string(),
        }
    }

    fn build_cover_url(sys_code: &str, title: &str) -> String {
        let clean_title = title.replace('/', "-").replace(':', "");
        let clean_encoded = clean_title.replace(' ', "%20");
        
        let repo_system = match sys_code.to_uppercase().as_str() {
            "MAME" => "MAME",
            "C64" => "Commodore_-_64",
            "SNES" => "Nintendo_-_Super_Nintendo_Entertainment_System",
            "NES" => "Nintendo_-_Nintendo_Entertainment_System",
            "N64" => "Nintendo_-_Nintendo_64",
            "GAMECUBE" => "Nintendo_-_GameCube",
            "DREAMCAST" => "Sega_-_Dreamcast",
            "NAOMI" => "Sega_-_NAOMI",
            "ATOMISWAVE" => "Sammy_-_Atomiswave",
            "PSX" => "Sony_-_PlayStation",
            "DOS" => "DOS",
            _ => "MAME",
        };

        format!(
            "https://raw.githubusercontent.com/libretro-thumbnails/{}/master/Named_Boxarts/{}.png",
            repo_system, clean_encoded
        )
    }
}

fn find_matching_lnk(search_dir: &Path, title: &str) -> Option<String> {
    if !search_dir.exists() {
        return None;
    }

    let title_clean = title.to_lowercase().replace([':', '-', '\'', '\"'], " ");
    let title_words: Vec<&str> = title_clean.split_whitespace().collect();

    // 1. Check direct shortcuts in directory
    if let Ok(entries) = fs::read_dir(search_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "lnk") {
                let name = path.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
                if title_words.iter().all(|w| name.contains(w)) {
                    return Some(path.to_string_lossy().to_string());
                }
            } else if path.is_dir() {
                // Check subfolder
                let sub_lnk = path.join("Gioca.lnk");
                if sub_lnk.exists() {
                    let dir_name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                    if title_words.iter().all(|w| dir_name.contains(w)) {
                        return Some(sub_lnk.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    None
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
