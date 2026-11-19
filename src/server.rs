use crate::db::Database;
use crate::launcher::Launcher;
use crate::lobby::{LobbyManager, LobbyRoom};
use crate::models::{LaunchRequest, LaunchResponse, SearchFilter};
use crate::netplay::NetplayManager;
use crate::search::SearchEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Response, Server, StatusCode};
use crate::library_config::LibraryConfig;

pub struct HttpServer {
    db: Arc<Mutex<Database>>,
    netplay: Arc<NetplayManager>,
    lobby: Arc<LobbyManager>,
    port: u16,
    shutdown: Arc<AtomicBool>,
    last_ping: Arc<AtomicU64>,
}

#[derive(Serialize)]
struct LobbyRoomInfo {
    id: String,
    game_title: String,
    platform: String,
    host_name: String,
    player_count: u8,
    max_players: u8,
    ping_ms: u64,
    region: String,
}

impl HttpServer {
    pub fn shutdown_flag(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }
}

#[derive(Deserialize)]
struct CreateRoomReq {
    game_id: i64,
    player_name: String,
}

#[derive(Deserialize)]
struct JoinRoomReq {
    token: String,
    player_name: String,
}

impl HttpServer {
    pub fn new(db: Database, port: u16) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            netplay: Arc::new(NetplayManager::new()),
            lobby: Arc::new(LobbyManager::new()),
            port,
            shutdown: Arc::new(AtomicBool::new(false)),
            last_ping: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let addr = format!("127.0.0.1:{}", self.port);
        let server = Server::http(&addr).map_err(|e| format!("Impossibile avviare il server HTTP su {}: {}", addr, e))?;

        println!("🚀 RetroHub UI Server in ascolto su: http://{}", addr);

        for mut request in server.incoming_requests() {
            if self.shutdown.load(Ordering::SeqCst) {
                break;
            }

            // Heartbeat timeout: se il browser non manda ping per 15 secondi (finestra chiusa), spegni il server
            let last = self.last_ping.load(Ordering::SeqCst);
            if last > 0 && now_secs().saturating_sub(last) > 15 {
                break;
            }

            let url = request.url().to_string();
            let method = request.method().clone();

            // Reset del timer heartbeat quando arriva una richiesta reale: il client ha il focus
            // e sta interagendo, quindi il server non deve spegnersi per timeout.
            if method != Method::Get || !url.starts_with("/api/cover") {
                self.last_ping.store(now_secs(), Ordering::SeqCst);
            }

            let (path, query_params) = parse_url(&url);

            // Servizio immagini locali generiche per file nel workspace/emu
            if (method == Method::Get || method == Method::Head) && path.starts_with("/api/cover/file/") {
                let sub_path = path.trim_start_matches("/api/cover/file/").trim();
                let clean_sub = sub_path.replace("%20", " ").replace("..", "");
                let emu_base = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");
                let full_path = emu_base.join(clean_sub);
                if full_path.exists() && full_path.is_file() {
                    if let Ok(bytes) = std::fs::read(&full_path) {
                        let mime = if full_path.extension().map_or(false, |e| e == "jpg" || e == "jpeg") {
                            "image/jpeg"
                        } else if full_path.extension().map_or(false, |e| e == "gif") {
                            "image/gif"
                        } else if full_path.extension().map_or(false, |e| e == "webp") {
                            "image/webp"
                        } else {
                            "image/png"
                        };
                        let header_ct = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();
                        let header_cache = Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=86400"[..]).unwrap();
                        let body = if method == Method::Head { Vec::new() } else { bytes };
                        let response = Response::from_data(body).with_header(header_ct).with_header(header_cache);
                        let _ = request.respond(response);
                        continue;
                    }
                }
                let _ = request.respond(Response::empty(404));
                continue;
            }

            // Servizio immagini locali MAME ad alta velocità (Flyers, Titles, Snaps)
            if (method == Method::Get || method == Method::Head) && path.starts_with("/api/cover/mame/") {
                let rom = path.trim_start_matches("/api/cover/mame/").trim();
                let clean_rom = rom.replace(['/', '\\', '.', ':', ';'], "");
                let emu_mame = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame");
                let candidates = [
                    emu_mame.join("flyers").join(format!("{}.png", clean_rom)),
                    emu_mame.join("flyers").join(format!("{}.jpg", clean_rom)),
                    emu_mame.join("titles").join(format!("{}.png", clean_rom)),
                    emu_mame.join("titles").join(format!("{}.jpg", clean_rom)),
                    emu_mame.join("snap").join(format!("{}.png", clean_rom)),
                    emu_mame.join("snap").join(format!("{}.jpg", clean_rom)),
                ];
                let found_file = candidates.into_iter().find(|p| p.exists());
                if let Some(path_buf) = found_file {
                    if let Ok(bytes) = std::fs::read(&path_buf) {
                        if bytes.len() > 500 && !bytes.starts_with(b"<!DOCTYPE") {
                            let mime = if path_buf.extension().map_or(false, |e| e == "jpg" || e == "jpeg") {
                                "image/jpeg"
                            } else {
                                "image/png"
                            };
                            let header_ct = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();
                            let header_cache = Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=86400"[..]).unwrap();
                            let body = if method == Method::Head { Vec::new() } else { bytes };
                            let response = Response::from_data(body).with_header(header_ct).with_header(header_cache);
                            let _ = request.respond(response);
                            continue;
                        }
                    }
                }
                let _ = request.respond(Response::empty(404));
                continue;
            }

            // Servizio boxart locali SEGA NAOMI / ATOMISWAVE (18 boxart ad alta risoluzione su disco a 0ms)
            if (method == Method::Get || method == Method::Head) && path.starts_with("/api/cover/naomi/") {
                let rom = path.trim_start_matches("/api/cover/naomi/").trim();
                let clean_rom = rom.replace(['/', '\\', '.', ':', ';'], "");
                let emu_naomi = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\NAOMI - Flycast\data\boxart");
                let mut candidates = vec![
                    emu_naomi.join(format!("{}.png", clean_rom)),
                    emu_naomi.join(format!("{}.jpg", clean_rom)),
                ];
                if clean_rom == "vtenis2c" || clean_rom == "vtennis2" {
                    candidates.push(emu_naomi.join("76261428.jpg"));
                }
                let found_file = candidates.into_iter().find(|p| p.exists());
                if let Some(path_buf) = found_file {
                    if let Ok(bytes) = std::fs::read(&path_buf) {
                        if bytes.len() > 500 && !bytes.starts_with(b"<!DOCTYPE") {
                            let mime = if path_buf.extension().map_or(false, |e| e == "jpg" || e == "jpeg") {
                                "image/jpeg"
                            } else {
                                "image/png"
                            };
                            let header_ct = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();
                            let header_cache = Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=86400"[..]).unwrap();
                            let body = if method == Method::Head { Vec::new() } else { bytes };
                            let response = Response::from_data(body).with_header(header_ct).with_header(header_cache);
                            let _ = request.respond(response);
                            continue;
                        }
                    }
                }
                let _ = request.respond(Response::empty(404));
                continue;
            }

            match (&method, path.as_str()) {
                (&Method::Get, "/" | "/index.html") => {
                    let html = include_str!("ui.html");
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
                    let no_cache = Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, no-store, must-revalidate"[..]).unwrap();
                    let response = Response::from_string(html).with_header(header).with_header(no_cache);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/games") => {
                    let filter = SearchFilter {
                        query: query_params.get("query").cloned(),
                        platform: query_params.get("platform").cloned(),
                        genre: query_params.get("genre").cloned(),
                        developer: query_params.get("developer").cloned(),
                        decade: query_params.get("decade").cloned(),
                        year_from: query_params.get("year_from").and_then(|v| v.parse::<u16>().ok()),
                        year_to: query_params.get("year_to").and_then(|v| v.parse::<u16>().ok()),
                        favorites_only: query_params.get("favorites_only").map_or(false, |v| v == "true"),
                        installed_only: query_params.get("installed_only").map_or(false, |v| v == "true"),
                        sort_by: query_params.get("sort_by").cloned(),
                        sort_asc: query_params.get("sort_asc").map_or(true, |v| v != "false"),
                    };

                    let db = self.db.lock().unwrap();
                    let all_games = db.get_all_games().unwrap_or_default();
                    let filtered = SearchEngine::filter_and_rank(&all_games, &filter);

                    let json = serde_json::to_string(&filtered).unwrap_or_else(|_| "[]".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/platforms") => {
                    let db = self.db.lock().unwrap();
                    let platforms = db.get_platforms().unwrap_or_default();
                    let json = serde_json::to_string(&platforms).unwrap_or_else(|_| "[]".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/decades") => {
                    let db = self.db.lock().unwrap();
                    let decades = db.get_decades().unwrap_or_default();
                    let json = serde_json::to_string(&decades).unwrap_or_else(|_| "[]".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/genres") => {
                    let db = self.db.lock().unwrap();
                    let genres = db.get_genres().unwrap_or_default();
                    let json = serde_json::to_string(&genres).unwrap_or_else(|_| "[]".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/developers") => {
                    let db = self.db.lock().unwrap();
                    let devs = db.get_developers().unwrap_or_default();
                    let json = serde_json::to_string(&devs).unwrap_or_else(|_| "[]".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/stats") => {
                    let db = self.db.lock().unwrap();
                    let stats = db.get_stats().ok();
                    let json = serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/manifest/platforms") => {
                    let json = include_str!("platforms_manifest.json");
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/manifest/indie") => {
                    let json = include_str!("indie_manifest.json");
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/manifest/recomp") => {
                    let json = include_str!("recomp_manifest.json");
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Post, "/api/launch") => {
                    let body = read_request_body(&mut request);
                    let req: Result<LaunchRequest, _> = serde_json::from_str(&body);

                    let res = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            if let Ok(Some(game)) = db.get_game_by_id(r.game_id) {
                                match Launcher::launch_game(&game) {
                                    Ok(msg) => {
                                        let play_count = db.record_play(r.game_id).unwrap_or(game.play_count + 1);
                                        LaunchResponse {
                                            success: true,
                                            message: format!("{}: {}", game.title, msg),
                                            play_count,
                                        }
                                    }
                                    Err(err) => LaunchResponse {
                                        success: false,
                                        message: err,
                                        play_count: game.play_count,
                                    },
                                }
                            } else {
                                LaunchResponse {
                                    success: false,
                                    message: "Gioco non trovato nel database".to_string(),
                                    play_count: 0,
                                }
                            }
                        }
                        Err(e) => LaunchResponse {
                            success: false,
                            message: format!("Payload non valido: {}", e),
                            play_count: 0,
                        },
                    };

                    let json = serde_json::to_string(&res).unwrap_or_else(|_| "{}".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Post, "/api/favorite") => {
                    let body = read_request_body(&mut request);
                    eprintln!("Favorite API body: {:?}", body);
                    let req: Result<LaunchRequest, _> = serde_json::from_str(&body);
                    if let Err(ref e) = req {
                        eprintln!("Favorite JSON parse error: {}", e);
                    }

                    let (status, is_fav) = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            let is_fav = db.toggle_favorite(r.game_id).unwrap_or(false);
                            (StatusCode(200), is_fav)
                        }
                        Err(_) => (StatusCode(400), false),
                    };

                    let json = format!(r#"{{"is_favorite": {}}}"#, is_fav);
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header).with_status_code(status);
                    let _ = request.respond(response);
                }

                // --- NETPLAY API ---
                (&Method::Post, "/api/netplay/create") => {
                    let body = read_request_body(&mut request);
                    let req: Result<CreateRoomReq, _> = serde_json::from_str(&body);

                    let resp_json = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            if let Ok(Some(game)) = db.get_game_by_id(r.game_id) {
                                match self.netplay.create_room(game.id, game.title, game.platform, r.player_name) {
                                    Ok(sess) => serde_json::to_string(&sess).unwrap(),
                                    Err(e) => format!(r#"{{"error": "{}"}}"#, e),
                                }
                            } else {
                                r#"{"error": "Gioco non trovato"}"#.to_string()
                            }
                        }
                        Err(e) => format!(r#"{{"error": "Payload invalido: {}"}}"#, e),
                    };

                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp_json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Post, "/api/netplay/join") => {
                    let body = read_request_body(&mut request);
                    let req: Result<JoinRoomReq, _> = serde_json::from_str(&body);

                    let resp_json = match req {
                        Ok(r) => match self.netplay.join_room(&r.token, r.player_name) {
                            Ok(sess) => serde_json::to_string(&sess).unwrap(),
                            Err(e) => format!(r#"{{"error": "{}"}}"#, e),
                        },
                        Err(e) => format!(r#"{{"error": "Payload invalido: {}"}}"#, e),
                    };

                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp_json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/netplay/status") => {
                    let sess = self.netplay.get_session();
                    let json = serde_json::to_string(&sess).unwrap_or_else(|_| "null".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Post, "/api/netplay/start") => {
                    let db = self.db.lock().unwrap();
                    let sess = self.netplay.get_session();

                    let res = if let Some(s) = sess {
                        let _ = self.netplay.start_game();
                        if let Ok(Some(game)) = db.get_game_by_id(s.game_id) {
                            let _ = Launcher::launch(&game.launch_target);
                        }
                        r#"{"success": true, "message": "Partita online avviata"}"#
                    } else {
                        r#"{"success": false, "message": "Nessuna sessione attiva"}"#
                    };

                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(res).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Post, "/api/netplay/leave") => {
                    self.netplay.close_room();
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(r#"{"success": true}"#).with_header(header);
                    let _ = request.respond(response);
                }

                // LOBBY API - List public rooms
                (&Method::Get, "/api/lobby/rooms") => {
                    self.lobby.cleanup_expired();
                    let rooms = self.lobby.list_rooms();
                    let room_infos: Vec<LobbyRoomInfo> = rooms
                        .into_iter()
                        .map(|r| LobbyRoomInfo {
                            id: r.id,
                            game_title: r.game_title,
                            platform: r.platform,
                            host_name: r.host_name,
                            player_count: r.player_count,
                            max_players: r.max_players,
                            ping_ms: r.ping_ms,
                            region: r.region,
                        })
                        .collect();
                    let json = serde_json::to_string(&room_infos).unwrap_or_else(|_| "[]".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // LOBBY API - Create room (host)
                (&Method::Post, "/api/lobby/create") => {
                    let body = read_request_body(&mut request);
                    let req: Result<CreateRoomReq, _> = serde_json::from_str(&body);
                    
                    let resp_json = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            if let Ok(Some(game)) = db.get_game_by_id(r.game_id) {
                                let ping = r.player_name.len() as u64 * 10;
                                let room_id = self.lobby.create_room(
                                    game.id,
                                    game.title.clone(),
                                    game.platform.clone(),
                                    r.player_name,
                                    "127.0.0.1".to_string(),
                                    self.port,
                                    ping,
                                );
                                serde_json::to_string(&serde_json::json!({
                                    "success": true,
                                    "room_id": room_id,
                                    "game_title": game.title,
                                    "platform": game.platform
                                })).unwrap_or_else(|_| r#"{"error": "JSON encode fallito"}"#.to_string())
                            } else {
                                r#"{"error": "Gioco non trovato"}"#.to_string()
                            }
                        }
                        Err(e) => format!(r#"{{"error": "Payload invalido: {0}"}}"#, e),
                    };
                    
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp_json).with_header(header);
                    let _ = request.respond(response);
                }

                // LOBBY API - Join room (guest)
                (&Method::Post, "/api/lobby/join") => {
                    let body = read_request_body(&mut request);
                    let req: Result<JoinRoomReq, _> = serde_json::from_str(&body);
                    
                    let resp_json = match req {
                        Ok(r) => match self.lobby.join_room(&r.token, r.player_name) {
                            Ok((_, connection_token)) => serde_json::to_string(&serde_json::json!({
                                "success": true,
                                "connection_token": connection_token
                            })).unwrap_or_else(|_| r#"{"error": "JSON encode fallito"}"#.to_string()),
                            Err(e) => format!(r#"{{"error": "{0}"}}"#, e),
                        },
                        Err(e) => format!(r#"{{"error": "Payload invalido: {0}"}}"#, e),
                    };
                    
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp_json).with_header(header);
                    let _ = request.respond(response);
                }

                // LIBRARY API - Get library config
                (&Method::Get, "/api/library/config") => {
                    let json = r#"{"platform_whitelist": [], "show_all_games": false, "auto_scan_roms": true, "download_on_demand": true, "prefer_keep_rom_after_game": false, "rom_scan_paths": ["C:\\Emulators\\ROMS", "D:\\Giochi\\ROMS", "E:\\Retrogaming"], "cdn_mirror": {"libretro_covers": "https://raw.githubusercontent.com/libretro-thumbnails", "internet_archive_roms": "https://archive.org/download", "igdb_covers": "https://images.igdb.com/igdb/image/upload/t_cover_big"}}"#;
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // LIBRARY API - Update library config
                (&Method::Post, "/api/library/config") => {
                    let body = read_request_body(&mut request);
                    let _config: Result<LibraryConfig, _> = serde_json::from_str(&body);
                    let json = r#"{"success": true}"#;
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // LIBRARY API - Scan ROMs
                (&Method::Post, "/api/library/scan") => {
                    let json = r#"{"success": true, "found": 0, "message": "Scan completato"}"#;
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // LIBRARY API - Download ROM
                (&Method::Post, "/api/library/download") => {
                    let body = read_request_body(&mut request);
                    let req: Result<LaunchRequest, _> = serde_json::from_str(&body);

                    let res_json = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            if let Ok(Some(game)) = db.get_game_by_id(r.game_id) {
                                let rom_name = crate::download_manager::DownloadManager::get_rom_filename(&game.title, &game.launch_target, &game.system_code);
                                match crate::download_manager::DownloadManager::download_rom(game.id, &rom_name, &game.system_code) {
                                    Ok(path) => {
                                        serde_json::to_string(&serde_json::json!({
                                            "success": true,
                                            "message": format!("ROM scaricata con successo per: {}", game.title),
                                            "path": path.to_string_lossy()
                                        })).unwrap()
                                    }
                                    Err(err) => {
                                        serde_json::to_string(&serde_json::json!({
                                            "success": false,
                                            "message": err
                                        })).unwrap()
                                    }
                                }
                            } else {
                                r#"{"success": false, "message": "Gioco non trovato"}"#.to_string()
                            }
                        }
                        Err(e) => format!(r#"{{"success": false, "message": "Payload non valido: {}"}}"#, e),
                    };

                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(res_json).with_header(header);
                    let _ = request.respond(response);
                }

                // LIBRARY API - Delete ROM
                (&Method::Post, "/api/library/delete") => {
                    let body = read_request_body(&mut request);
                    let req: Result<LaunchRequest, _> = serde_json::from_str(&body);

                    let res_json = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            if let Ok(Some(game)) = db.get_game_by_id(r.game_id) {
                                let rom_name = crate::download_manager::DownloadManager::get_rom_filename(&game.title, &game.launch_target, &game.system_code);
                                let download_path = std::path::PathBuf::from(format!("downloads/{}/{}", game.system_code.to_lowercase(), rom_name));
                                match crate::download_manager::DownloadManager::delete_rom(&download_path) {
                                    Ok(_) => {
                                        serde_json::to_string(&serde_json::json!({
                                            "success": true,
                                            "message": format!("ROM eliminata per: {}", game.title)
                                        })).unwrap()
                                    }
                                    Err(err) => {
                                        serde_json::to_string(&serde_json::json!({
                                            "success": false,
                                            "message": err
                                        })).unwrap()
                                    }
                                }
                            } else {
                                r#"{"success": false, "message": "Gioco non trovato"}"#.to_string()
                            }
                        }
                        Err(e) => format!(r#"{{"success": false, "message": "Payload non valido: {}"}}"#, e),
                    };

                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(res_json).with_header(header);
                    let _ = request.respond(response);
                }

                // Heartbeat: il browser manda ping periodici. Aggiorna il timestamp dell'ultimo ping.
                (&Method::Post, "/api/ping") => {
                    self.last_ping.store(now_secs(), Ordering::SeqCst);
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(r#"{"success": true}"#).with_header(header);
                    let _ = request.respond(response);
                }

                // Chiude il server quando la finestra del browser viene chiusa
                (&Method::Post, "/api/shutdown") => {
                    self.shutdown.store(true, Ordering::SeqCst);
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(r#"{"success": true}"#).with_header(header);
                    let _ = request.respond(response);
                    break;
                }

                _ => {
                    let response = Response::from_string("404 Not Found").with_status_code(StatusCode(404));
                    let _ = request.respond(response);
                }
            }
        }

        Ok(())
    }
}

fn parse_url(url: &str) -> (String, HashMap<String, String>) {
    let mut parts = url.splitn(2, '?');
    let path = parts.next().unwrap_or("").to_string();
    let mut params = HashMap::new();

    if let Some(query) = parts.next() {
        for pair in query.split('&') {
            let mut kv = pair.splitn(2, '=');
            if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                let decoded_val = urlencoding_decode(v);
                params.insert(k.to_string(), decoded_val);
            }
        }
    }

    (path, params)
}

fn urlencoding_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            }
        } else if ch == '+' {
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    result
}

fn read_request_body(request: &mut tiny_http::Request) -> String {
    let mut buf = Vec::new();
    use std::io::Read;
    let _ = request.as_reader().read_to_end(&mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
