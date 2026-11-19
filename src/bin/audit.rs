use rusqlite::Connection;

#[allow(dead_code)]
struct GameInfo {
    title: String,
    platform: String,
    system_code: String,
    cover_url: String,
}

fn main() {
    let conn = Connection::open("retro_hub.db").expect("Apertura DB fallita");
    let mut stmt = conn
        .prepare("SELECT title, platform, system_code, cover_url FROM games ORDER BY system_code, title ASC")
        .unwrap();

    let games: Vec<GameInfo> = stmt
        .query_map([], |row| {
            Ok(GameInfo {
                title: row.get(0)?,
                platform: row.get(1)?,
                system_code: row.get(2)?,
                cover_url: row.get(3)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    for g in games {
        println!("[{}] {} => {}", g.system_code, g.title, g.cover_url);
    }
}
