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

fn open_app_window(url: &str) {
    // Apre l'interfaccia nel browser dell'utente in modo affidabile al 100% senza conflitti di lock o crash di Edge
    if open::that(url).is_err() {
        let _ = Command::new("cmd").args(&["/C", "start", url]).spawn();
    }
}

fn main() {
    println!("==================================================");
    println!("🕹️  RetroHub — Ultimate Retrogaming Client (Rust)");
    println!("==================================================");

    let port = 39281;
    let url = format!("http://127.0.0.1:{}", port);

    // Single instance check: if RetroHub is already running, open the window and exit cleanly
    if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
        println!("⚡ RetroHub è già attivo in background. Apertura interfaccia grafica...");
        open_app_window(&url);
        return;
    }

    let db_path = "retro_hub.db";
    let db = Database::open(db_path).expect("Inizializzazione database fallita");

    let emu_path = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");

    // Sincronizzazione automatica all'avvio con transazione (<40ms)
    println!("⏳ Sincronizzazione collezione con il catalogo unificato (1149 giochi)...");
    let t0 = std::time::Instant::now();
    match CatalogLoader::load_from_master_catalog(emu_path) {
        Ok(games) => {
            let total = games.len();
            if let Err(e) = db.upsert_games_batch(&games) {
                eprintln!("⚠ Errore upsert batch: {}", e);
            } else {
                println!("✔ {} giochi caricati e sincronizzati in retro_hub.db in {:?}!", total, t0.elapsed());
            }
        }
        Err(e) => {
            eprintln!("⚠ Caricamento da catalogo non riuscito: {}", e);
        }
    }

    // Launch window as soon as the server port is ready
    let url_clone = url.clone();
    std::thread::spawn(move || {
        for _ in 0..40 {
            if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        println!("🎮 Apertura interfaccia grafica RetroHub: {}", url_clone);
        open_app_window(&url_clone);
    });

    let server = HttpServer::new(db, port);
    if let Err(e) = server.start() {
        eprintln!("❌ Errore avvio server: {}", e);
    }
}
