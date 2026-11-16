use rusqlite::Connection;
use std::collections::HashMap;
use std::path::Path;

struct GameInfo {
    id: i64,
    title: String,
    platform: String,
    cover_url: String,
    launch_target: String,
}

fn main() {
    println!("==================================================");
    println!("🔍 RETROHUB COVER AUDIT A TAPPETO (1149 GIOCHI)");
    println!("==================================================");

    let db_path = "retro_hub.db";
    let conn = Connection::open(db_path).expect("Apertura DB fallita");

    let mut stmt = conn
        .prepare("SELECT id, title, platform, cover_url, launch_target FROM games ORDER BY id ASC")
        .unwrap();

    let games: Vec<GameInfo> = stmt
        .query_map([], |row| {
            Ok(GameInfo {
                id: row.get(0)?,
                title: row.get(1)?,
                platform: row.get(2)?,
                cover_url: row.get(3)?,
                launch_target: row.get(4)?,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    println!("Totale giochi nel database: {}\n", games.len());

    let emu_base = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu");

    let mut by_platform: HashMap<String, Vec<&GameInfo>> = HashMap::new();
    for g in &games {
        by_platform.entry(g.platform.clone()).or_default().push(g);
    }

    // Ordina per numero decrescente
    let mut platforms: Vec<(String, usize)> = by_platform.iter().map(|(k, v)| (k.clone(), v.len())).collect();
    platforms.sort_by(|a, b| b.1.cmp(&a.1));

    let mut total_local_found = 0;
    let mut total_remote_assigned = 0;

    for (plat, count) in &platforms {
        let p_games = &by_platform[plat];
        let mut local_ok = 0;
        let mut remote_count = 0;
        let mut sample_urls = Vec::new();

        for g in p_games {
            if g.cover_url.starts_with("/api/cover/mame/") {
                let rom = g.cover_url.trim_start_matches("/api/cover/mame/");
                let mame_dir = emu_base.join("mame");
                let has_file = mame_dir.join("flyers").join(format!("{}.png", rom)).exists()
                    || mame_dir.join("flyers").join(format!("{}.jpg", rom)).exists()
                    || mame_dir.join("titles").join(format!("{}.png", rom)).exists()
                    || mame_dir.join("titles").join(format!("{}.jpg", rom)).exists()
                    || mame_dir.join("snap").join(format!("{}.png", rom)).exists()
                    || mame_dir.join("snap").join(format!("{}.jpg", rom)).exists();
                if has_file {
                    local_ok += 1;
                }
            } else if g.cover_url.starts_with("http") {
                remote_count += 1;
                if sample_urls.len() < 2 {
                    sample_urls.push((g.title.clone(), g.cover_url.clone()));
                }
            }
        }

        total_local_found += local_ok;
        total_remote_assigned += remote_count;

        println!("▶ {:<24} [{:>3} giochi] -> Locali OK: {:>3} | URL Remoti: {:>3}",
            plat, count, local_ok, remote_count);
        for (t, u) in sample_urls {
            let short_u = if u.len() > 65 { format!("{}...", &u[..62]) } else { u };
            println!("   - \"{}\" => {}", t, short_u);
        }
    }

    println!("\n--------------------------------------------------");
    println!("Riepilogo globale:");
    println!("Copertine verificate locali a 0ms: {}", total_local_found);
    println!("Copertine affidate a URL esterni:  {}", total_remote_assigned);
    println!("--------------------------------------------------");
}
