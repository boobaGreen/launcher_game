pub mod models;
pub mod launcher;
pub mod search;
pub mod db;
pub mod catalog;

use std::path::Path;
use catalog::CatalogLoader;
use db::Database;

fn main() {
    println!("==================================================");
    println!("🕹️  RetroHub Core Engine (Rust)");
    println!("==================================================");

    let emu_path = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");
    match CatalogLoader::load_from_master_catalog(emu_path) {
        Ok(games) => {
            println!("✔ Caricati con successo {} giochi dal Master Catalog!", games.len());

            // Initialize DB in memory or file
            let db_path = "retro_hub.db";
            let db = Database::open(db_path).expect("Inizializzazione database fallita");
            for g in &games {
                let _ = db.upsert_game(g);
            }
            println!("✔ Database SQLite sincronizzato con successo (retro_hub.db).");

            let platforms = db.get_platforms().unwrap();
            println!("\nPiattaforme disponibili:");
            for p in platforms {
                println!("  - {}: {} giochi ({})", p.name, p.count, p.code);
            }
        }
        Err(e) => {
            eprintln!("❌ Errore caricamento catalogo: {}", e);
        }
    }
}
