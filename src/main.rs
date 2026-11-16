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
    // Launch through Windows Shell to let the OS broker create a dedicated app window without lock conflicts
    let shell_app = Command::new("cmd")
        .args(&["/c", "start", "msedge", &format!("--app={}", url), "--window-size=1360,860"])
        .spawn();

    if shell_app.is_err() {
        let _ = open::that(url);
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

    // Sincronizzazione automatica all'avvio: aggiorna i percorsi e i link alle copertine in <80ms preservando i preferiti
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

    // Launch window in dedicated thread
    let url_clone = url.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(350));
        println!("🎮 Apertura interfaccia grafica RetroHub...");
        open_app_window(&url_clone);
    });

    let server = HttpServer::new(db, port);
    if let Err(e) = server.start() {
        eprintln!("❌ Errore avvio server: {}", e);
    }
}
