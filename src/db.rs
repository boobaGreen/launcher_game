use crate::models::{DecadeInfo, DeveloperInfo, Game, GenreInfo, PlatformInfo, StatsSummary};
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS games (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                platform TEXT NOT NULL,
                system_code TEXT NOT NULL,
                year INTEGER NOT NULL,
                genre TEXT NOT NULL,
                developer TEXT NOT NULL,
                publisher TEXT NOT NULL,
                controls TEXT NOT NULL,
                launch_target TEXT NOT NULL,
                cover_url TEXT,
                is_favorite INTEGER NOT NULL DEFAULT 0,
                play_count INTEGER NOT NULL DEFAULT 0,
                last_played_at TEXT
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_games_sys ON games(system_code)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_games_title ON games(title)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_games_favorite ON games(is_favorite)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_games_playcount ON games(play_count DESC)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_games_lastplayed ON games(last_played_at DESC)",
            [],
        )?;

        Ok(())
    }

    pub fn upsert_game(&self, g: &Game) -> Result<()> {
        self.conn.execute(
            "INSERT INTO games (id, title, platform, system_code, year, genre, developer, publisher, controls, launch_target, cover_url, is_favorite, play_count, last_played_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
             ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                platform = excluded.platform,
                system_code = excluded.system_code,
                year = excluded.year,
                genre = excluded.genre,
                developer = excluded.developer,
                publisher = excluded.publisher,
                controls = excluded.controls,
                launch_target = excluded.launch_target,
                cover_url = COALESCE(excluded.cover_url, games.cover_url)",
            params![
                g.id,
                g.title,
                g.platform,
                g.system_code,
                g.year,
                g.genre,
                g.developer,
                g.publisher,
                g.controls,
                g.launch_target,
                g.cover_url,
                if g.is_favorite { 1 } else { 0 },
                g.play_count,
                g.last_played_at
            ],
        )?;
        Ok(())
    }

    pub fn upsert_games_batch(&self, games: &[Game]) -> Result<()> {
        self.conn.execute_batch("BEGIN TRANSACTION;")?;
        {
            let mut stmt = self.conn.prepare(
                "INSERT INTO games (id, title, platform, system_code, year, genre, developer, publisher, controls, launch_target, cover_url, is_favorite, play_count, last_played_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
                 ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    platform = excluded.platform,
                    system_code = excluded.system_code,
                    year = excluded.year,
                    genre = excluded.genre,
                    developer = excluded.developer,
                    publisher = excluded.publisher,
                    controls = excluded.controls,
                    launch_target = excluded.launch_target,
                    cover_url = COALESCE(excluded.cover_url, games.cover_url)"
            )?;

            for g in games {
                let _ = stmt.execute(params![
                    g.id,
                    g.title,
                    g.platform,
                    g.system_code,
                    g.year,
                    g.genre,
                    g.developer,
                    g.publisher,
                    g.controls,
                    g.launch_target,
                    g.cover_url,
                    if g.is_favorite { 1 } else { 0 },
                    g.play_count,
                    g.last_played_at
                ]);
            }
        }
        self.conn.execute_batch("COMMIT;")?;
        Ok(())
    }

    pub fn get_all_games(&self) -> Result<Vec<Game>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, platform, system_code, year, genre, developer, publisher, controls, launch_target, cover_url, is_favorite, play_count, last_played_at
             FROM games ORDER BY title ASC",
        )?;

        let games = stmt
            .query_map([], |row| {
                let fav_num: i32 = row.get(11)?;
                Ok(Game {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    platform: row.get(2)?,
                    system_code: row.get(3)?,
                    year: row.get(4)?,
                    genre: row.get(5)?,
                    developer: row.get(6)?,
                    publisher: row.get(7)?,
                    controls: row.get(8)?,
                    launch_target: row.get(9)?,
                    cover_url: row.get(10)?,
                    is_favorite: fav_num == 1,
                    play_count: row.get(12)?,
                    last_played_at: row.get(13)?,
                })
            })?
            .collect::<Result<Vec<Game>>>()?;

        Ok(games)
    }

    pub fn get_game_by_id(&self, id: i64) -> Result<Option<Game>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, platform, system_code, year, genre, developer, publisher, controls, launch_target, cover_url, is_favorite, play_count, last_played_at
             FROM games WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let fav_num: i32 = row.get(11)?;
            Ok(Some(Game {
                id: row.get(0)?,
                title: row.get(1)?,
                platform: row.get(2)?,
                system_code: row.get(3)?,
                year: row.get(4)?,
                genre: row.get(5)?,
                developer: row.get(6)?,
                publisher: row.get(7)?,
                controls: row.get(8)?,
                launch_target: row.get(9)?,
                cover_url: row.get(10)?,
                is_favorite: fav_num == 1,
                play_count: row.get(12)?,
                last_played_at: row.get(13)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn toggle_favorite(&self, id: i64) -> Result<bool> {
        let current_fav: i32 = self
            .conn
            .query_row(
                "SELECT is_favorite FROM games WHERE id = ?1",
                params![id],
                |r| r.get(0),
            )
            .unwrap_or(0);

        let new_fav = if current_fav == 1 { 0 } else { 1 };
        self.conn.execute(
            "UPDATE games SET is_favorite = ?1 WHERE id = ?2",
            params![new_fav, id],
        )?;

        Ok(new_fav == 1)
    }

    pub fn record_play(&self, id: i64) -> Result<u32> {
        let now = chrono_free_now();
        self.conn.execute(
            "UPDATE games SET play_count = play_count + 1, last_played_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        let new_count: u32 = self
            .conn
            .query_row(
                "SELECT play_count FROM games WHERE id = ?1",
                params![id],
                |r| r.get(0),
            )
            .unwrap_or(0);

        Ok(new_count)
    }

    pub fn get_platforms(&self) -> Result<Vec<PlatformInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT system_code, platform, COUNT(*) as cnt FROM games GROUP BY system_code ORDER BY cnt DESC",
        )?;

        let res = stmt
            .query_map([], |row| {
                let code: String = row.get(0)?;
                let name: String = row.get(1)?;
                let count: usize = row.get(2)?;
                let badge_color = get_badge_color(&code).to_string();
                Ok(PlatformInfo {
                    code,
                    name,
                    count,
                    badge_color,
                })
            })?
            .collect::<Result<Vec<PlatformInfo>>>()?;

        Ok(res)
    }

    pub fn get_genres(&self) -> Result<Vec<GenreInfo>> {
        let games = self.get_all_games()?;
        let mut counts = HashMap::new();

        for g in games {
            for part in g.genre.split(&['/', ','][..]) {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    *counts.entry(trimmed.to_string()).or_insert(0) += 1;
                }
            }
        }

        let mut list: Vec<GenreInfo> = counts
            .into_iter()
            .map(|(name, count)| GenreInfo { name, count })
            .collect();

        list.sort_by(|a, b| b.count.cmp(&a.count));
        Ok(list)
    }

    pub fn get_developers(&self) -> Result<Vec<DeveloperInfo>> {
        let games = self.get_all_games()?;
        let mut counts: HashMap<String, usize> = HashMap::new();

        for g in games {
            let dev = g.developer.trim();
            if !dev.is_empty() && dev != "Unknown" && dev != "N/A" {
                *counts.entry(dev.to_string()).or_insert(0) += 1;
            }
        }

        let mut list: Vec<DeveloperInfo> = counts
            .into_iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(name, count)| DeveloperInfo { name, count })
            .collect();

        list.sort_by(|a, b| b.count.cmp(&a.count));
        Ok(list)
    }

    pub fn get_decades(&self) -> Result<Vec<DecadeInfo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT year, COUNT(*) FROM games GROUP BY year")?;

        let mut decade_counts: HashMap<&str, usize> = HashMap::new();
        decade_counts.insert("1980s", 0);
        decade_counts.insert("1990s", 0);
        decade_counts.insert("2000s", 0);
        decade_counts.insert("2010s", 0);
        decade_counts.insert("2020s", 0);

        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let year: u16 = row.get(0)?;
            let cnt: usize = row.get(1)?;
            let key = match year {
                1980..=1989 => "1980s",
                1990..=1999 => "1990s",
                2000..=2009 => "2000s",
                2010..=2019 => "2010s",
                _ if year >= 2020 => "2020s",
                _ => "Other",
            };
            if let Some(val) = decade_counts.get_mut(key) {
                *val += cnt;
            }
        }

        let order = ["1980s", "1990s", "2000s", "2010s", "2020s"];
        let list = order
            .iter()
            .map(|&d| DecadeInfo {
                decade: d.to_string(),
                count: *decade_counts.get(d).unwrap_or(&0),
            })
            .collect();

        Ok(list)
    }

    pub fn get_stats(&self) -> Result<StatsSummary> {
        // Ottimizzazione: query unica con tutti i dati aggregati
        let (total_games, total_platforms, total_favorites, total_plays) = self.conn.query_row(
            "SELECT 
                COUNT(*) as total,
                COUNT(DISTINCT system_code) as platforms,
                COUNT(CASE WHEN is_favorite = 1 THEN 1 END) as favorites,
                COALESCE(SUM(play_count), 0) as plays
             FROM games",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )?;

        // Most played
        let mut stmt = self.conn.prepare(
            "SELECT id, title, platform, system_code, year, genre, developer, publisher, controls, launch_target, cover_url, is_favorite, play_count, last_played_at
             FROM games WHERE play_count > 0 ORDER BY play_count DESC LIMIT 10",
        )?;

        let most_played = stmt
            .query_map([], |row| {
                let fav_num: i32 = row.get(11)?;
                Ok(Game {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    platform: row.get(2)?,
                    system_code: row.get(3)?,
                    year: row.get(4)?,
                    genre: row.get(5)?,
                    developer: row.get(6)?,
                    publisher: row.get(7)?,
                    controls: row.get(8)?,
                    launch_target: row.get(9)?,
                    cover_url: row.get(10)?,
                    is_favorite: fav_num == 1,
                    play_count: row.get(12)?,
                    last_played_at: row.get(13)?,
                })
            })?
            .collect::<Result<Vec<Game>>>()?;

        // Recently played
        let mut stmt2 = self.conn.prepare(
            "SELECT id, title, platform, system_code, year, genre, developer, publisher, controls, launch_target, cover_url, is_favorite, play_count, last_played_at
             FROM games WHERE last_played_at IS NOT NULL ORDER BY last_played_at DESC LIMIT 10",
        )?;

        let recently_played = stmt2
            .query_map([], |row| {
                let fav_num: i32 = row.get(11)?;
                Ok(Game {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    platform: row.get(2)?,
                    system_code: row.get(3)?,
                    year: row.get(4)?,
                    genre: row.get(5)?,
                    developer: row.get(6)?,
                    publisher: row.get(7)?,
                    controls: row.get(8)?,
                    launch_target: row.get(9)?,
                    cover_url: row.get(10)?,
                    is_favorite: fav_num == 1,
                    play_count: row.get(12)?,
                    last_played_at: row.get(13)?,
                })
            })?
            .collect::<Result<Vec<Game>>>()?;

        Ok(StatsSummary {
            total_games,
            total_platforms,
            total_favorites,
            total_plays,
            most_played,
            recently_played,
        })
    }
}

fn get_badge_color(sys: &str) -> &'static str {
    match sys.to_uppercase().as_str() {
        "MAME" => "#ff7b72",
        "C64" => "#79c0ff",
        "DOS" => "#ffa657",
        "SCUMMVM" => "#7ee787",
        "RECOMP" => "#d2a8ff",
        "INDIE" => "#f85149",
        "NAOMI" => "#56d364",
        "ATOMISWAVE" => "#3fb950",
        "SNES" => "#a5d6ff",
        "N64" => "#ffc680",
        "GAMECUBE" => "#bf91f3",
        "DREAMCAST" => "#ff9bce",
        "PSX" => "#80ccff",
        "HYPSEUS" => "#f0883e",
        "HIKARU" => "#f0883e",
        "NES" => "#f778ba",
        _ => "#8b949e",
    }
}

fn chrono_free_now() -> String {
    // Standard ISO8601 without extra heavy chrono dependency
    let now = std::time::SystemTime::now();
    let since_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", since_epoch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_crud() {
        let db = Database::open_in_memory().unwrap();
        let g = Game {
            id: 1,
            title: "Metal Slug".to_string(),
            platform: "MAME Arcade".to_string(),
            system_code: "MAME".to_string(),
            year: 1996,
            genre: "Run & Gun".to_string(),
            developer: "Nazca".to_string(),
            publisher: "SNK".to_string(),
            controls: "Pad".to_string(),
            launch_target: "mslug.lnk".to_string(),
            cover_url: None,
            is_favorite: false,
            play_count: 0,
            last_played_at: None,
        };

        db.upsert_game(&g).unwrap();
        let games = db.get_all_games().unwrap();
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].title, "Metal Slug");

        // Toggle fav
        let is_fav = db.toggle_favorite(1).unwrap();
        assert!(is_fav);
        assert_eq!(db.get_game_by_id(1).unwrap().unwrap().is_favorite, true);

        // Record play
        let count = db.record_play(1).unwrap();
        assert_eq!(count, 1);
    }
}
