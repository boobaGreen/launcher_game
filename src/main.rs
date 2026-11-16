pub mod models;
pub mod launcher;
pub mod search;
pub mod db;
pub mod catalog;
pub mod server;

use std::path::Path;
use std::process::Command;
use catalog::CatalogLoader;
use db::Database;
use server::HttpServer;

fn main() {
    println!("==================================================");
    println!("🕹️  RetroHub — Ultimate Retrogaming Client (Rust)");
    println!("==================================================");

    let db_path = "retro_hub.db";
    let db = Database::open(db_path).expect("Inizializzazione database fallita");

    let emu_path = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");
    println!("⏳ Sincronizzazione collezione con il catalogo unificato...");
    match CatalogLoader::load_from_master_catalog(emu_path) {
        Ok(games) => {
            let total = games.len();
            for g in &games {
                let _ = db.upsert_game(g);
            }
            println!("✔ {} giochi caricati e sincronizzati in retro_hub.db!", total);
        }
        Err(e) => {
            eprintln!("⚠ Caricamento da catalogo non riuscito: {}", e);
        }
    }

    let port = 39281;
    let url = format!("http://127.0.0.1:{}", port);

    // Launch window in dedicated thread
    let url_clone = url.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("🎮 Apertura interfaccia grafica in corso...");

        // Try launching Edge in App mode (Native Frameless Desktop Window)
        let edge_app = Command::new("msedge.exe")
            .args([&format!("--app={}", url_clone), "--window-size=1280,820"])
            .spawn();

        if edge_app.is_err() {
            // Fallback to default browser
            let _ = open::that(&url_clone);
        }
    });

    let server = HttpServer::new(db, port);
    if let Err(e) = server.start() {
        eprintln!("❌ Errore avvio server: {}", e);
    }
}
