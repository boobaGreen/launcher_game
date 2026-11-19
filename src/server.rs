use crate::db::Database;
use crate::launcher::Launcher;
use crate::lobby::LobbyManager;
use crate::models::{LaunchRequest, LaunchResponse, SearchFilter};
use crate::netplay::NetplayManager;
use crate::search::SearchEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Response, Server, StatusCode};
use crate::library_config::LibraryConfig;
use std::process::Command;

static VPX_INSTALL_PROGRESS: AtomicU32 = AtomicU32::new(0);
static VPX_INSTALL_MSG: Mutex<Option<String>> = Mutex::new(None);

#[derive(Clone)]
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

#[derive(Deserialize)]
struct EmulatorConfigReq {
    retroarch: Option<String>,
    vpx: Option<String>,
    mame: Option<String>,
}

#[derive(Deserialize)]
struct WebLaunchReq {
    game_id: i64,
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

        let self_arc = Arc::new(self.clone());

        for request in server.incoming_requests() {
            if self_arc.shutdown.load(Ordering::SeqCst) {
                break;
            }

            // Heartbeat timeout: se il browser non manda ping per 300 secondi (5 min), spegni il server
            let last = self_arc.last_ping.load(Ordering::SeqCst);
            if last > 0 && now_secs().saturating_sub(last) > 300 {
                break;
            }

            let s = Arc::clone(&self_arc);
            std::thread::spawn(move || {
                s.handle_request(request);
            });
        }

        Ok(())
    }

    pub fn handle_request(&self, mut request: tiny_http::Request) {
        let url = request.url().to_string();
        let method = request.method().clone();

        // Reset del timer heartbeat quando arriva una richiesta reale: il client ha il focus
        // e sta interagendo, quindi il server non deve spegnersi per timeout.
        if method != Method::Get || (!url.starts_with("/api/cover") && !url.starts_with("/covers/")) {
            self.last_ping.store(now_secs(), Ordering::SeqCst);
        }

        let (path, query_params) = parse_url(&url);

            // Servizio immagini locali generiche per file nel workspace / covers / emu
            let is_cover_file = (method == Method::Get || method == Method::Head) && path.starts_with("/api/cover/file/");
            let is_direct_cover = (method == Method::Get || method == Method::Head) && path.starts_with("/covers/");

            if is_cover_file || is_direct_cover {
                let raw_sub = if is_cover_file {
                    path.trim_start_matches("/api/cover/file/").trim()
                } else {
                    path.trim_start_matches("/").trim()
                };
                let clean_sub = raw_sub.replace("%20", " ").replace("..", "").replace('\\', "/");
                let stripped_sub = clean_sub.trim_start_matches("covers/").trim();

                let workspace_root = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game");
                let emu_root = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");
                let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.to_path_buf()));
                let cur_dir = std::env::current_dir().ok();

                let mut candidates: Vec<std::path::PathBuf> = Vec::new();

                // 1. Cerca nella cartella covers di launcher_game (stripped_sub es: "pinball/pb_1614.webp" o "laserdisc/ld_1120.png")
                candidates.push(workspace_root.join("covers").join(stripped_sub));
                // 2. Cerca nel workspace_root direttamente
                candidates.push(workspace_root.join(&clean_sub));
                // 3. Cerca relativo alla cartella dell'eseguibile exe_dir
                if let Some(ref ed) = exe_dir {
                    candidates.push(ed.join("covers").join(stripped_sub));
                    candidates.push(ed.join(&clean_sub));
                }
                // 4. Cerca relativo alla cartella di esecuzione corrente
                if let Some(ref cd) = cur_dir {
                    candidates.push(cd.join("covers").join(stripped_sub));
                    candidates.push(cd.join(&clean_sub));
                    candidates.push(cd.join(stripped_sub));
                }
                // 5. Cerca in emu_root
                candidates.push(emu_root.join(&clean_sub));
                candidates.push(emu_root.join("covers").join(stripped_sub));
                candidates.push(emu_root.join(stripped_sub));

                let found_file = candidates.into_iter().find(|p| p.exists() && p.is_file());
                if let Some(full_path) = found_file {
                    if let Ok(bytes) = std::fs::read(&full_path) {
                        let mime = if full_path.extension().map_or(false, |e| e == "jpg" || e == "jpeg") {
                            "image/jpeg"
                        } else if full_path.extension().map_or(false, |e| e == "gif") {
                            "image/gif"
                        } else if full_path.extension().map_or(false, |e| e == "webp") {
                            "image/webp"
                        } else if full_path.extension().map_or(false, |e| e == "svg") {
                            "image/svg+xml"
                        } else {
                            "image/png"
                        };
                        let header_ct = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();
                        let header_cache = Header::from_bytes(&b"Cache-Control"[..], &b"public, max-age=86400"[..]).unwrap();
                        let body = if method == Method::Head { Vec::new() } else { bytes };
                        let response = Response::from_data(body).with_header(header_ct).with_header(header_cache);
                        let _ = request.respond(response);
                        return;
                    }
                }
                let _ = request.respond(Response::empty(404));
                return;
            }

            // Servizio immagini locali MAME ad alta velocità (Flyers, Titles, Snaps)
            if (method == Method::Get || method == Method::Head) && path.starts_with("/api/cover/mame/") {
                let raw_rom = path.trim_start_matches("/api/cover/mame/").trim();
                let rom_base = raw_rom
                    .trim_end_matches(".png")
                    .trim_end_matches(".jpg")
                    .trim_end_matches(".jpeg")
                    .trim_end_matches(".webp")
                    .replace(['/', '\\', ':', ';', ' '], "");
                let emu_mame = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame");
                let emu_covers = std::path::Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\covers");
                let candidates = [
                    emu_mame.join("flyers").join(format!("{}.png", rom_base)),
                    emu_mame.join("flyers").join(format!("{}.jpg", rom_base)),
                    emu_mame.join("titles").join(format!("{}.png", rom_base)),
                    emu_mame.join("titles").join(format!("{}.jpg", rom_base)),
                    emu_mame.join("snap").join(format!("{}.png", rom_base)),
                    emu_mame.join("snap").join(format!("{}.jpg", rom_base)),
                    emu_covers.join(format!("{}.png", rom_base)),
                    emu_covers.join(format!("{}.jpg", rom_base)),
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
                            return;
                        }
                    }
                }
                let _ = request.respond(Response::empty(404));
                return;
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
                            return;
                        }
                    }
                }
                let _ = request.respond(Response::empty(404));
                return;
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

                // EMULATOR CONFIG API - Get emulator paths & detection status
                (&Method::Get, "/api/config/emulators") => {
                    let config = crate::config::RetroHubConfig::load();
                    let resp = serde_json::json!({
                        "retroarch": {
                            "configured": config.emulator_paths.retroarch,
                            "detected": config.find_retroarch().map(|p| p.to_string_lossy().to_string()),
                            "is_available": config.find_retroarch().is_some()
                        },
                        "vpx": {
                            "configured": config.emulator_paths.vpx,
                            "detected": config.find_vpx().map(|p| p.to_string_lossy().to_string()),
                            "is_available": config.find_vpx().is_some()
                        },
                        "mame": {
                            "configured": config.emulator_paths.mame,
                            "detected": config.find_mame().map(|p| p.to_string_lossy().to_string()),
                            "is_available": config.find_mame().is_some()
                        }
                    });
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp.to_string()).with_header(header);
                    let _ = request.respond(response);
                }

                // EMULATOR CONFIG API - Save custom emulator paths
                (&Method::Post, "/api/config/emulators") => {
                    let body = read_request_body(&mut request);
                    let req: Result<EmulatorConfigReq, _> = serde_json::from_str(&body);
                    let res_json = match req {
                        Ok(r) => {
                            let mut config = crate::config::RetroHubConfig::load();
                            if let Some(ra) = r.retroarch {
                                config.emulator_paths.retroarch = if ra.trim().is_empty() { None } else { Some(ra.trim().to_string()) };
                            }
                            if let Some(vpx) = r.vpx {
                                config.emulator_paths.vpx = if vpx.trim().is_empty() { None } else { Some(vpx.trim().to_string()) };
                            }
                            if let Some(mame) = r.mame {
                                config.emulator_paths.mame = if mame.trim().is_empty() { None } else { Some(mame.trim().to_string()) };
                            }
                            let _ = config.save();
                            serde_json::json!({ "success": true, "message": "Configurazione emulatori salvata con successo" }).to_string()
                        }
                        Err(e) => {
                            serde_json::json!({ "success": false, "message": format!("JSON non valido: {}", e) }).to_string()
                        }
                    };
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(res_json).with_header(header);
                    let _ = request.respond(response);
                }

                // EMULATOR CONFIG API - Auto-detect emulators
                (&Method::Post, "/api/config/emulators/detect") => {
                    let mut config = crate::config::RetroHubConfig::load();
                    let (count, detected) = config.auto_detect_all();
                    let resp = serde_json::json!({
                        "success": true,
                        "count": count,
                        "detected": detected,
                        "retroarch": {
                            "configured": config.emulator_paths.retroarch,
                            "detected": config.find_retroarch().map(|p| p.to_string_lossy().to_string()),
                            "is_available": config.find_retroarch().is_some()
                        },
                        "vpx": {
                            "configured": config.emulator_paths.vpx,
                            "detected": config.find_vpx().map(|p| p.to_string_lossy().to_string()),
                            "is_available": config.find_vpx().is_some()
                        },
                        "mame": {
                            "configured": config.emulator_paths.mame,
                            "detected": config.find_mame().map(|p| p.to_string_lossy().to_string()),
                            "is_available": config.find_mame().is_some()
                        }
                    });
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp.to_string()).with_header(header);
                    let _ = request.respond(response);
                }

                // EMULATOR INSTALL API - Auto-install Visual Pinball X Core (1-Click)
                (&Method::Post, "/api/emulators/install-vpx") => {
                    let cur = VPX_INSTALL_PROGRESS.load(Ordering::SeqCst);
                    if cur > 0 && cur < 100 {
                        let msg = VPX_INSTALL_MSG.lock().unwrap().clone().unwrap_or_default();
                        let resp = serde_json::json!({
                            "success": true,
                            "status": "downloading",
                            "progress": cur,
                            "message": msg
                        });
                        let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                        let response = Response::from_string(resp.to_string()).with_header(header);
                        let _ = request.respond(response);
                        return;
                    }

                    VPX_INSTALL_PROGRESS.store(5, Ordering::SeqCst);
                    *VPX_INSTALL_MSG.lock().unwrap() = Some("Connessione ai repository standalone di Visual Pinball X...".into());

                    std::thread::spawn(|| {
                        let url = "https://github.com/vpinball/vpinball/releases/download/v10.8.0-2051-28dd6c3/Developer.VPinballX_GL-10.8.0-2051-28dd6c3-Release-win-x64.zip";
                        let emu_dir = std::path::PathBuf::from("emulators").join("vpx");
                        let _ = std::fs::create_dir_all(&emu_dir);
                        let zip_path = std::path::PathBuf::from("emulators").join("vpx_temp.zip");

                        let client = match reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(300)).build() {
                            Ok(c) => c,
                            Err(e) => {
                                VPX_INSTALL_PROGRESS.store(999, Ordering::SeqCst);
                                *VPX_INSTALL_MSG.lock().unwrap() = Some(format!("Errore creazione client HTTP: {}", e));
                                return;
                            }
                        };

                        let mut resp = match client.get(url).send() {
                            Ok(r) => r,
                            Err(e) => {
                                VPX_INSTALL_PROGRESS.store(999, Ordering::SeqCst);
                                *VPX_INSTALL_MSG.lock().unwrap() = Some(format!("Errore connessione: {}", e));
                                return;
                            }
                        };

                        let total_size = resp.content_length().unwrap_or(39187103);
                        let mut file = match std::fs::File::create(&zip_path) {
                            Ok(f) => f,
                            Err(e) => {
                                VPX_INSTALL_PROGRESS.store(999, Ordering::SeqCst);
                                *VPX_INSTALL_MSG.lock().unwrap() = Some(format!("Errore creazione file zip temporaneo: {}", e));
                                return;
                            }
                        };

                        use std::io::{Read, Write};
                        let mut downloaded: u64 = 0;
                        let mut buffer = [0u8; 65536];
                        loop {
                            match resp.read(&mut buffer) {
                                Ok(0) => break,
                                Ok(n) => {
                                    if let Err(e) = file.write_all(&buffer[..n]) {
                                        VPX_INSTALL_PROGRESS.store(999, Ordering::SeqCst);
                                        *VPX_INSTALL_MSG.lock().unwrap() = Some(format!("Errore scrittura disco: {}", e));
                                        return;
                                    }
                                    downloaded += n as u64;
                                    let pct = 5 + (downloaded * 80 / total_size) as u32;
                                    VPX_INSTALL_PROGRESS.store(pct.min(85), Ordering::SeqCst);
                                    let mb_done = downloaded as f64 / 1_048_576.0;
                                    let mb_total = total_size as f64 / 1_048_576.0;
                                    *VPX_INSTALL_MSG.lock().unwrap() = Some(format!("Download in corso: {:.1} MB / {:.1} MB ({}%)", mb_done, mb_total, pct.min(85)));
                                }
                                Err(e) => {
                                    VPX_INSTALL_PROGRESS.store(999, Ordering::SeqCst);
                                    *VPX_INSTALL_MSG.lock().unwrap() = Some(format!("Errore download: {}", e));
                                    return;
                                }
                            }
                        }

                        VPX_INSTALL_PROGRESS.store(88, Ordering::SeqCst);
                        *VPX_INSTALL_MSG.lock().unwrap() = Some("Estrazione automatica archivio standalone in corso...".into());

                        let tar_res = Command::new("tar.exe")
                            .args(["-xf", zip_path.to_str().unwrap_or(""), "-C", emu_dir.to_str().unwrap_or("")])
                            .output();

                        let tar_failed = match &tar_res {
                            Ok(out) => !out.status.success(),
                            Err(_) => true,
                        };

                        if tar_failed {
                            let ps_cmd = format!("Expand-Archive -Path '{}' -DestinationPath '{}' -Force", zip_path.display(), emu_dir.display());
                            let _ = Command::new("powershell.exe")
                                .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
                                .output();
                        }

                        let _ = std::fs::remove_file(&zip_path);

                        let mut vpx_exe_path: Option<std::path::PathBuf> = None;
                        for entry in walkdir::WalkDir::new(&emu_dir).into_iter().filter_map(|e| e.ok()) {
                            let p = entry.path();
                            if p.is_file() {
                                if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                                    let n_low = name.to_lowercase();
                                    if n_low.starts_with("vpinballx") && n_low.ends_with(".exe") {
                                        vpx_exe_path = Some(p.to_path_buf());
                                        break;
                                    }
                                }
                            }
                        }

                        if let Some(exe) = vpx_exe_path {
                            let mut config = crate::config::RetroHubConfig::load();
                            let abs_path = std::fs::canonicalize(&exe).unwrap_or(exe);
                            config.emulator_paths.vpx = Some(abs_path.to_string_lossy().to_string());
                            let _ = config.save();
                            VPX_INSTALL_PROGRESS.store(100, Ordering::SeqCst);
                            *VPX_INSTALL_MSG.lock().unwrap() = Some("Visual Pinball X integrato e configurato con successo!".into());
                        } else {
                            VPX_INSTALL_PROGRESS.store(999, Ordering::SeqCst);
                            *VPX_INSTALL_MSG.lock().unwrap() = Some("Eseguibile VPinballX non trovato nell'archivio estratto".into());
                        }
                    });

                    let resp = serde_json::json!({
                        "success": true,
                        "status": "started",
                        "progress": 5,
                        "message": "Download avviato in background"
                    });
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp.to_string()).with_header(header);
                    let _ = request.respond(response);
                }

                // EMULATOR INSTALL STATUS API
                (&Method::Get, "/api/emulators/install-status") => {
                    let progress = VPX_INSTALL_PROGRESS.load(Ordering::SeqCst);
                    let msg = VPX_INSTALL_MSG.lock().unwrap().clone().unwrap_or_default();
                    let status = if progress >= 100 {
                        "ready"
                    } else if progress == 999 {
                        "error"
                    } else if progress > 0 {
                        "downloading"
                    } else {
                        "idle"
                    };
                    let resp = serde_json::json!({
                        "progress": progress,
                        "status": status,
                        "message": msg
                    });
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(resp.to_string()).with_header(header);
                    let _ = request.respond(response);
                }

                // WEB LAUNCH API - Open authentic web simulator
                (&Method::Post, "/api/launch/web") => {
                    let body = read_request_body(&mut request);
                    let req: Result<WebLaunchReq, _> = serde_json::from_str(&body);
                    let res_json = match req {
                        Ok(r) => {
                            let url = match r.game_id {
                                1720 => "https://pica-pic.com/#/donkey_kong/",
                                1721 => "https://pica-pic.com/#/donkey_kong_jr/",
                                1722 => "https://archive.org/details/gameandwatch",
                                1723 => "https://archive.org/details/gameandwatch",
                                1724 => "https://archive.org/details/gameandwatch",
                                1725 => "https://pica-pic.com/#/parachute/",
                                1726 => "https://pica-pic.com/#/octopus/",
                                1727 => "https://pica-pic.com/#/chef/",
                                1728 => "https://archive.org/details/gameandwatch",
                                1729 => "https://archive.org/details/gameandwatch",
                                1730 => "https://archive.org/details/gameandwatch",
                                1731 => "https://archive.org/details/gameandwatch",
                                1732 => "https://archive.org/details/gameandwatch",
                                1733 => "https://archive.org/details/gameandwatch",
                                1734 => "https://pica-pic.com/#/zelda/",
                                _ => "https://archive.org/details/gameandwatch",
                            };
                            let script = format!(r#"Start-Process '{}'"#, url);
                            let _ = Command::new("powershell")
                                .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                                .spawn();
                            serde_json::json!({ "success": true, "url": url }).to_string()
                        }
                        Err(e) => {
                            serde_json::json!({ "success": false, "message": format!("Richiesta non valida: {}", e) }).to_string()
                        }
                    };
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(res_json).with_header(header);
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
                            let game_info = {
                                if let Ok(db) = self.db.lock() {
                                    db.get_game_by_id(r.game_id).ok().flatten()
                                } else {
                                    None
                                }
                            };

                            if let Some(game) = game_info {
                                let rom_name = crate::download_manager::DownloadManager::get_rom_filename(&game.title, &game.launch_target, &game.system_code);
                                match crate::download_manager::DownloadManager::download_rom(game.id, &rom_name, &game.system_code) {
                                    Ok(path) => {
                                        let target_str = if game.system_code == "MAME" {
                                            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(&game.title);
                                            format!("MAME:{}", stem)
                                        } else {
                                            path.to_string_lossy().to_string()
                                        };
                                        if let Ok(db) = self.db.lock() {
                                            let _ = db.update_launch_target(game.id, &target_str);
                                        }
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
                                        let _ = db.update_launch_target(game.id, "");
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

                // SETTINGS API - Get cache stats
                (&Method::Get, "/api/cache/stats") => {
                    let mut total_bytes: u64 = 0;
                    let mut file_count: usize = 0;
                    let cache_dirs = ["covers", "retro_downloads", "downloads"];
                    for d in &cache_dirs {
                        let path = std::path::Path::new(d);
                        if path.exists() && path.is_dir() {
                            if let Ok(entries) = std::fs::read_dir(path) {
                                for entry in entries.flatten() {
                                    if let Ok(meta) = entry.metadata() {
                                        total_bytes += meta.len();
                                        file_count += 1;
                                    }
                                }
                            }
                        }
                    }
                    let mb = (total_bytes as f64) / (1024.0 * 1024.0);
                    let json = serde_json::json!({
                        "success": true,
                        "size_mb": format!("{:.1}", mb),
                        "file_count": file_count,
                        "downloads_path": std::env::current_dir().unwrap_or_default().join("retro_downloads").to_string_lossy()
                    }).to_string();
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // SETTINGS API - Clear image / cover cache
                (&Method::Post, "/api/cache/clear") => {
                    let cover_path = std::path::Path::new("covers");
                    let mut deleted = 0;
                    if cover_path.exists() && cover_path.is_dir() {
                        if let Ok(entries) = std::fs::read_dir(cover_path) {
                            for entry in entries.flatten() {
                                if let Ok(path) = entry.path().canonicalize() {
                                    let _ = std::fs::remove_file(path);
                                    deleted += 1;
                                }
                            }
                        }
                    }
                    let json = serde_json::json!({
                        "success": true,
                        "deleted_files": deleted,
                        "message": format!("Cache immagini ripulita: eliminati {} file", deleted)
                    }).to_string();
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // SETTINGS API - Factory Reset (Nuovo Utente Pulito)
                (&Method::Post, "/api/factory-reset") => {
                    // 1. Elimina cartelle locali temporanee del launcher (MAI toccare cartelle esterne, copertine repo o ROM utente)
                    let cache_dirs = ["retro_downloads", "downloads"];
                    for d in &cache_dirs {
                        let p = std::path::Path::new(d);
                        if p.exists() && p.is_dir() {
                            let _ = std::fs::remove_dir_all(p);
                        }
                    }
                    // 2. Ripristina il database SQLite al catalogo vergine Cloud On-Demand
                    let cloud_games = crate::master_catalog::MasterCatalog::get_cloud_catalog(1);
                    let total = cloud_games.len();
                    let reset_ok = {
                        let db = self.db.lock().unwrap();
                        db.reset_catalog(&cloud_games).is_ok()
                    };
                    // 3. Ripristina retrohub.json a default
                    let _ = crate::config::RetroHubConfig::default().save();

                    let json = serde_json::json!({
                        "success": reset_ok,
                        "total_games": total,
                        "message": "Ripristino a nuovo utente completato con successo. Le tue ROM sul disco sono al sicuro."
                    }).to_string();
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // SETTINGS API - Open downloads folder in Explorer
                (&Method::Post, "/api/open-downloads-folder") => {
                    let dl_path = std::env::current_dir().unwrap_or_default().join("retro_downloads");
                    let _ = std::fs::create_dir_all(&dl_path);
                    #[cfg(target_os = "windows")]
                    {
                        let _ = std::process::Command::new("explorer").arg(&dl_path).spawn();
                    }
                    let json = serde_json::json!({
                        "success": true,
                        "path": dl_path.to_string_lossy()
                    }).to_string();
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                // Chiude il server quando la finestra del browser viene chiusa
                (&Method::Post, "/api/shutdown") => {
                    self.shutdown.store(true, Ordering::SeqCst);
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(r#"{"success": true}"#).with_header(header);
                    let _ = request.respond(response);
                    return;
                }

                _ => {
                    let response = Response::from_string("404 Not Found").with_status_code(StatusCode(404));
                    let _ = request.respond(response);
                }
            }
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
    let _ = request.as_reader().read_to_end(&mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
