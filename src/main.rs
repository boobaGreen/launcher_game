#![windows_subsystem = "windows"]

pub mod models;
pub mod config;
pub mod core;
pub mod plugins;
pub mod launcher;
pub mod search;
pub mod db;
pub mod catalog;
pub mod server;
pub mod netplay;
pub mod lobby;
pub mod library_config;
pub mod platform_wizard;
pub mod rom_scanner;
pub mod download_manager;
pub mod master_catalog;

use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use catalog::CatalogLoader;
use config::RetroHubConfig;
use db::Database;
use plugins::{load_plugins, PluginManager};
use server::HttpServer;

fn is_server_alive(_url: &str) -> bool {
    // Verifica che il server esistente risponda davvero a una richiesta HTTP (senza bloccarsi)
    use std::io::{Read, Write};
    if let Ok(mut stream) = std::net::TcpStream::connect_timeout(
        &"127.0.0.1:39281".parse().unwrap(),
        std::time::Duration::from_millis(300),
    ) {
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_millis(300)));
        let _ = stream.set_write_timeout(Some(std::time::Duration::from_millis(300)));
        let req = "GET /api/platforms HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n";
        if stream.write_all(req.as_bytes()).is_ok() {
            let mut buf = [0u8; 512];
            if let Ok(n) = stream.read(&mut buf) {
                let resp = String::from_utf8_lossy(&buf[..n]);
                return resp.contains("200");
            }
        }
    }
    false
}

fn open_app_window(url: &str) {
    // Apre RetroHub come finestra applicativa standalone dedicata (modalità App senza barra URL o schede)
    let edge_paths = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];

    let user_data = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| r"C:\Temp".to_string())
        + r"\RetroHub\edge_profile";

    for path in &edge_paths {
        if Path::new(path).exists() {
            if Command::new(path)
                .args(&[
                    &format!("--app={}", url),
                    &format!("--user-data-dir={}", user_data),
                    "--window-size=1440,900",
                ])
                .spawn()
                .is_ok()
            {
                return;
            }
        }
    }

    // Fallback: browser di default se Edge non è presente
    if open::that(url).is_err() {
        let _ = Command::new("cmd").args(&["/C", "start", url]).spawn();
    }
}

fn main() {
    println!("==================================================");
    println!("🕹️  RetroHub — Ultimate Retrogaming Client (Rust)");
    println!("==================================================");

    let config = RetroHubConfig::load();
    let port = 39281;
    let url = format!("http://127.0.0.1:{}", port);

    // Single instance check: if RetroHub is already running, open the window and exit cleanly
    if is_server_alive(&url) {
        println!("⚡ RetroHub è già attivo in background. Apertura interfaccia grafica...");
        open_app_window(&url);
        return;
    }

    let db_path = "retro_hub.db";
    let db = Database::open(db_path).expect("Inizializzazione database fallita");

    let emu_path_buf = config.emu_root();
    let emu_path = emu_path_buf.as_path();
    
    // Salva configurazione se non esiste
    if !Path::new("retrohub.json").exists() {
        if let Err(e) = RetroHubConfig::default().save() {
            eprintln!("⚠ Errore salvatura configurazione: {}", e);
        } else {
            println!("✅ Configurazione iniziale creata in retrohub.json");
        }
    }
    
    // Load plugins
    let plugins = load_plugins();
    println!("🔌 Plugin caricati: MAME, Gluiz");

    // Sincronizzazione catalogo: se emu_root è vuoto (Modalità Vergine), popola la libreria on-demand cloud da zero
    let t0 = std::time::Instant::now();
    let emu_str = emu_path.to_string_lossy();
    if emu_str.trim().is_empty() || !emu_path.exists() {
        println!("✨ RetroHub avviato in Modalità Vergine (Nuovo Utente): 0 file locali collegati.");
        println!("☁️ Inizializzazione Catalogo Cloud On-Demand completo...");
        let cloud_games = crate::master_catalog::MasterCatalog::get_cloud_catalog(1);
        let total = cloud_games.len();
        if let Err(e) = db.reset_catalog(&cloud_games) {
            eprintln!("⚠ Errore reset catalogo cloud: {}", e);
        } else {
            println!("✔ {} giochi Cloud On-Demand inizializzati a 0 byte in {:?}!", total, t0.elapsed());
        }
    } else {
        println!("⏳ Sincronizzazione collezione da cartella locale: {}...", emu_str);
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
    }

    // Launch window in background thread as soon as server port is ready
    let url_clone = url.clone();
    std::thread::spawn(move || {
        for _ in 0..60 {
            if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        println!("🎮 Apertura interfaccia grafica RetroHub: {}", url_clone);
        open_app_window(&url_clone);
    });

    // Start server on main thread (blocking)
    let server = HttpServer::new(db, port);
    if let Err(e) = server.start() {
        eprintln!("❌ Errore avvio server: {}", e);
    }
    println!("👋 RetroHub chiuso. Arrivederci!");
}
