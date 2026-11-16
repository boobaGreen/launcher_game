use rusqlite::Connection;
use std::path::Path;

fn main() {
    let db_path = "retro_hub.db";
    let conn = Connection::open(db_path).unwrap();
    let emu_base = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");
    let mame_dir = emu_base.join("mame");

    let mut stmt = conn
        .prepare("SELECT id, title, cover_url FROM games WHERE platform = 'MAME Arcade'")
        .unwrap();

    let games = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let title: String = row.get(1)?;
            let cover_url: String = row.get(2)?;
            Ok((id, title, cover_url))
        })
        .unwrap();

    let mut missing = Vec::new();
    for g in games.flatten() {
        let (id, title, cover_url) = g;
        if cover_url.starts_with("/api/cover/mame/") {
            let rom = cover_url.trim_start_matches("/api/cover/mame/");
            let has_file = mame_dir.join("flyers").join(format!("{}.png", rom)).exists()
                || mame_dir.join("flyers").join(format!("{}.jpg", rom)).exists()
                || mame_dir.join("titles").join(format!("{}.png", rom)).exists()
                || mame_dir.join("titles").join(format!("{}.jpg", rom)).exists()
                || mame_dir.join("snap").join(format!("{}.png", rom)).exists()
                || mame_dir.join("snap").join(format!("{}.jpg", rom)).exists();
            if !has_file {
                missing.push((id, title, rom.to_string()));
            }
        } else {
            missing.push((id, title, format!("NON_MAME_URL: {}", cover_url)));
        }
    }

    println!("MAME Games senza file locale ({} su 596):", missing.len());
    for (id, t, r) in missing {
        println!("  ID {:3} | {:<35} | ROM: {}", id, t, r);
    }
}
