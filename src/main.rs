pub mod models;
pub mod launcher;
pub mod search;
pub mod db;
pub mod catalog;
pub mod server;
pub mod netplay;

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

    let count = db.get_all_games().unwrap_or_default().len();
    let force_refresh = std::env::args().any(|a| a == "--refresh");

    let emu_path = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");

    if count == 0 || force_refresh {
        println!("⏳ Sincronizzazione collezione con il catalogo unificato (1149 giochi)...");
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
    } else {
        println!("⚡ Avvio istantaneo: {} giochi pronti nel database locale.", count);
    }

    let port = 39281;
    let url = format!("http://127.0.0.1:{}", port);

    // Launch window in dedicated thread
    let url_clone = url.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(350));
        println!("🎮 Apertura interfaccia grafica RetroHub...");

        let edge_candidates = [
            r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
            r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        ];

        let mut launched = false;
        for exe in edge_candidates {
            if Path::new(exe).exists() {
                if Command::new(exe)
                    .args([&format!("--app={}", url_clone), "--window-size=1360,860"])
                    .spawn()
                    .is_ok()
                {
                    launched = true;
                    break;
                }
            }
        }

        if !launched {
            let _ = open::that(&url_clone);
        }
    });

    let server = HttpServer::new(db, port);
    if let Err(e) = server.start() {
        eprintln!("❌ Errore avvio server: {}", e);
    }
}
