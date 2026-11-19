use crate::models::Game;

static MASTER_CLOUD_JSON: &str = include_str!("../data/master_cloud_catalog.json");

pub struct MasterCatalog;

impl MasterCatalog {
    /// Returns the built-in Master Cloud Catalog of ~1,600 legendary titles across all major retro systems.
    /// Games not present locally will have `is_installed: false` and are available on-demand.
    pub fn get_cloud_catalog(start_id: i64) -> Vec<Game> {
        let mut games: Vec<Game> = match serde_json::from_str(MASTER_CLOUD_JSON) {
            Ok(list) => list,
            Err(e) => {
                eprintln!("⚠ Errore deserializzazione master_cloud_catalog.json: {}", e);
                Vec::new()
            }
        };

        // Reassign sequential IDs based on start_id and guarantee is_installed = false
        let mut id = start_id;
        for g in &mut games {
            g.id = id;
            g.is_installed = false;
            id += 1;
        }

        games
    }
}
