use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryConfig {
    pub platform_whitelist: Vec<String>,
    pub show_all_games: bool,
    pub auto_scan_roms: bool,
    pub download_on_demand: bool,
    pub prefer_keep_rom_after_game: bool,
    pub rom_scan_paths: Vec<String>,
    pub cdn_mirror: CdnMirror,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CdnMirror {
    pub libretro_covers: String,
    pub internet_archive_roms: String,
    pub igdb_covers: String,
}