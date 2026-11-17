use crate::models::Game;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Default, Clone)]
struct MameMeta {
    pub year: u16,
    pub developer: String,
    pub genre: String,
}

#[derive(Deserialize, Default, Clone)]
struct DosMeta {
    pub developer: String,
    pub genre: String,
}

static MAME_METADATA_JSON: &str = include_str!("mame_metadata.json");
static DOS_METADATA_JSON: &str = include_str!("dos_metadata.json");

fn is_mame_bios_or_hardware(title: &str, rom: &str) -> bool {
    let t = title.to_lowercase();
    let r = rom.to_lowercase();

    let hardware_roms = [
        "atarisy1", "bsmt2000", "sente6vb", "midcsd", "iteagle", "hng64", "kviper",
        "midssio", "m68705p3", "m68705p5", "naomi", "naomi2", "namco50", "namco51",
        "namco52", "namco53", "namco54", "namcoc69", "pgm", "qsound_hle", "virge_pci",
        "jvs13551", "segabill", "segadimm", "skns", "tms320c31", "konamigx", "cchip",
        "neogeo", "cvs", "decocass", "coh1000t", "coh1000w", "coh1002e", "coh1002m",
        "taitofx1", "stvbios", "crysbios", "sys573", "aleck64", "macsbios",
        "votrsc01", "votrsc01a", "ym2608", "ym2413", "ym2151", "ym2203", "ym2610",
        "ym2612", "ym3812", "ymf278b", "ay8910", "sn76496", "dac"
    ];

    if hardware_roms.contains(&r.as_str()) {
        return true;
    }

    if t.ends_with(" bios") 
        || t.contains(" bios ") 
        || t.contains("system bios")
        || t.contains("sound board") 
        || t.contains("audio board") 
        || t.contains("i/o board") 
        || t.contains("dimm board") 
        || t.contains("billboard") 
        || t.starts_with("motorola mc") 
        || t.starts_with("namco 5") 
        || t.starts_with("namco c") 
        || t.starts_with("texas instruments") 
        || t.starts_with("s3 86c") 
        || t.starts_with("bsmt") 
        || t.starts_with("qsound") 
        || t.starts_with("system gx") 
        || t.contains("(c-chip)") 
        || t.starts_with("votrax") 
        || t.starts_with("ym2") 
        || t.starts_with("ym3") 
        || t.starts_with("ymf") 
        || t.starts_with("yamaha ") 
    {
        return true;
    }

    false
}

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

        // Pre-carica metadati MAME (produttori autentici, anni e generi)
        let json_clean = MAME_METADATA_JSON.trim_start_matches('\u{feff}');
        let mame_meta_map: HashMap<String, MameMeta> =
            serde_json::from_str(json_clean).unwrap_or_default();

        // Pre-carica metadati MS-DOS (produttori autentici e generi)
        let dos_clean = DOS_METADATA_JSON.trim_start_matches('\u{feff}');
        let dos_meta_map: HashMap<String, DosMeta> =
            serde_json::from_str(dos_clean).unwrap_or_default();

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
            let mut year: u16 = cap.get(5).map_or("1990", |m| m.as_str()).trim().parse().unwrap_or(1990);
            let mut genre = html_decode(cap.get(6).map_or("", |m| m.as_str()).trim());
            let mut dev = html_decode(cap.get(7).map_or("", |m| m.as_str()).trim());
            let raw_controls = cap.get(8).map_or("", |m| m.as_str());
            let controls = clean_controls_text(raw_controls);

            let (launch_target, cover_url) = if sys_code.eq_ignore_ascii_case("MAME") {
                let t_low = title.to_lowercase();
                if let Some(rom) = mame_rom_map.get(&t_low) {
                    if is_mame_bios_or_hardware(&title, rom) {
                        continue;
                    }
                    if let Some(meta) = mame_meta_map.get(rom) {
                        dev = meta.developer.clone();
                        genre = meta.genre.clone();
                        if meta.year > 0 {
                            year = meta.year;
                        }
                    }
                    (format!("MAME:{}", rom), format!("/api/cover/mame/{}", rom))
                } else {
                    if is_mame_bios_or_hardware(&title, "") {
                        continue;
                    }
                    (dir_cache.resolve(emu_root, sys_code, &title), Self::build_cover_url(sys_code, &title))
                }
            } else if sys_code.eq_ignore_ascii_case("NAOMI") || sys_code.eq_ignore_ascii_case("ATOMISWAVE") {
                // Estrai codice ROM tra parentesi es. "( ikaruga )" -> "ikaruga"
                let rom_code = if let Some(open) = title.rfind('(') {
                    if let Some(close) = title[open..].find(')') {
                        title[open + 1..open + close].trim()
                    } else { "" }
                } else { "" };
                let c_url = if !rom_code.is_empty() {
                    format!("/api/cover/naomi/{}", rom_code)
                } else {
                    Self::build_cover_url(sys_code, &title)
                };
                (dir_cache.resolve(emu_root, sys_code, &title), c_url)
            } else {
                if sys_code.eq_ignore_ascii_case("DOS") {
                    if let Some(meta) = dos_meta_map.get(&title) {
                        dev = meta.developer.clone();
                        genre = meta.genre.clone();
                    }
                }
                (dir_cache.resolve(emu_root, sys_code, &title), Self::build_cover_url(sys_code, &title))
            };

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
                else if t_lower.contains("la-mulana") { Some("230700") }
                else if t_lower.contains("maldita castilla") || t_lower.contains("cursed castilla") { Some("534290") }
                else if t_lower.contains("hydorah") { Some("628800") }
                else { None };

            if let Some(app_id) = steam_app {
                return format!("https://steamcdn-a.akamaihd.net/steam/apps/{}/library_600x900.jpg", app_id);
            }

            if t_lower.contains("the curse of issyos") || t_lower.contains("curse of issyos") {
                return "/api/cover/file/covers/the-curse-of-issyos.jpg".to_string();
            }
            if t_lower.contains("darkula") {
                return "/api/cover/file/covers/darkula.jpg".to_string();
            }
            if t_lower.contains("l'abbaye") || t_lower.contains("abbaye des morts") {
                return "/api/cover/file/covers/l-abbaye-des-morts.jpg".to_string();
            }
            if t_lower.contains("8bit killer") {
                return "/api/cover/file/covers/8bit-killer.jpg".to_string();
            }
            if t_lower.contains("i, robot") {
                return "/api/cover/file/covers/i-robot-remastered.jpg".to_string();
            }
            if t_lower.contains("am2r") {
                return "/api/cover/file/covers/am2r.jpg".to_string();
            }
            if t_lower.contains("streets of rage") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Mega_Drive_-_Genesis/master/Named_Boxarts/Streets%20of%20Rage%202%20(USA).png".to_string();
            }
            if t_lower.contains("hero core") {
                return "/api/cover/file/covers/hero_core.png".to_string();
            }
            if t_lower.contains("iji") {
                return "/api/cover/file/covers/iji.jpg".to_string();
            }
            if t_lower.contains("an untitled story") || t_lower.contains("untitled story") {
                return "https://static.wikitide.net/archipelagowiki/6/65/An_Untitled_Story_Cover.png".to_string();
            }
            if t_lower.contains("knytt stories") || t_lower.contains("knytt") {
                return "https://images.igdb.com/igdb/image/upload/t_cover_big/co28f1.jpg".to_string();
            }
            if t_lower.contains("within a deep forest") {
                return "https://images.igdb.com/igdb/image/upload/t_cover_big/co61kx.jpg".to_string();
            }
        }

        // 2. Mappe dedicate per Nintendo 64
        if sys_code.eq_ignore_ascii_case("N64") {
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
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Snaps/Wave%20Race%2064%20-%20Kawasaki%20Jet%20Ski%20(USA).png".to_string();
            }
            if t_lower.contains("diddy kong") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Diddy%20Kong%20Racing%20(USA)%20(En,Fr).png".to_string();
            }
        }

        // 3. Mappe dedicate per Nintendo NES
        if sys_code.eq_ignore_ascii_case("NES") {
            if t_lower.contains("goonies ii") || t_lower.contains("goonies 2") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_Entertainment_System/master/Named_Boxarts/Goonies%20II,%20The%20(USA).png".to_string();
            }
            if t_lower.contains("goonies") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_Entertainment_System/master/Named_Boxarts/Goonies%20(Japan).png".to_string();
            }
            if t_lower.contains("gradius") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_Entertainment_System/master/Named_Boxarts/Gradius%20(USA).png".to_string();
            }
            if t_lower.contains("life force") || t_lower.contains("salamander") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_Entertainment_System/master/Named_Boxarts/Life%20Force%20(USA).png".to_string();
            }
            if t_lower.contains("lupin") {
                return "/api/cover/file/NES/roms/Lupin%20Sansei%20-%20Pandora%20no%20Isan%20(1987)/cover.png".to_string();
            }
        }

        // 4. Mappe dedicate per Sega Hikaru Arcade
        if sys_code.eq_ignore_ascii_case("HIKARU") {
            if t_lower.contains("racer") || t_lower.contains("swracer") {
                return "/api/cover/mame/swracer".to_string();
            }
            if t_lower.contains("planet harriers") || t_lower.contains("pharrier") {
                return "/api/cover/mame/pharrier".to_string();
            }
            if t_lower.contains("brave") || t_lower.contains("braveff") {
                return "/api/cover/mame/braveff".to_string();
            }
            if t_lower.contains("nascar") || t_lower.contains("sgnascar") {
                return "/api/cover/mame/sgnascar".to_string();
            }
        }

        // 5. Mappe dedicate per PlayStation
        if sys_code.eq_ignore_ascii_case("PSX") {
            if t_lower.contains("vib-ribbon") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Vib-Ribbon%20(Europe)%20(En,Fr,De,Es,It).png".to_string();
            }
            if t_lower.contains("lupin") {
                return "/api/cover/file/PSX/DuckStation/covers/Lupin%203sei%20(Japan).png".to_string();
            }
        }

        // 6. Mappe dedicate per Sega Dreamcast
        if sys_code.eq_ignore_ascii_case("DREAMCAST") {
            if t_lower.contains("virtua tennis 2") || t_lower.contains("tennis 2k2") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Dreamcast/master/Named_Boxarts/Virtua%20Tennis%202%20-%20Sega%20Professional%20Tennis%20(Europe)%20(En,Fr,De,Es).png".to_string();
            }
        }

        // 7. Mappe dedicate per Super Nintendo (SNES)
        if sys_code.eq_ignore_ascii_case("SNES") {
            if t_lower.contains("country 2") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Donkey%20Kong%20Country%202%20-%20Diddy's%20Kong%20Quest%20(USA)%20(En,Fr).png".to_string();
            }
            if t_lower.contains("country 3") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Donkey%20Kong%20Country%203%20-%20Dixie%20Kong's%20Double%20Trouble!%20(USA)%20(En,Fr).png".to_string();
            }
            if t_lower.contains("lupin") {
                return "/api/cover/file/SNES/cover_lupin.png".to_string();
            }
            if t_lower.contains("super drift out") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Drift%20Out%20-%20World%20Rally%20Championships%20(Japan)%20(En).png".to_string();
            }
            if t_lower.contains("mario rpg") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Mario%20RPG%20-%20Legend%20of%20the%20Seven%20Stars%20(USA).png".to_string();
            }
            if t_lower.contains("yoshi's island") || t_lower.contains("mario world 2") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Mario%20World%202%20-%20Yoshi's%20Island%20(USA).png".to_string();
            }
            if t_lower.contains("super metroid") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Metroid%20(Japan,%20USA)%20(En,Ja).png".to_string();
            }
            if t_lower.contains("link to the past") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20A%20Link%20to%20the%20Past%20(USA).png".to_string();
            }
        }

        // 8. Mappe dedicate per Nintendo GameCube
        if sys_code.eq_ignore_ascii_case("GAMECUBE") {
            if t_lower.contains("wind waker") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_GameCube/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20The%20Wind%20Waker%20(USA).png".to_string();
            }
            if t_lower.contains("twilight princess") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_GameCube/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20Twilight%20Princess%20(USA).png".to_string();
            }
            if t_lower.contains("metroid prime 2") || t_lower.contains("echoes") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_GameCube/master/Named_Boxarts/Metroid%20Prime%202%20-%20Echoes%20(USA).png".to_string();
            }
            if t_lower.contains("metroid prime") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_GameCube/master/Named_Boxarts/Metroid%20Prime%20(USA).png".to_string();
            }
            if t_lower.contains("f-zero") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_GameCube/master/Named_Boxarts/F-Zero%20GX%20(USA).png".to_string();
            }
        }

        // 9. Mappe dedicate per LaserDisc Arcade (Hypseus Singe)
        if sys_code.eq_ignore_ascii_case("HYPSEUS") {
            if t_lower.contains("dragon's lair ii") || t_lower.contains("dragons lair 2") || t_lower.contains("time warp") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_Amiga/master/Named_Boxarts/Dragon's%20Lair%20II%20-%20Time%20Warp.png".to_string();
            }
            if t_lower.contains("dragon's lair") || t_lower.contains("dragons lair") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Mega-CD_-_Sega_CD/master/Named_Boxarts/Dragon's%20Lair%20(USA).png".to_string();
            }
            if t_lower.contains("space ace") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Mega-CD_-_Sega_CD/master/Named_Boxarts/Space%20Ace%20(USA).png".to_string();
            }
            if t_lower.contains("time gal") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Mega-CD_-_Sega_CD/master/Named_Boxarts/Time%20Gal%20(USA).png".to_string();
            }
            if t_lower.contains("cliff hanger") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Cliff%20Hanger.png".to_string();
            }
        }

        // 10. Mappe dedicate per RECOMP PC Port (Cover originali fedeli console/arcade)
        if sys_code.eq_ignore_ascii_case("RECOMP") {
            if t_lower.contains("ocarina of time") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20Ocarina%20of%20Time%20(USA).png".to_string();
            }
            if t_lower.contains("majora") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Legend%20of%20Zelda,%20The%20-%20Majora's%20Mask%20(USA).png".to_string();
            }
            if t_lower.contains("infinite mario") || t_lower.contains("mario 64") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Super%20Mario%2064%20(USA).png".to_string();
            }
            if t_lower.contains("blue wake") || t_lower.contains("wave race") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Snaps/Wave%20Race%2064%20-%20Kawasaki%20Jet%20Ski%20(USA).png".to_string();
            }
            if t_lower.contains("diddy kong") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Nintendo_64/master/Named_Boxarts/Diddy%20Kong%20Racing%20(USA)%20(En,Fr).png".to_string();
            }
            if t_lower.contains("super metroid") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Super%20Metroid%20(Japan,%20USA)%20(En,Ja).png".to_string();
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
                return "https://raw.githubusercontent.com/libretro-thumbnails/Nintendo_-_Super_Nintendo_Entertainment_System/master/Named_Boxarts/Donkey%20Kong%20Country%203%20-%20Dixie%20Kong's%20Double%20Trouble!%20(USA)%20(En,Fr).png".to_string();
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
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Vib-Ribbon%20(Europe)%20(En,Fr,De,Es,It).png".to_string();
            }
            if t_lower.contains("ridge racer") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Ridge%20Racer%20(USA).png".to_string();
            }
            if t_lower.contains("silhouette mirage") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sony_-_PlayStation/master/Named_Boxarts/Silhouette%20Mirage%20(USA).png".to_string();
            }
            if t_lower.contains("outrunners") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/OutRunners.png".to_string();
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
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Naomi/main/Named_Boxarts/Initial%20D%20Arcade%20Stage%20Ver.%203%20(GDL-0023).png".to_string();
            }
            if t_lower.contains("dig dug") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Dig%20Dug.png".to_string();
            }
            if t_lower.contains("emergency call ambulance") {
                return "/api/cover/file/covers/emergency_call.png".to_string();
            }
            if t_lower.contains("forza horizon") {
                return "/api/cover/file/covers/forza_horizon.jpg".to_string();
            }
            if t_lower.contains("fighting layer") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Fighting%20Layer.png".to_string();
            }
            if t_lower.contains("in the hunt") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/In%20The%20Hunt.png".to_string();
            }
            if t_lower.contains("mortal kombat") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Mortal%20Kombat.png".to_string();
            }
            if t_lower.contains("nba jam") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/NBA%20Jam%20TE.png".to_string();
            }
            if t_lower.contains("neo drift out") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Neo%20Drift%20Out%20-%20New%20Technology.png".to_string();
            }
            if t_lower.contains("rolling thunder") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Rolling%20Thunder.png".to_string();
            }
            if t_lower.contains("segasonic") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/SegaSonic%20The%20Hedgehog.png".to_string();
            }
            if t_lower.contains("silent scope") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Silent%20Scope.png".to_string();
            }
            if t_lower.contains("spikeout") {
                return "/api/cover/file/covers/spikeout.png".to_string();
            }
            if t_lower.contains("super hang-on") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Super%20Hang-On.png".to_string();
            }
            if t_lower.contains("king of fighters '98") || t_lower.contains("kof '98") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/The%20King%20of%20Fighters%20'98%20-%20The%20Slugfest%20_%20King%20of%20Fighters%20'98%20-%20dream%20match%20never%20ends.png".to_string();
            }
            if t_lower.contains("super mario bros") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Vs.%20Super%20Mario%20Bros..png".to_string();
            }
            if t_lower.contains("virtua nba") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/Virtua%20NBA.png".to_string();
            }
            if t_lower.contains("virtua tennis") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Sega_-_Dreamcast/master/Named_Boxarts/Virtua%20Tennis%20(USA).png".to_string();
            }
            if t_lower.contains("world rally") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/MAME/master/Named_Snaps/World%20Rally.png".to_string();
            }
        }

        // 11. Risoluzione generale per tutte le altre piattaforme (con suffissi regionali ottimali)
        let mut clean = title.trim();
        // Rimuovi anno finale in parentesi es. (1991), (1995) o tag versione corti
        if let Some(open_p) = clean.rfind('(') {
            if let Some(close_p) = clean[open_p..].find(')') {
                let inside = clean[open_p + 1..open_p + close_p].trim();
                if inside.len() <= 12 {
                    clean = clean[..open_p].trim();
                }
            }
        }

        // Mappe dedicate per capolavori PC MS-DOS, ScummVM e Commodore 64
        if sys_code.eq_ignore_ascii_case("DOS") {
            if t_lower.contains("epic baseball") {
                return "/api/cover/file/covers/epic_baseball.jpg".to_string();
            }
            if t_lower == "civilization" { clean = "Sid Meier's Civilization"; }
            else if t_lower == "colonization" { clean = "Sid Meier's Colonization"; }
            else if t_lower == "railroad tycoon" { clean = "Sid Meier's Railroad Tycoon"; }
            else if t_lower.contains("dune ii") { clean = "Dune II_ The Building of a Dynasty"; }
            else if t_lower.contains("commander keen 1") { clean = "Commander Keen 1 - Marooned on Mars"; }
            else if t_lower.contains("commander keen 4") { clean = "Commander Keen 4 - Secret of the Oracle"; }
            else if t_lower.contains("comanche") { clean = "Comanche maximum overkill"; }
            else if t_lower.contains("desert strike") { clean = "Desert Strike_ Return to the Gulf"; }
            else if t_lower.contains("even more incredible machine") { clean = "Even More Incredible Machine, The"; }
            else if t_lower.contains("incredible machine 2") { clean = "Incredible Machine 2, The"; }
            else if t_lower.contains("incredible machine") { clean = "Incredible Machine, The"; }
            else if t_lower.contains("lands of lore") { clean = "Lands of Lore_ The Throne of Chaos"; }
            else if t_lower.contains("magic carpet 2") { clean = "Magic Carpet 2_ The Netherworlds"; }
            else if t_lower.contains("micro machines 2") { clean = "Micro Machines 2_ Turbo Tournament"; }
            else if t_lower.contains("patrician") { clean = "Patrician, The"; }
            else if t_lower.contains("prince of persia 2") { clean = "Prince of Persia 2 - The Shadow and The Flame"; }
            else if t_lower.contains("settlers ii") { clean = "Settlers II, The (Gold Edition)"; }
            else if t_lower.contains("simant") { clean = "SimAnt_ The Electronic Ant Colony"; }
            else if t_lower == "simcity" { clean = "SimCity"; }
            else if t_lower.contains("simearth") { clean = "SimEarth_ The Living Planet"; }
            else if t_lower.contains("speedball 2") { clean = "Speedball 2_ Brutal Deluxe"; }
            else if t_lower.contains("tie fighter") { clean = "Star Wars_ TIE Fighter (Collector's CD-ROM)"; }
            else if t_lower.contains("x-wing") { clean = "Star Wars_ X-Wing"; }
            else if t_lower.contains("strike commander") { clean = "Strike Commander (CD-ROM Edition)"; }
            else if t_lower == "stunts" { clean = "Stunts"; }
            else if t_lower.contains("test drive iii") { clean = "Test Drive III the passion"; }
            else if t_lower.contains("ufo") { clean = "UFO Enemy Unknown"; }
            else if t_lower.contains("ultima underworld ii") { clean = "Ultima Underworld 2 - Labyrinth of Worlds"; }
            else if t_lower.contains("ultima vii") { clean = "Ultima VII the black gate"; }
            else if t_lower.contains("warcraft") { clean = "WarCraft - Orcs and Humans"; }
            else if t_lower.contains("privateer") { clean = "Wing Commander Privateer"; }
            else if t_lower.contains("wing commander ii") { clean = "Wing Commander II_ Vengeance of the Kilrathi"; }
            else if t_lower.contains("formula one grand prix") { clean = "World Circuit"; }
            else if t_lower.contains("halloween harry") || t_lower.contains("alien carnage") { clean = "Alien Carnage"; }
            else if t_lower.contains("humans 3") { clean = "Humans 3_ Evolution_ Lost in Time"; }
            else if t_lower == "humans" || t_lower == "the humans" { clean = "Humans 1 and 2"; }
        } else if sys_code.eq_ignore_ascii_case("SCUMMVM") {
            if t_lower.contains("broken sword 2.5") { clean = "Broken Sword 2.5"; }
            else if t_lower.contains("broken sword ii") || t_lower.contains("smoking") { clean = "Broken Sword II_ The Smoking Mirror"; }
            else if t_lower.contains("broken sword") { clean = "Broken Sword_ The Shadow of the Templars"; }
            else if t_lower.contains("elvira ii") { clean = "Elvira II_ The Jaws of Cerberus"; }
            else if t_lower.contains("future wars") { clean = "Future Wars_ Time Travellers"; }
            else if t_lower.contains("gobliins 2") { clean = "Gobliins 2"; }
            else if t_lower.contains("goblins quest 3") { clean = "Goblins Quest 3"; }
            else if t_lower.contains("last crusade") { clean = "Indiana Jones and the Last Crusade"; }
            else if t_lower.contains("king's quest i") { clean = "King's Quest I (SCI remake)"; }
            else if t_lower.contains("king's quest v") { clean = "King's Quest V_ Absence Makes the Heart Go Yonder!"; }
            else if t_lower.contains("king's quest vi") { clean = "King's Quest VI_ Heir Today, Gone Tomorrow"; }
            else if t_lower.contains("king's quest vii") { clean = "King's Quest VII_ The Princeless Bride"; }
            else if t_lower.contains("colonel's bequest") { clean = "Laura Bow 1_ The Colonels Bequest"; }
            else if t_lower.contains("dagger of amon ra") { clean = "Laura Bow 2_ The Dagger of Amon Ra"; }
            else if t_lower.contains("discworld 2") || t_lower.contains("discworld ii") { clean = "Discworld II_ Missing Presumed...!_"; }
            else if t_lower.contains("discworld") { clean = "Discworld"; }
            else if t_lower.contains("gabriel knight") { clean = "Gabriel Knight_ Sins of the Fathers"; }
            else if t_lower.contains("larry 5") { clean = "Leisure Suit Larry 5"; }
            else if t_lower.contains("larry 7") { clean = "Leisure Suit Larry 7_ Love for Sail!"; }
            else if t_lower.contains("monkey island 2") { clean = "Monkey Island 2_ LeChuck's Revenge"; }
            else if t_lower.contains("secret of monkey island") { clean = "Secret of Monkey Island, The"; }
            else if t_lower.contains("curse of monkey island") { clean = "Curse of Monkey Island, The"; }
            else if t_lower.contains("escape from monkey island") { clean = "Escape from Monkey Island"; }
            else if t_lower.contains("myst") { clean = "Myst"; }
            else if t_lower.contains("operation stealth") { clean = "Operation Stealth"; }
            else if t_lower.contains("quest for glory") { clean = "Quest for Glory 1_ So You Want to Be a Hero"; }
            else if t_lower.contains("sam & max") || t_lower.contains("sam and max") { clean = "Sam _ Max Hit the Road"; }
            else if t_lower.contains("simon the sorcerer ii") { clean = "Simon the Sorcerer II_ The Lion, the Wizard and the Wardrobe"; }
            else if t_lower.contains("space quest iv") { clean = "Space Quest IV_ Roger Wilco and the Time Rippers"; }
            else if t_lower.contains("space quest v") { clean = "Space Quest V"; }
            else if t_lower.contains("kyrandia 2") { clean = "The Legend of Kyrandia_ Book Two_ The Hand of Fate"; }
            else if t_lower.contains("kyrandia 3") { clean = "The Legend of Kyrandia_ Book Three_ Malcolms Revenge"; }
            else if t_lower.contains("kyrandia") { clean = "The Legend of Kyrandia_ Book One"; }
            else if t_lower.contains("touch") { clean = "Touche_ The Adventures of the Fifth Musketeer"; }
            else if t_lower.contains("woodruff") { clean = "The Bizarre Adventures of Woodruff and the Schnibble"; }
            else if t_lower.contains("zak mckracken") { clean = "Zak McKracken and the Alien Mindbenders"; }
            else if t_lower.contains("thimbleweed park") { clean = "Thimbleweed Park"; }
        } else if sys_code.eq_ignore_ascii_case("C64") {
            let c64_dir = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\covers\c64");
            if c64_dir.exists() {
                let slug = title.to_lowercase()
                    .replace([':', '-', '\'', '\"', '!', '?', ',', '.', '(', ')'], " ");
                let clean_slug = slug.split_whitespace().collect::<Vec<_>>().join("_");
                for ext in &["png", "jpg", "gif", "webp"] {
                    let file_name = format!("{}.{}", clean_slug, ext);
                    if c64_dir.join(&file_name).exists() {
                        return format!("/api/cover/file/covers/c64/{}", file_name);
                    }
                }
            }
            if t_lower == "bal8tro" || t_lower.contains("bal8tro") {
                return "/api/cover/file/covers/c64/bal8tro.gif".to_string();
            }
            if t_lower == "hero" || t_lower == "h.e.r.o." {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Atari_-_2600/master/Named_Boxarts/H.E.R.O.%20(USA).png".to_string();
            }
            if t_lower.contains("advanced action movie simulator") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Advanced%20Action%20Movie%20Simulator%20(2001)(Cadaver)(PD).png".to_string();
            }
            if t_lower.contains("assembloids") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Assembloids%20(2012)(Bevilacqua%2C%20S.)%5Bcr%20LXT%5D%5Bt%20%2B2%5D.png".to_string();
            }
            if t_lower.contains("barnsley badger") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Barnsley%20Badger%20(2016)(Psytronik)%5Bcr%20ONS%5D.png".to_string();
            }
            if t_lower.contains("bobsled") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Bobsled%20(1983)(General%20Masters%20Corporation).png".to_string();
            }
            if t_lower.contains("bomberland") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Bomberland%20(2013)(RGCD)%5Bcr%20ONS%5D.png".to_string();
            }
            if t_lower.contains("burnin' rubber") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Burnin'%20Rubber%20(1983)(Colosoftware).png".to_string();
            }
            if t_lower.contains("caren and the tangled tentacles") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Caren%20and%20the%20Tangled%20Tentacles%20(2015)(Wendt%2C%20Martin)(de-en)(PD).png".to_string();
            }
            if t_lower.contains("cliff hanger") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Cliff%20Hanger%20(1984)(New%20Generation%20Software)%5Bcr%20Bert%5D.png".to_string();
            }
            if t_lower == "creatures" {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Creatures%20(Europe)%20(The%20Hits%202%20Compilation).png".to_string();
            }
            if t_lower.contains("creatures ii") || t_lower.contains("creatures 2") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Creatures%20II%20-%20Torture%20Trouble%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("first samurai") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/First%20Samurai%20(USA).png".to_string();
            }
            if t_lower.contains("flappy bird") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Flappy%20Bird%20(2013)(Kaminski%2C%20Mikolaj)(PD)%5Bcr%20LXT%5D.png".to_string();
            }
            if t_lower.contains("gridlock") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Gridlock%20(1986)(Electrical%20Software).png".to_string();
            }
            if t_lower.contains("gridrunner") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Gridrunner%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("guns 'n' ghosts") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Snaps/Guns%20'n'%20Ghosts%20(2013)(RGCD%20%26%20Psytronik)%5Bcr%20LXT%20-%20Hackersoft%5D.png".to_string();
            }
            if t_lower.contains("hawkeye") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Hawkeye%20(Europe)%20(Thalamus%20Hits%201986-1988%20Compilation).png".to_string();
            }
            if t_lower.contains("katakis") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Katakis%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("lords of chaos") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Lords%20of%20Chaos%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("matrix") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Matrix%20(Europe).png".to_string();
            }
            if t_lower.contains("montezuma's revenge") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Montezuma's%20Revenge%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("planet x2") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Planet%20X2%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("space rogue") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Space%20Rogue%20(USA%2C%20Europe).png".to_string();
            }
            if t_lower.contains("wizard of wor") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Commodore_-_64/master/Named_Boxarts/Wizard%20of%20Wor%20(Japan)%20(MAX).png".to_string();
            }
            if t_lower.contains("chopper command") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/Atari_-_2600/master/Named_Boxarts/Chopper%20Command%20(USA).png".to_string();
            }
            if t_lower.contains("red storm rising") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/DOS/master/Named_Boxarts/Red%20Storm%20Rising.png".to_string();
            }
            if t_lower.contains("parasol stars") {
                return "https://raw.githubusercontent.com/libretro-thumbnails/NEC_-_PC_Engine_-_TurboGrafx_16/master/Named_Boxarts/Parasol%20Stars%20-%20The%20Story%20of%20Bubble%20Bobble%20III%20(USA).png".to_string();
            }

            if t_lower == "1942" { clean = "1942 (Europe)"; }
            else if t_lower == "1943" || t_lower.contains("1943") { clean = "1943 - The Battle of Midway (Europe)"; }
            else if t_lower.contains("goonies") { clean = "Goonies, The (USA, Europe)"; }
            else if t_lower.contains("giana sisters") { clean = "Great Giana Sisters, The (Europe)"; }
            else if t_lower.contains("last ninja 3") { clean = "Last Ninja III, The - Real Hatred Is Timeless (Europe)"; }
            else if t_lower.contains("last ninja 2") { clean = "Last Ninja II - Back with a Vengeance (Europe)"; }
            else if t_lower.contains("last ninja") { clean = "Last Ninja, The (Europe)"; }
            else if t_lower.contains("turbo outrun") || t_lower.contains("turbo out run") { clean = "Turbo Out Run (Europe)"; }
            else if t_lower.contains("ultima iv") { clean = "Ultima IV - Quest of the Avatar (USA, Europe)"; }
            else if t_lower.contains("ultima v") { clean = "Ultima V - Warriors of Destiny (Europe)"; }
            else if t_lower.contains("ghosts 'n goblins") || t_lower.contains("ghosts n goblins") { clean = "Ghosts'n Goblins (Europe)"; }
            else if t_lower.contains("ghouls 'n ghosts") || t_lower.contains("ghouls n ghosts") { clean = "Ghouls 'n' Ghosts (Europe)"; }
            else if t_lower.contains("ik+") || t_lower.contains("international karate") { clean = "IK+ (Europe)"; }
            else if t_lower.contains("paperboy") { clean = "Paperboy (Europe)"; }
            else if t_lower.contains("pitstop ii") || t_lower.contains("pitstop 2") { clean = "Pitstop II (Europe)"; }
            else if t_lower.contains("pirates!") || t_lower.contains("pirates") { clean = "Pirates! (Europe)"; }
            else if t_lower.contains("rampage") { clean = "Rampage (Europe)"; }
            else if t_lower.contains("river raid") { clean = "River Raid (Europe)"; }
            else if t_lower.contains("robocop") { clean = "RoboCop (USA)"; }
            else if t_lower.contains("summer games ii") || t_lower.contains("summer games 2") { clean = "Summer Games II (Europe) (2 sides) (b-highscores)"; }
            else if t_lower.contains("summer games") { clean = "Summer Games (Europe)"; }
            else if t_lower.contains("winter games") { clean = "Winter Games (Europe)"; }
            else if t_lower.contains("world games") { clean = "World Games (Europe)"; }
            else if t_lower.contains("california games") { clean = "California Games (Europe)"; }
            else if t_lower.contains("zynaps") { clean = "Zynaps (Europe) (Hewson)"; }
            else if t_lower.contains("bruce lee") { clean = "Bruce Lee (Europe)"; }
            else if t_lower.contains("teenage mutant") || t_lower.contains("turtles") { clean = "Teenage Mutant Hero Turtles (Europe)"; }
            else if t_lower.contains("empire strikes back") { clean = "Star Wars - Empire Strikes Back, The (Europe)"; }
            else if t_lower.contains("star wars") { clean = "Star Wars (Europe)"; }
            else if t_lower.contains("arkanoid") { clean = "Arkanoid (Europe)"; }
            else if t_lower.contains("boulder dash") || t_lower.contains("boulderdash") { clean = "Boulder Dash (Europe)"; }
            else if t_lower.contains("bubble bobble") { clean = "Bubble Bobble (Europe)"; }
            else if t_lower.contains("commando") { clean = "Commando (Europe)"; }
            else if t_lower.contains("defender") { clean = "Defender (USA, Europe)"; }
            else if t_lower.contains("donkey kong") { clean = "Donkey Kong (Europe)"; }
            else if t_lower.contains("elite") { clean = "Elite (Europe)"; }
            else if t_lower.contains("gauntlet") { clean = "Gauntlet (Europe)"; }
            else if t_lower.contains("green beret") { clean = "Green Beret (Europe)"; }
            else if t_lower.contains("ikari warriors") { clean = "Ikari Warriors (Europe)"; }
            else if t_lower.contains("impossible mission") { clean = "Impossible Mission (Europe)"; }
            else if t_lower.contains("leaderboard") { clean = "Leaderboard (Europe)"; }
            else if t_lower.contains("maniac mansion") { clean = "Maniac Mansion (USA, Europe)"; }
            else if t_lower.contains("microprose soccer") { clean = "Microprose Soccer (Europe)"; }
            else if t_lower.contains("midnight resistance") { clean = "Midnight Resistance (Europe)"; }
            else if t_lower.contains("nebulus") { clean = "Nebulus (Europe)"; }
            else if t_lower.contains("pang") { clean = "Pang (Europe)"; }
            else if t_lower.contains("paradroid") { clean = "Paradroid (USA, Europe)"; }
            else if t_lower.contains("platoon") { clean = "Platoon (Europe)"; }
            else if t_lower.contains("prince of persia") { clean = "Prince of Persia (USA, Europe)"; }
            else if t_lower.contains("rainbow islands") { clean = "Rainbow Islands (Europe)"; }
            else if t_lower.contains("rick dangerous") { clean = "Rick Dangerous (Europe)"; }
            else if t_lower.contains("silkworm") { clean = "Silkworm (Europe)"; }
            else if t_lower.contains("space harrier") { clean = "Space Harrier (Europe)"; }
            else if t_lower.contains("spy hunter") { clean = "Spy Hunter (Europe)"; }
            else if t_lower.contains("spy vs spy") { clean = "Spy vs Spy (Europe)"; }
            else if t_lower.contains("strider") { clean = "Strider (Europe)"; }
            else if t_lower.contains("stunt car racer") { clean = "Stunt Car Racer (USA, Europe)"; }
            else if t_lower.contains("target renegade") { clean = "Target Renegade (Europe)"; }
            else if t_lower.contains("uridium") { clean = "Uridium (Europe)"; }
            else if t_lower.contains("wizball") { clean = "Wizball (Europe)"; }
            else if t_lower.contains("wonder boy") { clean = "Wonder Boy (USA, Europe)"; }
            else if t_lower.contains("zak mckracken") { clean = "Zak McKracken and the Alien Mindbenders (USA, Europe)"; }
            else if t_lower.contains("turrican ii") { clean = "Turrican II - The Final Fight (Europe)"; }
            else if t_lower.contains("turrican") { clean = "Turrican (Europe)"; }
            else if t_lower.contains("bomb jack") || t_lower == "bombjack" { clean = "Bomb Jack (Europe)"; }
            else if t_lower.contains("cybernoid ii") { clean = "Cybernoid II (Europe)"; }
            else if t_lower.contains("cybernoid") { clean = "Cybernoid - The Fighting Machine (Europe)"; }
            else if t_lower.contains("archon ii") { clean = "Archon II - Adept (USA, Europe)"; }
            else if t_lower.contains("archon") { clean = "Archon (USA, Europe)"; }
            else if t_lower.contains("ghostbusters") { clean = "Ghostbusters (Europe)"; }
            else if t_lower.contains("wasteland") { clean = "Wasteland (USA)"; }
            else if t_lower.contains("pig quest") { clean = "A Pig Quest"; }
            else if t_lower.contains("petscii robots") { clean = "Attack of the PETSCII Robots"; }
            else if t_lower.contains("briley witch") { clean = "Briley Witch Chronicles"; }
            else if t_lower.contains("galencia") { clean = "Galencia"; }
            else if t_lower.contains("hunter's moon") { clean = "Hunter's Moon"; }
            else if t_lower.contains("knight 'n' grail") { clean = "Knight 'n' Grail"; }
            else if t_lower.contains("mayhem in monsterland") { clean = "Mayhem in Monsterland"; }
            else if t_lower.contains("micro hexagon") { clean = "Micro Hexagon"; }
            else if t_lower.contains("nixy") { clean = "Nixy and the Seeds of Doom"; }
            else if t_lower.contains("organism") { clean = "Organism"; }
            else if t_lower.contains("sam's journey") { clean = "Sam's Journey"; }
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

        let full_name = if clean.ends_with(')') {
            clean.to_string()
        } else {
            format!("{}{}", clean, suffix)
        };
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

        let branch = if repo_system == "Sega_-_Naomi" { "main" } else { "master" };

        format!(
            "https://raw.githubusercontent.com/libretro-thumbnails/{}/{}/{}/{}.png",
            repo_system, branch, folder, clean_encoded
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
        let t_norm = title.to_lowercase();
        // Special overrides for games with localized / completely different directory names
        if sys_code.eq_ignore_ascii_case("SCUMMVM") {
            if t_norm.contains("broken sword") {
                if t_norm.contains("2.5") || t_norm.contains("ritorno") {
                    let p = emu_root.join("ScummVM").join("games").join("Broken Sword 2.5 - The Return of the Templars (2008)").join("Gioca.lnk");
                    if p.exists() { return p.to_string_lossy().to_string(); }
                } else if t_norm.contains(" ii") || t_norm.contains("profezia") || t_norm.contains("smoking") {
                    let p = emu_root.join("ScummVM").join("games").join("Broken Sword II - The Smoking Mirror (1997)").join("Gioca.lnk");
                    if p.exists() { return p.to_string_lossy().to_string(); }
                } else {
                    let p = emu_root.join("ScummVM").join("games").join("Broken Sword - The Shadow of the Templars (1996)").join("Gioca.lnk");
                    if p.exists() { return p.to_string_lossy().to_string(); }
                }
            } else if t_norm.contains("touche") || t_norm.contains("quinto moschettiere") {
                let p = emu_root.join("ScummVM").join("games").join("Touche - The Adventures of the Fifth Musketeer (1995)").join("Gioca.lnk");
                if p.exists() { return p.to_string_lossy().to_string(); }
            }
        }

        let title_tokens = normalize_and_tokenize(title);

        if let Some(list) = self.cache_by_sys.get(sys_code) {
            let mut best_match: Option<(i32, String)> = None;

            for (name, lnk_path) in list {
                let name_tokens = normalize_and_tokenize(name);
                if name_tokens.is_empty() || title_tokens.is_empty() {
                    continue;
                }

                // Isolate single-digit sequel numbers (e.g. "2", "3", "4", but ignoring years like "2000" or "99")
                let title_nums: Vec<&String> = title_tokens.iter().filter(|t| t.len() == 1 && t.chars().all(|c| c.is_ascii_digit())).collect();
                let name_nums: Vec<&String> = name_tokens.iter().filter(|t| t.len() == 1 && t.chars().all(|c| c.is_ascii_digit())).collect();

                // If candidate name has a sequel number that does NOT appear in title, reject it
                // (e.g. title "MediEvil" must not match "MediEvil 2", "Crash Bandicoot" must not match "Crash Bandicoot 2")
                if name_nums.iter().any(|num| !title_nums.contains(num)) {
                    continue;
                }

                // If title has a sequel number and candidate also has one, make sure they match
                if !title_nums.is_empty() && !name_nums.is_empty() {
                    if !title_nums.iter().any(|num| name_nums.contains(num)) {
                        continue;
                    }
                }

                let overlap = name_tokens.iter().filter(|w| title_tokens.contains(w)).count();
                if overlap == 0 {
                    continue;
                }

                let mut score = 0;
                // Exact token match
                if title_tokens == name_tokens {
                    score = 1000 + (overlap as i32) * 10;
                }
                // All name tokens are present in title (e.g. "donkey kong country 2" in "donkey kong country 2 diddys kong quest")
                else if name_tokens.iter().all(|w| title_tokens.contains(w)) {
                    let bonus = if !name_nums.is_empty() { 50 } else { 0 };
                    score = 600 + bonus + (overlap as i32) * 10;
                }
                // All title tokens are present in name
                else if title_tokens.iter().all(|w| name_tokens.contains(w)) {
                    score = 500 + (overlap as i32) * 10;
                }
                // Strong overlap: at least 2 tokens and covers most of name
                else if overlap >= 2 && overlap >= name_tokens.len().saturating_sub(1) {
                    score = 300 + (overlap as i32) * 10;
                }
                // Moderate overlap: at least 2 tokens and covers most of title
                else if overlap >= 2 && overlap >= title_tokens.len().saturating_sub(1) {
                    score = 200 + (overlap as i32) * 10;
                }
                // Single token match only if both are single-token
                else if overlap == 1 && title_tokens.len() == 1 && name_tokens.len() == 1 {
                    score = 100;
                }

                if score > 0 {
                    if let Some((best_score, _)) = &best_match {
                        if score > *best_score {
                            best_match = Some((score, lnk_path.clone()));
                        }
                    } else {
                        best_match = Some((score, lnk_path.clone()));
                    }
                }
            }

            if let Some((_, path)) = best_match {
                return path;
            }
        }

        // Fallback default system launcher
        match sys_code.to_uppercase().as_str() {
            "C64" => emu_root.join("Avvia Commodore 64 - VICE.lnk").to_string_lossy().to_string(),
            "MAME" => emu_root.join("Avvia MAME - Arcade.lnk").to_string_lossy().to_string(),
            "NAOMI" | "ATOMISWAVE" => emu_root.join("Avvia Sega NAOMI Arcade - Flycast.lnk").to_string_lossy().to_string(),
            "HIKARU" => emu_root.join("Hikaru - DEmul").join("demul.exe").to_string_lossy().to_string(),
            "HYPSEUS" => {
                let lnk = emu_root.join("Avvia LaserDisc - Hypseus Singe.lnk");
                if lnk.exists() {
                    lnk.to_string_lossy().to_string()
                } else {
                    emu_root.join("Hypseus Singe").join("hypseus.exe").to_string_lossy().to_string()
                }
            },
            "DOS" => emu_root.join("Avvia DOS - DOSBox.lnk").to_string_lossy().to_string(),
            "SCUMMVM" => emu_root.join("Avvia ScummVM - Avventure Grafiche.lnk").to_string_lossy().to_string(),
            "SNES" => emu_root.join("Avvia Super Nintendo - Snes9x.lnk").to_string_lossy().to_string(),
            "INDIE" => emu_root.join("LocoMalito & Indie").to_string_lossy().to_string(),
            "RECOMP" => emu_root.join("Recomp Library").to_string_lossy().to_string(),
            "N64" => {
                let n64_rmg = emu_root.join("Avvia Nintendo 64 - RMG.lnk");
                if n64_rmg.exists() {
                    n64_rmg.to_string_lossy().to_string()
                } else {
                    emu_root.join("Avvia Nintendo 64.lnk").to_string_lossy().to_string()
                }
            },
            "GAMECUBE" => emu_root.join("Avvia GameCube - Dolphin.lnk").to_string_lossy().to_string(),
            "DREAMCAST" => emu_root.join("Avvia Dreamcast - Flycast.lnk").to_string_lossy().to_string(),
            "PSX" => {
                let psx_lnk = emu_root.join("Avvia PSX - DuckStation.lnk");
                if psx_lnk.exists() {
                    psx_lnk.to_string_lossy().to_string()
                } else {
                    emu_root.join("Avvia PlayStation - DuckStation.lnk").to_string_lossy().to_string()
                }
            },
            "NES" => emu_root.join("Avvia NES - Mesen.lnk").to_string_lossy().to_string(),
            _ => emu_root.to_string_lossy().to_string(),
        }
    }
}

fn normalize_and_tokenize(text: &str) -> Vec<String> {
    let decoded = html_decode(text).to_lowercase();
    let unaccented: String = decoded.chars().map(|c| match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' => 'a',
        'è' | 'é' | 'ê' | 'ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' => 'i',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
        'ù' | 'ú' | 'û' | 'ü' => 'u',
        other => other,
    }).collect();

    // Replace "naomi 2" or "naomi2" so the arcade board hardware version 2 is not treated as a game sequel number
    let unaccented = unaccented.replace("naomi 2", "naomi").replace("naomi2", "naomi");

    // 1. Strip apostrophes so "dragon's" -> "dragons", "diddy's" -> "diddys"
    let without_apostrophes = unaccented.replace(['\'', '’', '`'], "");

    // 2. Replace all punctuation with spaces so tokens like "ii:", "(2)", "iv)" become surrounded by spaces
    let spaced_punct = without_apostrophes.replace(
        [':', '-', '\"', '!', '?', ',', '.', '(', ')', '[', ']', ';', '_', '/', '\\', '&', '+'],
        " "
    );

    // 3. Standalone roman numerals to arabic
    let padded = format!(" {} ", spaced_punct);
    let roman_map = [
        (" viii ", " 8 "),
        (" vii ", " 7 "),
        (" vi ", " 6 "),
        (" iv ", " 4 "),
        (" iii ", " 3 "),
        (" ii ", " 2 "),
        (" ix ", " 9 "),
        (" x ", " 10 "),
    ];
    let mut mapped = padded;
    for (rom, num) in roman_map {
        mapped = mapped.replace(rom, num);
    }

    let stopwords = [
        "gioca", "arcade", "naomi", "atomiswave", "the", "of", "and", "in", "a", "an",
        "il", "lo", "la", "i", "gli", "le", "del", "dei", "della", "delle", "de", "di", "da",
        "set", "ver", "version", "edition", "ultimate", "talkie", "fan", "dub", "ita", "eng",
        "japan", "usa", "europe", "world", "ju", "enhanced", "legend"
    ];

    mapped.split_whitespace()
        .filter(|w| {
            // skip 4-digit years like 1991, 1996, 2008 (except games literally titled 1942, 1943, 1941)
            if w.len() == 4 && w.chars().all(|c| c.is_ascii_digit()) && *w != "1942" && *w != "1943" && *w != "1941" {
                return false;
            }
            !stopwords.contains(w)
        })
        .map(|w| w.to_string())
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mame_meta_deser() {
        let json_clean = MAME_METADATA_JSON.trim_start_matches('\u{feff}');
        let map: HashMap<String, MameMeta> = match serde_json::from_str(json_clean) {
            Ok(m) => m,
            Err(e) => panic!("Deserialization failed: {}", e),
        };
        assert!(map.contains_key("crzrally"), "crzrally not found in mame_meta_map");
        let meta = map.get("crzrally").unwrap();
        assert_eq!(meta.developer, "Tecfri");
        assert_eq!(meta.year, 1985);
        assert_eq!(meta.genre, "Guida / Top-Down Rally");
    }

    #[test]
    fn test_dos_meta_deser() {
        let json_clean = DOS_METADATA_JSON.trim_start_matches('\u{feff}');
        let map: HashMap<String, DosMeta> = match serde_json::from_str(json_clean) {
            Ok(m) => m,
            Err(e) => panic!("DOS Deserialization failed: {}", e),
        };
        assert!(map.contains_key("Civilization"));
        let civ = map.get("Civilization").unwrap();
        assert_eq!(civ.developer, "MicroProse / Sid Meier");
        assert_eq!(civ.genre, "Strategia a Turni / 4X");

        assert!(map.contains_key("Dune II - The Building of a Dynasty"));
        let dune = map.get("Dune II - The Building of a Dynasty").unwrap();
        assert_eq!(dune.developer, "Westwood Studios / Virgin");
    }

    #[test]
    fn test_normalize_and_tokenize() {
        let t1 = normalize_and_tokenize("The Legend of Zelda: Ocarina of Time");
        assert_eq!(t1, vec!["zelda", "ocarina", "time"]);

        let t2 = normalize_and_tokenize("Border Down ( bdrdown )");
        assert!(t2.contains(&"border".to_string()));
        assert!(t2.contains(&"down".to_string()));

        let t3 = normalize_and_tokenize("Dragon's Lair II: Time Warp (Enhanced)");
        assert_eq!(t3, vec!["dragons", "lair", "2", "time", "warp"]);

        let t4 = normalize_and_tokenize("Donkey Kong Country 2: Diddy's Kong Quest");
        assert_eq!(t4, vec!["donkey", "kong", "country", "2", "diddys", "kong", "quest"]);
    }

    #[test]
    fn test_preloaded_resolve() {
        let mut map = HashMap::new();
        map.insert(
            "NAOMI".to_string(),
            vec![
                ("gioca - border down (arcade naomi)".to_string(), "C:\\emu\\NAOMI\\Gioca - Border Down (arcade Naomi).lnk".to_string()),
                ("gioca - castle of shikigami 2 (arcade naomi)".to_string(), "C:\\emu\\NAOMI\\Gioca - Castle of Shikigami 2 (arcade Naomi).lnk".to_string()),
            ]
        );
        map.insert(
            "N64".to_string(),
            vec![
                ("gioca - zelda - ocarina of time".to_string(), "C:\\emu\\N64\\Gioca - Zelda - Ocarina of Time.lnk".to_string()),
                ("gioca - zelda - majoras mask".to_string(), "C:\\emu\\N64\\Gioca - Zelda - Majoras Mask.lnk".to_string()),
            ]
        );
        map.insert(
            "SNES".to_string(),
            vec![
                ("gioca - donkey kong country".to_string(), "C:\\emu\\SNES\\Gioca - Donkey Kong Country.lnk".to_string()),
                ("gioca - donkey kong country 2".to_string(), "C:\\emu\\SNES\\Gioca - Donkey Kong Country 2.lnk".to_string()),
                ("gioca - donkey kong country 3".to_string(), "C:\\emu\\SNES\\Gioca - Donkey Kong Country 3.lnk".to_string()),
            ]
        );

        let cache = PreloadedDirCache { cache_by_sys: map };
        let emu_root = Path::new("C:\\emu");

        let p1 = cache.resolve(emu_root, "NAOMI", "Border Down ( bdrdown )");
        assert_eq!(p1, "C:\\emu\\NAOMI\\Gioca - Border Down (arcade Naomi).lnk");

        let p2 = cache.resolve(emu_root, "N64", "The Legend of Zelda: Ocarina of Time");
        assert_eq!(p2, "C:\\emu\\N64\\Gioca - Zelda - Ocarina of Time.lnk");

        let p3 = cache.resolve(emu_root, "SNES", "Donkey Kong Country 2: Diddy's Kong Quest");
        assert_eq!(p3, "C:\\emu\\SNES\\Gioca - Donkey Kong Country 2.lnk");

        let p4 = cache.resolve(emu_root, "SNES", "Donkey Kong Country");
        assert_eq!(p4, "C:\\emu\\SNES\\Gioca - Donkey Kong Country.lnk");
    }
}
