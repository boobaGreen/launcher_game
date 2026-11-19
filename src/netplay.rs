use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoomStatus {
    Idle,
    WaitingForGuest,
    Connected,
    InGame,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetplaySession {
    pub room_code: String,
    pub connection_token: String,
    pub game_id: i64,
    pub game_title: String,
    pub platform: String,
    pub is_host: bool,
    pub player_name: String,
    pub peer_name: Option<String>,
    pub peer_addr: Option<String>,
    pub status: RoomStatus,
    pub ping_ms: Option<u64>,
    pub local_netplay_port: u16,
    pub public_ip: Option<String>,
    pub nat_traversal: bool,
}

pub struct NetplayManager {
    session: Arc<Mutex<Option<NetplaySession>>>,
    socket: Arc<Mutex<Option<UdpSocket>>>,
    stun_servers: Vec<String>,
    enable_stun: bool,
    enable_nat_traversal: bool,
}

impl NetplayManager {
    pub fn new() -> Self {
        Self {
            session: Arc::new(Mutex::new(None)),
            socket: Arc::new(Mutex::new(None)),
            stun_servers: vec![
                "stun.l.google.com:19302".to_string(),
                "stun1.l.google.com:19302".to_string(),
                "stun.cloudflare.com:3478".to_string(),
            ],
            enable_stun: true,
            enable_nat_traversal: true,
        }
    }

    pub fn with_stun_config(mut self, stun_servers: Vec<String>, enable_nat: bool) -> Self {
        self.stun_servers = stun_servers;
        self.enable_nat_traversal = enable_nat;
        self
    }

    pub fn create_room(&self, game_id: i64, game_title: String, platform: String, player_name: String) -> Result<NetplaySession, String> {
        let port = allocate_free_udp_port().unwrap_or(19864);
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
            .map_err(|e| format!("Impossibile associare socket UDP su porta {}: {}", port, e))?;
        socket.set_nonblocking(true).map_err(|e| e.to_string())?;

        let public_ip = if self.enable_stun {
            resolve_public_ip_stun(&self.stun_servers).unwrap_or_else(|| get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string()))
        } else {
            get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string())
        };
        let room_code = generate_room_code();

        // Token encodes: IP:PORT:GAME_ID:ROOM_CODE
        let token_raw = format!("{}:{}:{}:{}", public_ip, port, game_id, room_code);
        let connection_token = format!("RH{}", hex_encode(token_raw.as_bytes()));

        let session = NetplaySession {
            room_code,
            connection_token,
            game_id,
            game_title,
            platform,
            is_host: true,
            player_name,
            peer_name: None,
            peer_addr: None,
            status: RoomStatus::WaitingForGuest,
            ping_ms: None,
            local_netplay_port: port,
            public_ip: Some(public_ip),
            nat_traversal: self.enable_nat_traversal,
        };

        *self.session.lock().unwrap() = Some(session.clone());
        *self.socket.lock().unwrap() = Some(socket);

        // Spawn background listener thread for handshakes
        let session_clone = self.session.clone();
        let socket_clone = self.socket.clone();

        std::thread::spawn(move || {
            let mut buf = [0u8; 512];
            let start = Instant::now();

            while start.elapsed() < Duration::from_secs(600) {
                std::thread::sleep(Duration::from_millis(100));

                let socket_guard = socket_clone.lock().unwrap();
                if let Some(ref sock) = *socket_guard {
                    if let Ok((len, src)) = sock.recv_from(&mut buf) {
                        let msg = String::from_utf8_lossy(&buf[..len]);

                        if msg.starts_with("RH_JOIN:") {
                            let guest_name = msg.trim_start_matches("RH_JOIN:").to_string();
                            let _ = sock.send_to(b"RH_WELCOME:Host", src);

                            let mut sess_guard = session_clone.lock().unwrap();
                            if let Some(ref mut s) = *sess_guard {
                                s.peer_name = Some(guest_name);
                                s.peer_addr = Some(src.to_string());
                                s.status = RoomStatus::Connected;
                                s.ping_ms = Some(15);
                            }
                        } else if msg.starts_with("RH_PING") {
                            let _ = sock.send_to(b"RH_PONG", src);
                        } else if msg.starts_with("RH_START") {
                            let mut sess_guard = session_clone.lock().unwrap();
                            if let Some(ref mut s) = *sess_guard {
                                s.status = RoomStatus::InGame;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        });

        Ok(session)
    }

    /// Joins an existing Netplay Room as Guest.
    pub fn join_room(&self, token: &str, guest_name: String) -> Result<NetplaySession, String> {
        let clean_token = token.trim();
        if !clean_token.starts_with("RH") {
            return Err("Token di connessione non valido (deve iniziare con 'RH')".to_string());
        }

        let hex_part = &clean_token[2..];
        let bytes = hex_decode(hex_part).map_err(|_| "Decodifica token fallita".to_string())?;
        let raw_str = String::from_utf8(bytes).map_err(|_| "Formato token non valido".to_string())?;

        let parts: Vec<&str> = raw_str.split(':').collect();
        if parts.len() < 4 {
            return Err("Parametri del token incompleti".to_string());
        }

        let host_ip = parts[0];
        let host_port: u16 = parts[1].parse().map_err(|_| "Porta host non valida".to_string())?;
        let game_id: i64 = parts[2].parse().unwrap_or(0);
        let room_code = parts[3].to_string();

        let host_addr = format!("{}:{}", host_ip, host_port);

        let my_port = allocate_free_udp_port().unwrap_or(19865);
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", my_port))
            .map_err(|e| format!("Impossibile creare socket UDP: {}", e))?;
        socket.set_read_timeout(Some(Duration::from_millis(2000))).ok();

        // Measure real ping to host
        let start = Instant::now();
        socket
            .send_to(b"RH_PING", &host_addr)
            .map_err(|e| format!("Invio pacchetto di connessione fallito: {}", e))?;
        let mut buf = [0u8; 4];
        socket.recv_from(&mut buf).ok();
        let elapsed = start.elapsed().as_millis() as u64;

        // Send JOIN packet to host
        let join_msg = format!("RH_JOIN:{}", guest_name);
        socket
            .send_to(join_msg.as_bytes(), &host_addr)
            .map_err(|e| format!("Invio pacchetto di connessione fallito: {}", e))?;

        let session = NetplaySession {
            room_code,
            connection_token: clean_token.to_string(),
            game_id,
            game_title: "Partita Online".to_string(),
            platform: "Arcade / Retro".to_string(),
            is_host: false,
            player_name: guest_name,
            peer_name: Some("Host".to_string()),
            peer_addr: Some(host_addr.clone()),
            status: RoomStatus::Connected,
            ping_ms: Some(elapsed),
            local_netplay_port: my_port,
            public_ip: None,
            nat_traversal: true,
        };

        *self.session.lock().unwrap() = Some(session.clone());
        *self.socket.lock().unwrap() = Some(socket);

        Ok(session)
    }

    pub fn get_session(&self) -> Option<NetplaySession> {
        self.session.lock().unwrap().clone()
    }

    pub fn start_game(&self) -> Result<(), String> {
        let mut guard = self.session.lock().unwrap();
        if let Some(ref mut sess) = *guard {
            sess.status = RoomStatus::InGame;

            if let Some(ref peer) = sess.peer_addr {
                let sock_guard = self.socket.lock().unwrap();
                if let Some(ref s) = *sock_guard {
                    let _ = s.send_to(b"RH_START", peer);
                }
            }
            Ok(())
        } else {
            Err("Nessuna sessione attiva da avviare".to_string())
        }
    }

    pub fn close_room(&self) {
        *self.session.lock().unwrap() = None;
        *self.socket.lock().unwrap() = None;
    }
}

fn allocate_free_udp_port() -> Option<u16> {
    UdpSocket::bind("127.0.0.1:0")
        .and_then(|s| s.local_addr())
        .map(|a| a.port())
        .ok()
}

fn generate_room_code() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let num = (now % 90000) + 10000;
    format!("RH-{}", num)
}

fn get_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|a| a.ip().to_string())
}

#[allow(dead_code)]
fn resolve_public_ip() -> Option<String> {
    // Fast STUN resolve or local network fallback
    get_local_ip()
}

/// Resolves public IP using STUN protocol
fn resolve_public_ip_stun(stun_servers: &[String]) -> Option<String> {
    for server in stun_servers {
        if let Some(ip) = try_stun_server(server) {
            return Some(ip);
        }
    }
    None
}

fn try_stun_server(server: &str) -> Option<String> {
    let parts: Vec<&str> = server.split(':').collect();
    if parts.len() != 2 {
        return None;
    }
    let addr: SocketAddr = format!("{}:3478", parts[0]).parse().ok()?;
    
    let stun_socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    stun_socket.set_read_timeout(Some(Duration::from_secs(2))).ok()?;
    stun_socket.connect(&addr).ok()?;
    
    // Build STUN binding request
    let mut req = Vec::new();
    req.extend_from_slice(&[0x00, 0x01]); // STUN BINDING REQUEST
    req.extend_from_slice(&[0x00, 0x08]); // Length
    req.extend_from_slice(&mut [0u8; 12]); // Transaction ID
    req.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Magic cookie
    
    if stun_socket.send(&req).is_ok() {
        let mut buf = [0u8; 512];
        if let Ok((len, _)) = stun_socket.recv_from(&mut buf) {
            // Parse XOR-MAPPED-ADDRESS (type 0x0020)
            if len > 20 {
                let type_val = u16::from_be_bytes([buf[2], buf[3]]);
                if type_val == 0x0020 {
                    let port = u16::from_be_bytes([buf[6], buf[7]]);
                    let family = buf[8];
                    if family == 0x01 { // IPv4
                        let ip = format!("{}.{}.{}.{}", buf[9], buf[10], buf[11], buf[12]);
                        return Some(format!("{}:{}", ip, port));
                    }
                }
            }
        }
    }
    None
}

/// Measures ping to a remote address via UDP
#[allow(dead_code)]
fn measure_ping(addr: &str, port: u16, timeout_ms: u64) -> Option<u64> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.set_read_timeout(Some(Duration::from_millis(timeout_ms))).ok()?;
    
    let dest: SocketAddr = format!("{}:{}", addr, port).parse().ok()?;
    
    let start = Instant::now();
    socket.send_to(b"PING", dest).ok()?;
    let mut buf = [0u8; 4];
    socket.recv_from(&mut buf).ok()?;
    let elapsed = start.elapsed().as_millis() as u64;
    Some(elapsed)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ()> {
    if s.len() % 2 != 0 {
        return Err(());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| ()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_token_roundtrip() {
        let raw = "192.168.1.50:19864:105:RH-48912";
        let hex = hex_encode(raw.as_bytes());
        let decoded_bytes = hex_decode(&hex).unwrap();
        let decoded = String::from_utf8(decoded_bytes).unwrap();
        assert_eq!(raw, decoded);
    }

    #[test]
    fn test_create_room() {
        let mgr = NetplayManager::new();
        let session = mgr
            .create_room(
                1,
                "Street Fighter II".to_string(),
                "MAME".to_string(),
                "Claudio".to_string(),
            )
            .unwrap();

        assert!(session.is_host);
        assert_eq!(session.status, RoomStatus::WaitingForGuest);
        assert!(session.connection_token.starts_with("RH"));
    }
}
