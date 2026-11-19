use crate::models::Game;

fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

pub struct MasterCatalog;

impl MasterCatalog {
    /// Returns the built-in Master Cloud Catalog of legendary titles across major retro systems.
    /// Games not present locally will have `is_installed: false` and are available on-demand.
    pub fn get_cloud_catalog(start_id: i64) -> Vec<Game> {
        let mut id = start_id;
        let mut games = Vec::new();

        macro_rules! add_game {
            ($sys:expr, $plat:expr, $title:expr, $year:expr, $genre:expr, $dev:expr, $subpath:expr) => {
                let clean_title = $title.replace(':', " -").replace('/', " -");
                let suffixed = if clean_title.ends_with(')') {
                    clean_title.clone()
                } else if $sys == "C64" || $sys == "AMIGA" {
                    format!("{} (Europe)", clean_title)
                } else if $sys == "DOS" || $sys == "SCUMMVM" || $sys == "LASERDISC" || $sys == "PINBALL" {
                    clean_title.clone()
                } else {
                    format!("{} (USA)", clean_title)
                };
                let cover = format!(
                    "https://raw.githubusercontent.com/libretro-thumbnails/{}/master/Named_Boxarts/{}.png",
                    $subpath,
                    url_encode(&suffixed)
                );
                games.push(Game {
                    id,
                    title: $title.to_string(),
                    platform: $plat.to_string(),
                    system_code: $sys.to_string(),
                    year: $year,
                    genre: $genre.to_string(),
                    developer: $dev.to_string(),
                    publisher: $dev.to_string(),
                    controls: String::new(),
                    launch_target: String::new(),
                    cover_url: Some(cover),
                    is_favorite: false,
                    play_count: 0,
                    last_played_at: None,
                    is_installed: false,
                });
                id += 1;
            };
        }

        // ==========================================
        // SUPER NINTENDO (SNES) - On-Demand Cloud
        // ==========================================
        let snes_sub = "Nintendo_-_Super_Nintendo_Entertainment_System";
        add_game!("SNES", "Super Nintendo", "Mega Man X", 1993, "Platform Action", "Capcom", snes_sub);
        add_game!("SNES", "Super Nintendo", "Street Fighter II Turbo", 1993, "Picchiaduro", "Capcom", snes_sub);
        add_game!("SNES", "Super Nintendo", "Killer Instinct", 1995, "Picchiaduro", "Rare", snes_sub);
        add_game!("SNES", "Super Nintendo", "Star Fox", 1993, "Rail Shooter", "Nintendo", snes_sub);
        add_game!("SNES", "Super Nintendo", "Super Punch-Out!!", 1994, "Sport / Boxe", "Nintendo", snes_sub);
        add_game!("SNES", "Super Nintendo", "Earthworm Jim", 1994, "Platform Action", "Shiny Entertainment", snes_sub);
        add_game!("SNES", "Super Nintendo", "Teenage Mutant Ninja Turtles IV - Turtles in Time", 1992, "Picchiaduro a Scorrimento", "Konami", snes_sub);
        add_game!("SNES", "Super Nintendo", "Contra III - The Alien Wars", 1992, "Run and Gun", "Konami", snes_sub);
        add_game!("SNES", "Super Nintendo", "Sunset Riders", 1993, "Run and Gun", "Konami", snes_sub);
        add_game!("SNES", "Super Nintendo", "Super Bomberman 2", 1994, "Azione / Party", "Hudson Soft", snes_sub);
        add_game!("SNES", "Super Nintendo", "Secret of Evermore", 1995, "Action RPG", "Square", snes_sub);
        add_game!("SNES", "Super Nintendo", "Illusion of Gaia", 1993, "Action RPG", "Quintet", snes_sub);
        add_game!("SNES", "Super Nintendo", "NBA Jam Tournament Edition", 1995, "Sport / Basket", "Midway", snes_sub);
        add_game!("SNES", "Super Nintendo", "Top Gear 2", 1993, "Corse", "Gremlin Graphics", snes_sub);
        add_game!("SNES", "Super Nintendo", "Super Castlevania IV", 1991, "Platform Action", "Konami", snes_sub);
        add_game!("SNES", "Super Nintendo", "ActRaiser", 1990, "Platform / Gestionale", "Quintet", snes_sub);

        // ==========================================
        // NINTENDO NES - On-Demand Cloud
        // ==========================================
        let nes_sub = "Nintendo_-_Nintendo_Entertainment_System";
        add_game!("NES", "Nintendo NES", "Super Mario Bros. 3", 1988, "Platform", "Nintendo", nes_sub);
        add_game!("NES", "Nintendo NES", "The Legend of Zelda", 1986, "Action Adventure", "Nintendo", nes_sub);
        add_game!("NES", "Nintendo NES", "Zelda II - The Adventure of Link", 1987, "Action RPG", "Nintendo", nes_sub);
        add_game!("NES", "Nintendo NES", "Castlevania III - Dracula's Curse", 1989, "Platform Action", "Konami", nes_sub);
        add_game!("NES", "Nintendo NES", "Mega Man 2", 1988, "Platform Action", "Capcom", nes_sub);
        add_game!("NES", "Nintendo NES", "Mega Man 3", 1990, "Platform Action", "Capcom", nes_sub);
        add_game!("NES", "Nintendo NES", "Contra", 1988, "Run and Gun", "Konami", nes_sub);
        add_game!("NES", "Nintendo NES", "Super C", 1990, "Run and Gun", "Konami", nes_sub);
        add_game!("NES", "Nintendo NES", "DuckTales", 1989, "Platform", "Capcom", nes_sub);
        add_game!("NES", "Nintendo NES", "Ninja Gaiden", 1988, "Platform Action", "Tecmo", nes_sub);
        add_game!("NES", "Nintendo NES", "Kirby's Adventure", 1993, "Platform", "HAL Laboratory", nes_sub);
        add_game!("NES", "Nintendo NES", "Mike Tyson's Punch-Out!!", 1987, "Sport / Boxe", "Nintendo", nes_sub);
        add_game!("NES", "Nintendo NES", "Metroid", 1986, "Action Adventure", "Nintendo", nes_sub);
        add_game!("NES", "Nintendo NES", "Bionic Commando", 1988, "Platform Action", "Capcom", nes_sub);
        add_game!("NES", "Nintendo NES", "Double Dragon II - The Revenge", 1989, "Picchiaduro", "Technos Japan", nes_sub);

        // ==========================================
        // SEGA MEGA DRIVE / GENESIS - On-Demand Cloud
        // ==========================================
        let md_sub = "Sega_-_Mega_Drive_-_Genesis";
        add_game!("MEGADRIVE", "Sega Mega Drive", "Sonic The Hedgehog 2", 1992, "Platform", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Sonic The Hedgehog 3", 1994, "Platform", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Sonic & Knuckles", 1994, "Platform", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Streets of Rage 2", 1992, "Picchiaduro a Scorrimento", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Streets of Rage 3", 1994, "Picchiaduro a Scorrimento", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Gunstar Heroes", 1993, "Run and Gun", "Treasure", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Shinobi III - Return of the Ninja Master", 1993, "Platform Action", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Castlevania - Bloodlines", 1994, "Platform Action", "Konami", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Contra - Hard Corps", 1994, "Run and Gun", "Konami", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Golden Axe II", 1991, "Picchiaduro a Scorrimento", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Disney's Aladdin", 1993, "Platform", "Virgin Games", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Phantasy Star IV", 1993, "RPG", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Comix Zone", 1995, "Picchiaduro", "Sega", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Earthworm Jim 2", 1995, "Platform Action", "Shiny Entertainment", md_sub);
        add_game!("MEGADRIVE", "Sega Mega Drive", "Ristar", 1995, "Platform", "Sega", md_sub);

        // ==========================================
        // GAME BOY ADVANCE (GBA) - On-Demand Cloud
        // ==========================================
        let gba_sub = "Nintendo_-_Game_Boy_Advance";
        add_game!("GBA", "Game Boy Advance", "Pokemon - Smeraldo", 2004, "RPG", "Game Freak", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Pokemon - Rosso Fuoco", 2004, "RPG", "Game Freak", gba_sub);
        add_game!("GBA", "Game Boy Advance", "The Legend of Zelda - The Minish Cap", 2004, "Action Adventure", "Capcom / Nintendo", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Metroid Fusion", 2002, "Action Adventure", "Nintendo", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Metroid - Zero Mission", 2004, "Action Adventure", "Nintendo", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Castlevania - Aria of Sorrow", 2003, "Action RPG", "Konami", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Golden Sun", 2001, "RPG", "Camelot", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Golden Sun - The Lost Age", 2002, "RPG", "Camelot", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Advance Wars 2 - Black Hole Rising", 2003, "Strategia a Turni", "Intelligent Systems", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Fire Emblem - The Blazing Blade", 2003, "Strategia RPG", "Intelligent Systems", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Mario Kart - Super Circuit", 2001, "Corse", "Intelligent Systems", gba_sub);
        add_game!("GBA", "Game Boy Advance", "WarioWare, Inc. - Minigame Mania", 2003, "Party / Puzzle", "Nintendo", gba_sub);
        add_game!("GBA", "Game Boy Advance", "Mega Man Battle Network 3 - Blue", 2002, "Action RPG", "Capcom", gba_sub);

        // ==========================================
        // SONY PLAYSTATION (PSX) - On-Demand Cloud
        // ==========================================
        let psx_sub = "Sony_-_PlayStation";
        add_game!("PSX", "PlayStation", "Final Fantasy VII", 1997, "RPG", "Square", psx_sub);
        add_game!("PSX", "PlayStation", "Final Fantasy VIII", 1999, "RPG", "Square", psx_sub);
        add_game!("PSX", "PlayStation", "Final Fantasy IX", 2000, "RPG", "Square", psx_sub);
        add_game!("PSX", "PlayStation", "Castlevania - Symphony of the Night", 1997, "Action RPG", "Konami", psx_sub);
        add_game!("PSX", "PlayStation", "Metal Gear Solid", 1998, "Stealth Action", "Konami", psx_sub);
        add_game!("PSX", "PlayStation", "Resident Evil 2", 1998, "Survival Horror", "Capcom", psx_sub);
        add_game!("PSX", "PlayStation", "Resident Evil 3 - Nemesis", 1999, "Survival Horror", "Capcom", psx_sub);
        add_game!("PSX", "PlayStation", "Silent Hill", 1999, "Survival Horror", "Konami", psx_sub);
        add_game!("PSX", "PlayStation", "Tekken 3", 1998, "Picchiaduro", "Namco", psx_sub);
        add_game!("PSX", "PlayStation", "Crash Team Racing", 1999, "Corse", "Naughty Dog", psx_sub);
        add_game!("PSX", "PlayStation", "Spyro the Dragon", 1998, "Platform 3D", "Insomniac Games", psx_sub);
        add_game!("PSX", "PlayStation", "Tony Hawk's Pro Skater 2", 2000, "Sport / Skate", "Neversoft", psx_sub);
        add_game!("PSX", "PlayStation", "Chrono Cross", 1999, "RPG", "Square", psx_sub);

        // ==========================================
        // NEOGEO & ARCADE - On-Demand Cloud
        // ==========================================
        let ng_sub = "SNK_-_Neo_Geo";
        add_game!("NEOGEO", "SNK Neo Geo", "Metal Slug X", 1999, "Run and Gun", "SNK", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "Metal Slug 3", 2000, "Run and Gun", "SNK", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "The King of Fighters '98", 1998, "Picchiaduro", "SNK", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "Garou - Mark of the Wolves", 1999, "Picchiaduro", "SNK", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "Samurai Shodown II", 1994, "Picchiaduro", "SNK", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "Shock Troopers", 1997, "Run and Gun", "Saurus", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "Blazing Star", 1998, "Shoot 'em up", "Yumekobo", ng_sub);
        add_game!("NEOGEO", "SNK Neo Geo", "Windjammers", 1994, "Sport / Azione", "Data East", ng_sub);

        // ==========================================
        // NINTENDO 64 - On-Demand Cloud
        // ==========================================
        let n64_sub = "Nintendo_-_Nintendo_64";
        add_game!("N64", "Nintendo 64", "Super Mario 64", 1996, "Platform 3D", "Nintendo", n64_sub);
        add_game!("N64", "Nintendo 64", "The Legend of Zelda - Ocarina of Time", 1998, "Action Adventure", "Nintendo", n64_sub);
        add_game!("N64", "Nintendo 64", "The Legend of Zelda - Majora's Mask", 2000, "Action Adventure", "Nintendo", n64_sub);
        add_game!("N64", "Nintendo 64", "GoldenEye 007", 1997, "FPS", "Rare", n64_sub);
        add_game!("N64", "Nintendo 64", "Banjo-Kazooie", 1998, "Platform 3D", "Rare", n64_sub);
        add_game!("N64", "Nintendo 64", "Super Smash Bros.", 1999, "Picchiaduro", "HAL Laboratory", n64_sub);
        add_game!("N64", "Nintendo 64", "Mario Kart 64", 1996, "Corse", "Nintendo", n64_sub);
        add_game!("N64", "Nintendo 64", "Paper Mario", 2000, "RPG", "Intelligent Systems", n64_sub);
        add_game!("N64", "Nintendo 64", "Star Fox 64", 1997, "Rail Shooter", "Nintendo", n64_sub);

        // ==========================================
        // COMMODORE 64 - On-Demand Cloud
        // ==========================================
        let c64_sub = "Commodore_-_64";
        add_game!("C64", "Commodore 64", "Turrican II - The Final Fight", 1991, "Run and Gun", "Rainbow Arts", c64_sub);
        add_game!("C64", "Commodore 64", "The Great Giana Sisters", 1987, "Platform", "Time Warp", c64_sub);
        add_game!("C64", "Commodore 64", "International Karate +", 1987, "Picchiaduro", "System 3", c64_sub);
        add_game!("C64", "Commodore 64", "Mayhem in Monsterland", 1993, "Platform", "Apex Computer", c64_sub);
        add_game!("C64", "Commodore 64", "Wizball", 1987, "Shoot 'em up", "Sensible Software", c64_sub);
        add_game!("C64", "Commodore 64", "California Games", 1987, "Sport", "Epyx", c64_sub);
        add_game!("C64", "Commodore 64", "Paradroid", 1985, "Azione / Fantascienza", "Graftgold", c64_sub);

        // ==========================================
        // LASERDISC & FMV INTERACTIVE MOVIE
        // ==========================================
        let fmv_sub = "MAME";
        add_game!("LASERDISC", "Laserdisc FMV", "Dragon's Lair", 1983, "Interactive Movie", "Cinematronics", fmv_sub);
        add_game!("LASERDISC", "Laserdisc FMV", "Space Ace", 1984, "Interactive Movie", "Cinematronics", fmv_sub);
        add_game!("LASERDISC", "Laserdisc FMV", "Mad Dog McCree", 1990, "Live-Action Lightgun", "American Laser Games", fmv_sub);

        // ==========================================
        // PINBALL & FLIPPER ELETTRONICI
        // ==========================================
        let pin_sub = "MAME";
        add_game!("PINBALL", "Pinball Simulator", "Medieval Madness", 1997, "Pinball Classic", "Williams", pin_sub);
        add_game!("PINBALL", "Pinball Simulator", "The Addams Family", 1992, "Pinball Classic", "Midway / Bally", pin_sub);
        add_game!("PINBALL", "Pinball Simulator", "Attack from Mars", 1995, "Pinball Classic", "Bally", pin_sub);

        // ==========================================
        // NINTENDO GAME & WATCH & TABLETOP
        // ==========================================
        let gw_sub = "Nintendo_-_Game_Boy";
        add_game!("GAMEWATCH", "Game & Watch", "Donkey Kong (Multi Screen)", 1982, "LCD Handheld", "Nintendo", gw_sub);
        add_game!("GAMEWATCH", "Game & Watch", "Ball (Silver Series)", 1980, "LCD Handheld", "Nintendo", gw_sub);
        add_game!("GAMEWATCH", "Game & Watch", "Zelda (Multi Screen)", 1989, "LCD Handheld", "Nintendo", gw_sub);

        // ==========================================
        // NIPPON MICRO COMPUTERS (MSX)
        // ==========================================
        let msx_sub = "Microsoft_-_MSX";
        add_game!("MSX", "MSX Home Computer", "Metal Gear", 1987, "Stealth Action", "Konami", msx_sub);
        add_game!("MSX", "MSX Home Computer", "Vampire Killer", 1986, "Action Adventure", "Konami", msx_sub);
        add_game!("MSX", "MSX Home Computer", "Nemesis 2", 1987, "Shoot 'em up", "Konami", msx_sub);

        // ==========================================
        // GCE VECTREX VECTOR CONSOLE
        // ==========================================
        let vec_sub = "GCE_-_Vectrex";
        add_game!("VECTREX", "GCE Vectrex", "Scramble", 1982, "Vector Shmup", "GCE", vec_sub);
        add_game!("VECTREX", "GCE Vectrex", "Space Wars", 1982, "Vector Space Combat", "GCE", vec_sub);
        add_game!("VECTREX", "GCE Vectrex", "Clean Sweep", 1982, "Vector Maze Action", "GCE", vec_sub);

        // ==========================================
        // MS-DOS PC CLASSICS
        // ==========================================
        let dos_sub = "DOS";
        add_game!("DOS", "MS-DOS PC Classics", "DOOM (1993)", 1993, "FPS", "id Software", dos_sub);
        add_game!("DOS", "MS-DOS PC Classics", "DOOM II - Hell on Earth", 1994, "FPS", "id Software", dos_sub);
        add_game!("DOS", "MS-DOS PC Classics", "Duke Nukem 3D", 1996, "FPS", "3D Realms", dos_sub);
        add_game!("DOS", "MS-DOS PC Classics", "Prince of Persia", 1989, "Cinematic Platformer", "Broderbund", dos_sub);

        // ==========================================
        // SCUMMVM POINT & CLICK ADVENTURES
        // ==========================================
        let scumm_sub = "ScummVM";
        add_game!("SCUMMVM", "ScummVM Point & Click", "Day of the Tentacle", 1993, "Avventura Grafica", "LucasArts", scumm_sub);
        add_game!("SCUMMVM", "ScummVM Point & Click", "Curse of Monkey Island, The", 1997, "Avventura Grafica", "LucasArts", scumm_sub);
        add_game!("SCUMMVM", "ScummVM Point & Click", "Monkey Island 2_ LeChuck's Revenge", 1991, "Avventura Grafica", "LucasArts", scumm_sub);

        // ==========================================
        // COMMODORE AMIGA 500 / 1200
        // ==========================================
        let amiga_sub = "Commodore_-_Amiga";
        add_game!("AMIGA", "Commodore Amiga (500/1200)", "Turrican II - The Final Fight", 1991, "Run and Gun", "Rainbow Arts", amiga_sub);
        add_game!("AMIGA", "Commodore Amiga (500/1200)", "All New World of Lemmings", 1994, "Puzzle Platform", "DMA Design", amiga_sub);
        add_game!("AMIGA", "Commodore Amiga (500/1200)", "Sensible Soccer", 1992, "Sport / Calcio", "Sensible Software", amiga_sub);

        // ==========================================
        // SEGA DREAMCAST & SATURN
        // ==========================================
        let dc_sub = "Sega_-_Dreamcast";
        add_game!("DREAMCAST", "Sega Dreamcast", "Crazy Taxi", 2000, "Arcade Racing", "Sega", dc_sub);
        add_game!("DREAMCAST", "Sega Dreamcast", "Shenmue", 1999, "Open World Adventure", "Sega AM2", dc_sub);
        add_game!("DREAMCAST", "Sega Dreamcast", "Sonic Adventure", 1998, "3D Platform", "Sega", dc_sub);

        let saturn_sub = "Sega_-_Saturn";
        add_game!("SATURN", "Sega Saturn", "NiGHTS into Dreams...", 1996, "Action 3D", "Sonic Team", saturn_sub);
        add_game!("SATURN", "Sega Saturn", "Sega Rally Championship", 1995, "Arcade Racing", "Sega", saturn_sub);

        // ==========================================
        // SONY PLAYSTATION 2
        // ==========================================
        let ps2_sub = "Sony_-_PlayStation_2";
        add_game!("PS2", "PlayStation 2", "Shadow of the Colossus", 2005, "Action Adventure", "Team Ico / Sony", ps2_sub);
        add_game!("PS2", "PlayStation 2", "Metal Gear Solid 3 - Snake Eater", 2004, "Stealth Action", "Konami", ps2_sub);

        // ==========================================
        // NEC PC ENGINE / TURBOGRAFX
        // ==========================================
        let pce_sub = "NEC_-_PC_Engine_-_TurboGrafx_16";
        add_game!("PCENGINE", "PC Engine / TurboGrafx", "Castlevania - Rondo of Blood", 1993, "Action Platform", "Konami", pce_sub);
        add_game!("PCENGINE", "PC Engine / TurboGrafx", "Bonk's Adventure", 1989, "Platform", "Hudson Soft", pce_sub);

        // ==========================================
        // ATARI 2600 PIONEERS
        // ==========================================
        let a26_sub = "Atari_-_2600";
        add_game!("ATARI2600", "Atari 2600", "Space Invaders", 1980, "Arcade Classic", "Atari / Taito", a26_sub);
        add_game!("ATARI2600", "Atari 2600", "River Raid", 1982, "Scrolling Shooter", "Activision", a26_sub);

        // ==========================================
        // PC INDIE & LOCOMALITO - Built-in Manifest
        // ==========================================
        static INDIE_JSON: &str = include_str!("indie_manifest.json");
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(INDIE_JSON) {
            if let Some(arr) = val.get("games").and_then(|g| g.as_array()) {
                for item in arr {
                    let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("Indie Game");
                    let dev = item.get("developer").and_then(|v| v.as_str()).unwrap_or("Indie Developer");
                    let genre = item.get("genre").and_then(|v| v.as_str()).unwrap_or("Indie");
                    let year = item.get("year").and_then(|v| v.as_u64()).unwrap_or(2010) as u16;
                    let controls = item.get("controls").and_then(|v| v.as_str()).unwrap_or("Gamepad Xbox / Tastiera");
                    let cover = item.get("cover_url").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let folder = item.get("folder_name").and_then(|v| v.as_str()).unwrap_or("");
                    let primary_exe = item.get("primary_exe").and_then(|v| v.as_str()).unwrap_or("");

                    games.push(Game {
                        id,
                        title: title.to_string(),
                        platform: "PC Indie".to_string(),
                        system_code: "INDIE".to_string(),
                        year,
                        genre: genre.to_string(),
                        developer: dev.to_string(),
                        publisher: dev.to_string(),
                        controls: controls.to_string(),
                        launch_target: format!("indie://{}/{}", folder, primary_exe),
                        cover_url: cover,
                        is_favorite: false,
                        play_count: 0,
                        last_played_at: None,
                        is_installed: false,
                    });
                    id += 1;
                }
            }
        }

        // ==========================================
        // PC RECOMPILATIONS - Built-in Manifest
        // ==========================================
        static RECOMP_JSON: &str = include_str!("recomp_manifest.json");
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(RECOMP_JSON) {
            if let Some(arr) = val.get("recomps").and_then(|g| g.as_array()) {
                for item in arr {
                    let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("PC Recomp");
                    let dev = item.get("developer").and_then(|v| v.as_str()).unwrap_or("Various");
                    let genre = item.get("genre").and_then(|v| v.as_str()).unwrap_or("Action");
                    let year = item.get("year").and_then(|v| v.as_u64()).unwrap_or(1998) as u16;
                    let controls = item.get("controls").and_then(|v| v.as_str()).unwrap_or("Gamepad Xbox / Tastiera");
                    let cover = item.get("cover_url").and_then(|v| v.as_str()).map(|s| s.to_string());
                    let exe_name = item.get("exe_name").and_then(|v| v.as_str()).unwrap_or("");
                    let orig_sys = item.get("original_system").and_then(|v| v.as_str()).unwrap_or("ARCADE");

                    games.push(Game {
                        id,
                        title: title.to_string(),
                        platform: format!("PC Recomp ({})", orig_sys),
                        system_code: "RECOMP".to_string(),
                        year,
                        genre: genre.to_string(),
                        developer: dev.to_string(),
                        publisher: dev.to_string(),
                        controls: controls.to_string(),
                        launch_target: format!("recomp://{}", exe_name),
                        cover_url: cover,
                        is_favorite: false,
                        play_count: 0,
                        last_played_at: None,
                        is_installed: false,
                    });
                    id += 1;
                }
            }
        }

        games
    }
}
