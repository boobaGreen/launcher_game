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

        // Pre-carica mappatura MAME: titolo -> nome file rom (da Catalogo Giochi MAME Arcade.html)
        let mame_catalog_path = emu_root.join("mame").join("Catalogo Giochi MAME Arcade.html");
        let mut mame_rom_map = HashMap::new();
        if let Ok(mame_html) = fs::read_to_string(&mame_catalog_path) {
            if let Ok(re_mame) = Regex::new(r"<code>([^<]+)\.zip</code></td><td><span class='title-text'>([^<]+)</span>") {
                for cap in re_mame.captures_iter(&mame_html) {
                    let rom = cap.get(1).map_or("", |m| m.as_str()).trim();
                    let t = html_decode(cap.get(2).map_or("", |m| m.as_str()).trim()).to_lowercase();
                    mame_rom_map.insert(t, rom.to_string());
                }
            }
        }

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

            let launch_target = if sys_code.eq_ignore_ascii_case("MAME") {
                let t_low = title.to_lowercase();
                if let Some(rom) = mame_rom_map.get(&t_low) {
                    format!("MAME:{}", rom)
                } else {
                    dir_cache.resolve(emu_root, sys_code, &title)
                }
            } else {
                dir_cache.resolve(emu_root, sys_code, &title)
            };
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
        let t_lower = title.to_lowercase();

        // 1. Mappe dedicate per INDIE popolari (Steam vertical capsule ad altissima risoluzione 600x900)
        if sys_code.eq_ignore_ascii_case("INDIE") {
            let steam_app = if t_lower.contains("hollow knight") { Some("367520") }
                else if t_lower.contains("celeste") { Some("504230") }
                else if t_lower.contains("hotline miami") { Some("219150") }
                else if t_lower.contains("braid") { Some("26800") }
                else if t_lower.contains("broforce") { Some("274190") }
                else if t_lower.contains("fez") { Some("224760") }
                else if t_lower.contains("limbo") { Some("48000") }
                else if t_lower.contains("super meat boy") { Some("40800") }
                else if t_lower.contains("the messenger") { Some("764790") }
                else if t_lower.contains("vvvvvv") { Some("70300") }
                else if t_lower.contains("downwell") { Some("360740") }
                else if t_lower.contains("katana zero") { Some("460950") }
                else if t_lower.contains("blazing chrome") { Some("609110") }
                else if t_lower.contains("bloodstained") { Some("838310") }
                else if t_lower.contains("huntdown") { Some("598550") }
                else if t_lower.contains("ufo 50") { Some("1147860") }
                else if t_lower.contains("cave story") { Some("200900") }
                else if t_lower.contains("spelunky") { Some("239350") }
                else if t_lower.contains("maldita castilla") || t_lower.contains("cursed castilla") { Some("534290") }
                else { None };

            if let Some(app_id) = steam_app {
                return format!("https://steamcdn-a.akamaihd.net/steam/apps/{}/library_600x900.jpg", app_id);
            }

            if t_lower.contains("am2r") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Game_Boy/master/Named_Boxarts/Metroid%20II%20-%20Return%20of%20Samus%20(USA,%20Europe).png".to_string();
            }
            if t_lower.contains("streets of rage") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Mega_Drive_-_Genesis/master/Named_Boxarts/Streets%20of%20Rage%202%20(USA).png".to_string();
            }
        }

        // 2. Mappe dedicate per RECOMP PC Port (Cover originali fedeli console/arcade)
        if sys_code.eq_ignore_ascii_case("RECOMP") {
            if t_lower.contains("ocarina of time") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20Ocarina%20of%20Time%20(USA).png".to_string();
            }
            if t_lower.contains("majora") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20Majora's%20Mask%20(USA).png".to_string();
            }
            if t_lower.contains("mario 64") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Super%20Mario%2064%20(USA).png".to_string();
            }
            if t_lower.contains("wave race") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Wave%20Race%2064%20(USA).png".to_string();
            }
            if t_lower.contains("diddy kong racing") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Diddy%20Kong%20Racing%20(USA).png".to_string();
            }
            if t_lower.contains("super metroid") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Metroid%20(USA).png".to_string();
            }
            if t_lower.contains("mario world") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Mario%20World%20(USA).png".to_string();
            }
            if t_lower.contains("mario rpg") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Mario%20RPG%20-%20Legend%20of%20the%20Seven%20Stars%20(USA).png".to_string();
            }
            if t_lower.contains("mario all-stars") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Mario%20All-Stars%20(USA).png".to_string();
            }
            if t_lower.contains("earthbound") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/EarthBound%20(USA).png".to_string();
            }
            if t_lower.contains("donkey kong country 3") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Donkey%20Kong%20Country%203%20-%20Dixie%20Kong's%20Double%20Trouble!%20(USA).png".to_string();
            }
            if t_lower.contains("f-zero") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/F-Zero%20(USA).png".to_string();
            }
            if t_lower.contains("chrono cross") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Chrono%20Cross%20(USA).png".to_string();
            }
            if t_lower.contains("symphony of the night") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Castlevania%20-%20Symphony%20of%20the%20Night%20(USA).png".to_string();
            }
            if t_lower.contains("vib-ribbon") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Vib-Ribbon%20(Europe).png".to_string();
            }
            if t_lower.contains("outrun") || t_lower.contains("out run") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Out%20Run.png".to_string();
            }
            if t_lower.contains("daytona usa") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Dreamcast/master/Named_Boxarts/Daytona%20USA%20(USA).png".to_string();
            }
            if t_lower.contains("soulcalibur") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Dreamcast/master/Named_Boxarts/Soulcalibur%20(USA).png".to_string();
            }
            if t_lower.contains("metal slug") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Metal%20Slug%20-%20Super%20Vehicle-001.png".to_string();
            }
            if t_lower.contains("turtles in time") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Teenage%20Mutant%20Ninja%20Turtles%20-%20Turtles%20in%20Time.png".to_string();
            }
            if t_lower.contains("initial d") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Naomi/master/Named_Boxarts/Initial%20D%20Arcade%20Stage%20Ver.%203.png".to_string();
            }
        }

        // 3. Risoluzione generale per tutte le altre piattaforme (con suffissi regionali ottimali)
        let mut clean = title.trim();
        if let Some(open_p) = clean.rfind('(') {
            if let Some(close_p) = clean[open_p..].find(')') {
                let inside = clean[open_p + 1..open_p + close_p].trim();
                if inside.len() <= 12 && !inside.contains("19") && !inside.contains("20") {
                    clean = clean[..open_p].trim();
                }
            }
        }

        let (repo_system, folder, suffix) = match sys_code.to_uppercase().as_str() {
            "MAME" => ("MAME", "Named_Snaps", ""),
            "C64" => ("Commodore_-_64", "Named_Boxarts", " (Europe)"),
            "SNES" => ("Nintendo_-_Super_Nintendo_Entertainment_System", "Named_Boxarts", " (USA)"),
            "NES" => ("Nintendo_-_Nintendo_Entertainment_System", "Named_Boxarts", " (USA)"),
            "N64" => ("Nintendo_-_Nintendo_64", "Named_Boxarts", " (USA)"),
            "GAMECUBE" => ("Nintendo_-_GameCube", "Named_Boxarts", " (USA)"),
            "DREAMCAST" => ("Sega_-_Dreamcast", "Named_Boxarts", " (USA)"),
            "NAOMI" => ("Sega_-_Naomi", "Named_Boxarts", ""),
            "ATOMISWAVE" => ("Atomiswave", "Named_Boxarts", ""),
            "PSX" => ("Sony_-_PlayStation", "Named_Boxarts", " (USA)"),
            "DOS" => ("DOS", "Named_Boxarts", ""),
            "SCUMMVM" => ("ScummVM", "Named_Boxarts", ""),
            "HIKARU" => ("MAME", "Named_Snaps", ""),
            "HYPSEUS" => ("MAME", "Named_Snaps", ""),
            _ => ("MAME", "Named_Snaps", ""),
        };

        let full_name = format!("{}{}", clean, suffix);
        let clean_title = full_name
            .replace(':', "_")
            .replace('/', "_")
            .replace('*', "_")
            .replace('?', "_")
            .replace('"', "_")
            .replace('<', "_")
            .replace('>', "_")
            .replace('|', "_");
        let clean_encoded = clean_title.replace(' ', "%20");

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
