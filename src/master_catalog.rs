use crate::models::Game;

static MASTER_CLOUD_JSON: &str = include_str!("../data/master_cloud_catalog.json");

pub struct MasterCatalog;

impl MasterCatalog {
    /// Returns the built-in Master Cloud Catalog of ~1,600 legendary titles across all major retro systems.
    /// Games not present locally will have `is_installed: false` and are available on-demand.
    pub fn get_cloud_catalog(start_id: i64) -> Vec<Game> {
        let json_clean = MASTER_CLOUD_JSON.strip_prefix('\u{feff}').unwrap_or(MASTER_CLOUD_JSON);
        let mut games: Vec<Game> = match serde_json::from_str(json_clean) {
            Ok(list) => list,
            Err(e) => {
                eprintln!("⚠ Errore deserializzazione master_cloud_catalog.json: {}", e);
                Vec::new()
            }
        };

        // Strict safety filter: ensure zero BIOS, diagnostic, or hardware test cartridges enter the catalog
        games.retain(|g| {
            let t = g.title.to_lowercase();
            let gen = g.genre.to_lowercase();
            !t.contains("bios") && !t.contains("firmware") && !gen.contains("bios")
                && !t.contains("sound board") && !t.contains("audio board") && !t.contains("dimm board")
                && !t.contains("billboard") && !t.contains("c-chip") && !t.starts_with("namco 5")
                && !t.starts_with("motorola mc") && !t.starts_with("texas instruments")
                && !t.contains("qsound") && !t.contains("votrax") && !t.contains("ym2413") && !t.contains("ym2608")
                && !t.contains("bsmt2000") && !t.contains("system gx") && !t.contains("g net")
        });

        // Verifica automatica presenza locale su disco (MAME, NAOMI, PSX, DOS, Recomp, Indie)
        let mame_roms_dir = std::path::PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame\roms");
        let mut id = start_id;
        for g in &mut games {
            g.id = id;
            if let Some(rom) = g.launch_target.strip_prefix("MAME:") {
                let rom_zip = format!("{}.zip", rom.trim());
                if mame_roms_dir.join(&rom_zip).exists() {
                    g.is_installed = true;
                }
            } else {
                if let Some(target) = crate::launcher::Launcher::auto_resolve_target(g) {
                    g.launch_target = target;
                    g.is_installed = true;
                }
            }
            id += 1;
        }

        games
    }
}
