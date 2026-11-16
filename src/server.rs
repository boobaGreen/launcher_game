use crate::db::Database;
use crate::launcher::Launcher;
use crate::models::{LaunchRequest, LaunchResponse, SearchFilter};
use crate::netplay::NetplayManager;
use crate::search::SearchEngine;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub struct HttpServer {
    db: Arc<Mutex<Database>>,
    netplay: Arc<NetplayManager>,
    port: u16,
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
            port,
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let addr = format!("127.0.0.1:{}", self.port);
        let server = Server::http(&addr).map_err(|e| format!("Impossibile avviare il server HTTP su {}: {}", addr, e))?;

        println!("🚀 RetroHub UI Server in ascolto su: http://{}", addr);

        for mut request in server.incoming_requests() {
            let url = request.url().to_string();
            let method = request.method().clone();

            let (path, query_params) = parse_url(&url);

            match (&method, path.as_str()) {
                (&Method::Get, "/" | "/index.html") => {
                    let html = include_str!("ui.html");
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
                    let response = Response::from_string(html).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Get, "/api/games") => {
                    let filter = SearchFilter {
                        query: query_params.get("query").cloned(),
                        platform: query_params.get("platform").cloned(),
                        genre: query_params.get("genre").cloned(),
                        decade: query_params.get("decade").cloned(),
                        favorites_only: query_params.get("favorites_only").map_or(false, |v| v == "true"),
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

                (&Method::Get, "/api/stats") => {
                    let db = self.db.lock().unwrap();
                    let stats = db.get_stats().ok();
                    let json = serde_json::to_string(&stats).unwrap_or_else(|_| "{}".to_string());
                    let header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
                    let response = Response::from_string(json).with_header(header);
                    let _ = request.respond(response);
                }

                (&Method::Post, "/api/launch") => {
                    let mut body = String::new();
                    let _ = request.as_reader().read_to_string(&mut body);
                    let req: Result<LaunchRequest, _> = serde_json::from_str(&body);

                    let res = match req {
                        Ok(r) => {
                            let db = self.db.lock().unwrap();
                            if let Ok(Some(game)) = db.get_game_by_id(r.game_id) {
                                match Launcher::launch(&game.launch_target) {
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
                    let mut body = String::new();
                    let _ = request.as_reader().read_to_string(&mut body);
                    let req: Result<LaunchRequest, _> = serde_json::from_str(&body);

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
                    let mut body = String::new();
                    let _ = request.as_reader().read_to_string(&mut body);
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
                    let mut body = String::new();
                    let _ = request.as_reader().read_to_string(&mut body);
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
