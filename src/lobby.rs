use crate::models::Game;
use crate::netplay::NetplaySession;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct LobbyRoom {
    pub id: String,
    pub game_id: i64,
    pub game_title: String,
    pub platform: String,
    pub host_name: String,
    pub host_addr: String,
    pub port: u16,
    pub player_count: u8,
    pub max_players: u8,
    pub status: LobbyStatus,
    pub ping_ms: u64,
    pub created_at: u64,
    pub region: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LobbyStatus {
    Waiting,
    InGame,
    Closed,
}

pub struct LobbyManager {
    rooms: Arc<Mutex<HashMap<String, LobbyRoom>>>,
    heartbeat_interval: Duration,
}

impl LobbyManager {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(Mutex::new(HashMap::new())),
            heartbeat_interval: Duration::from_secs(30),
        }
    }

    pub fn create_room(
        &self,
        game_id: i64,
        game_title: String,
        platform: String,
        host_name: String,
        host_addr: String,
        port: u16,
        ping_ms: u64,
    ) -> String {
        let room_id = Self::generate_room_id();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let room = LobbyRoom {
            id: room_id.clone(),
            game_id,
            game_title,
            platform,
            host_name,
            host_addr,
            port,
            player_count: 1,
            max_players: 2,
            status: LobbyStatus::Waiting,
            ping_ms,
            created_at: now,
            region: "auto".to_string(),
        };

        self.rooms.lock().unwrap().insert(room_id.clone(), room);
        room_id
    }

    fn generate_room_id() -> String {
        use rand::Rng;
        let id: u16 = rand::thread_rng().gen_range(1000..9999);
        format!("RH-{:04}", id)
    }

    pub fn join_room(&self, room_id: &str, guest_name: String) -> Result<(LobbyRoom, String), String> {
        let mut rooms = self.rooms.lock().unwrap();
        
        if let Some(room) = rooms.get_mut(room_id) {
            if room.status != LobbyStatus::Waiting {
                return Err("Stanza chiusa o già in gioco".to_string());
            }
            if room.player_count >= room.max_players {
                return Err("Stanza piena".to_string());
            }
            
            room.player_count += 1;
            let connection_token = format!("RH{}", hex_encode(format!("{}:{}:0:{}", room.host_addr, room.port, room_id).as_bytes()));
            let session = NetplaySession {
                room_code: room_id.to_string(),
                connection_token: connection_token.clone(),
                game_id: room.game_id,
                game_title: room.game_title.clone(),
                platform: room.platform.clone(),
                is_host: false,
                player_name: guest_name,
                peer_name: Some(room.host_name.clone()),
                peer_addr: Some(room.host_addr.clone()),
                status: crate::netplay::RoomStatus::Connected,
                ping_ms: Some(room.ping_ms),
                local_netplay_port: room.port,
                public_ip: None,
                nat_traversal: true,
            };
            
            // Store session for retrieval
            let rooms_clone = self.rooms.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(300));
                let _r = rooms_clone.lock().unwrap();
            });
            
            Ok((room.clone(), connection_token))
        } else {
            Err("Stanza non trovata".to_string())
        }
    }

    pub fn list_rooms(&self) -> Vec<LobbyRoom> {
        let rooms = self.rooms.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        rooms
            .iter()
            .filter(|(_, r)| r.status == LobbyStatus::Waiting && (now - r.created_at) < 1800)
            .map(|(_, r)| r.clone())
            .collect()
    }

    pub fn close_room(&self, room_id: &str) {
        if let Some(mut room) = self.rooms.lock().unwrap().remove(room_id) {
            room.status = LobbyStatus::Closed;
        }
    }

    pub fn cleanup_expired(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let rooms = self.rooms.lock().unwrap();
        for (id, room) in rooms.iter() {
            if (now - room.created_at) > 1800 {
                drop(rooms.clone());
                self.rooms.lock().unwrap().remove(id);
            }
        }
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}