use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub title: String,
    pub platform: String,
    pub system_code: String,
    pub year: u16,
    pub genre: String,
    pub developer: String,
    pub publisher: String,
    pub controls: String,
    pub launch_target: String,
    pub cover_url: Option<String>,
    pub is_favorite: bool,
    pub play_count: u32,
    pub last_played_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub code: String,
    pub name: String,
    pub count: usize,
    pub badge_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreInfo {
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecadeInfo {
    pub decade: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilter {
    pub query: Option<String>,
    pub platform: Option<String>,
    pub genre: Option<String>,
    pub decade: Option<String>,
    pub favorites_only: bool,
    pub sort_by: Option<String>, // "title", "year", "play_count", "platform"
    pub sort_asc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchRequest {
    pub game_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchResponse {
    pub success: bool,
    pub message: String,
    pub play_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsSummary {
    pub total_games: usize,
    pub total_platforms: usize,
    pub total_favorites: usize,
    pub total_plays: u32,
    pub most_played: Vec<Game>,
    pub recently_played: Vec<Game>,
}
