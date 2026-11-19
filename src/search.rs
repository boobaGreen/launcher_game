use crate::models::{Game, SearchFilter};
use strsim::jaro_winkler;

#[derive(Debug, Clone, Default)]
pub struct ParsedQuery {
    pub positive_terms: Vec<String>,
    pub exact_terms: Vec<String>,
    pub negative_terms: Vec<String>,
    pub year_min: Option<u16>,
    pub year_max: Option<u16>,
    pub required_genre: Option<String>,
    pub excluded_genre: Option<String>,
    pub required_dev: Option<String>,
    pub excluded_dev: Option<String>,
    pub required_sys: Option<String>,
    pub excluded_sys: Option<String>,
    pub required_title: Option<String>,
    pub no_multiplayer: bool,
    pub only_multiplayer: bool,
}

pub struct SearchEngine;

impl SearchEngine {
    /// Tokenizes query string into terms preserving quoted phrases e.g. "lupin" or 'super mario'
    pub fn tokenize(raw: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut quote_char = '"';

        for c in raw.chars() {
            if (c == '"' || c == '\'') && !in_quotes {
                in_quotes = true;
                quote_char = c;
                current.push(c);
            } else if in_quotes && c == quote_char {
                in_quotes = false;
                current.push(c);
            } else if c.is_whitespace() && !in_quotes {
                if !current.is_empty() {
                    tokens.push(current);
                    current = String::new();
                }
            } else {
                current.push(c);
            }
        }
        if !current.is_empty() {
            tokens.push(current);
        }
        tokens
    }

    /// Parses a raw user query string into structured compound criteria.
    pub fn parse_query(raw: &str) -> ParsedQuery {
        let mut pq = ParsedQuery::default();
        let tokens = Self::tokenize(raw);
        let mut i = 0;

        while i < tokens.len() {
            let token = &tokens[i];
            let lower = token.to_lowercase();

            // Support standalone 'NOT', 'SENZA', '!' as prefix keyword for the next token
            if (lower == "not" || lower == "senza" || lower == "!") && i + 1 < tokens.len() {
                i += 1;
                let next_token = &tokens[i];
                let next_clean = next_token.trim_matches(|c| c == '"' || c == '\'').to_lowercase();
                Self::add_negative_clause(&mut pq, &next_clean);
                i += 1;
                continue;
            }

            // 1. Check for exclusions with '-' or '!' prefix
            if lower.starts_with('-') || lower.starts_with('!') {
                let neg = &token[1..];
                let neg_clean = neg.trim_matches(|c| c == '"' || c == '\'').to_lowercase();
                if !neg_clean.is_empty() {
                    Self::add_negative_clause(&mut pq, &neg_clean);
                }
                i += 1;
                continue;
            }

            // 2. Exact quoted term: "lupin" or 'super mario'
            if (lower.starts_with('"') && lower.ends_with('"') && lower.len() >= 2)
                || (lower.starts_with('\'') && lower.ends_with('\'') && lower.len() >= 2)
            {
                let exact = token[1..token.len() - 1].trim().to_lowercase();
                if !exact.is_empty() {
                    pq.exact_terms.push(exact);
                }
                i += 1;
                continue;
            }

            // 3. Year range: e.g. "1985..1992" or "1985-1992" or "year:1985..1992"
            let year_target = lower.strip_prefix("year:")
                .or_else(|| lower.strip_prefix("anno:"))
                .unwrap_or(&lower);

            if let Some((start_s, end_s)) = year_target.split_once("..") {
                if let (Ok(y1), Ok(y2)) = (start_s.parse::<u16>(), end_s.parse::<u16>()) {
                    pq.year_min = Some(y1.min(y2));
                    pq.year_max = Some(y1.max(y2));
                    i += 1;
                    continue;
                }
            } else if let Some((start_s, end_s)) = year_target.split_once('-') {
                if start_s.len() == 4 && end_s.len() == 4 {
                    if let (Ok(y1), Ok(y2)) = (start_s.parse::<u16>(), end_s.parse::<u16>()) {
                        pq.year_min = Some(y1.min(y2));
                        pq.year_max = Some(y1.max(y2));
                        i += 1;
                        continue;
                    }
                }
            } else if let Some(min_s) = year_target.strip_prefix('>') {
                if let Ok(y) = min_s.parse::<u16>() {
                    pq.year_min = Some(y);
                    i += 1;
                    continue;
                }
            } else if let Some(max_s) = year_target.strip_prefix('<') {
                if let Ok(y) = max_s.parse::<u16>() {
                    pq.year_max = Some(y);
                    i += 1;
                    continue;
                }
            } else if lower.starts_with("year:") || lower.starts_with("anno:") {
                if let Ok(y) = year_target.parse::<u16>() {
                    pq.year_min = Some(y);
                    pq.year_max = Some(y);
                    i += 1;
                    continue;
                }
            }

            // 4. Field prefixes: title:, exact:, genre:, dev:, sys:
            if let Some(val) = lower.strip_prefix("title:")
                .or_else(|| lower.strip_prefix("titolo:"))
                .or_else(|| lower.strip_prefix("t:"))
                .or_else(|| lower.strip_prefix("exact:"))
            {
                let clean_val = val.trim_matches(|c| c == '"' || c == '\'').trim();
                if !clean_val.is_empty() {
                    pq.required_title = Some(clean_val.to_string());
                }
                i += 1;
                continue;
            }

            if let Some(val) = lower.strip_prefix("genre:")
                .or_else(|| lower.strip_prefix("genere:"))
                .or_else(|| lower.strip_prefix("g:"))
            {
                let clean_val = val.trim_matches(|c| c == '"' || c == '\'').trim();
                pq.required_genre = Some(clean_val.to_string());
                i += 1;
                continue;
            }

            if let Some(val) = lower.strip_prefix("dev:")
                .or_else(|| lower.strip_prefix("developer:"))
                .or_else(|| lower.strip_prefix("autore:"))
                .or_else(|| lower.strip_prefix("pub:"))
            {
                let clean_val = val.trim_matches(|c| c == '"' || c == '\'').trim();
                pq.required_dev = Some(clean_val.to_string());
                i += 1;
                continue;
            }

            if let Some(val) = lower.strip_prefix("sys:")
                .or_else(|| lower.strip_prefix("platform:"))
                .or_else(|| lower.strip_prefix("p:"))
            {
                let clean_val = val.trim_matches(|c| c == '"' || c == '\'').trim();
                pq.required_sys = Some(clean_val.to_string());
                i += 1;
                continue;
            }

            // 5. Multiplayer / Singleplayer flags
            if lower == "single" || lower == "singleplayer" || lower == "1p" {
                pq.no_multiplayer = true;
                i += 1;
                continue;
            }
            if lower == "multi" || lower == "multiplayer" || lower == "2p" {
                pq.only_multiplayer = true;
                i += 1;
                continue;
            }

            // 6. Default positive keyword (multi-field search)
            let clean_pos = lower.trim_matches(|c| c == '"' || c == '\'').trim();
            if !clean_pos.is_empty() {
                pq.positive_terms.push(clean_pos.to_string());
            }
            i += 1;
        }

        pq
    }

    fn add_negative_clause(pq: &mut ParsedQuery, neg: &str) {
        if neg == "multi" || neg == "multiplayer" || neg == "2p" || neg == "coop" {
            pq.no_multiplayer = true;
        } else if let Some(val) = neg.strip_prefix("dev:")
            .or_else(|| neg.strip_prefix("developer:"))
            .or_else(|| neg.strip_prefix("autore:"))
            .or_else(|| neg.strip_prefix("pub:"))
        {
            pq.excluded_dev = Some(val.to_string());
        } else if let Some(val) = neg.strip_prefix("genre:")
            .or_else(|| neg.strip_prefix("genere:")
            .or_else(|| neg.strip_prefix("g:")))
        {
            pq.excluded_genre = Some(val.to_string());
        } else if let Some(val) = neg.strip_prefix("sys:")
            .or_else(|| neg.strip_prefix("platform:")
            .or_else(|| neg.strip_prefix("p:")))
        {
            pq.excluded_sys = Some(val.to_string());
        } else {
            pq.negative_terms.push(neg.to_string());
        }
    }

    /// Determines if a game has multiplayer features based on controls/genre/title.
    fn is_multiplayer_game(game: &Game) -> bool {
        let controls = game.controls.to_lowercase();
        let genre = game.genre.to_lowercase();
        let title = game.title.to_lowercase();

        controls.contains("2p")
            || controls.contains("2 giocatori")
            || controls.contains("due giocatori")
            || controls.contains("multiplayer")
            || controls.contains("coop")
            || controls.contains("co-op")
            || controls.contains("vs")
            || genre.contains("multiplayer")
            || genre.contains("vs")
            || title.contains("2 player")
    }

    /// Determines if a single game satisfies all clauses in a ParsedQuery, returning (matches, score)
    pub fn matches_parsed(g: &Game, parsed: &ParsedQuery) -> (bool, f64) {
        // Year Min / Max Range
        if let Some(min) = parsed.year_min {
            if g.year < min {
                return (false, 0.0);
            }
        }
        if let Some(max) = parsed.year_max {
            if g.year > max {
                return (false, 0.0);
            }
        }

        // Required Genre (e.g. genre:shmup)
        if let Some(ref req_g) = parsed.required_genre {
            if !g.genre.to_lowercase().contains(req_g) {
                return (false, 0.0);
            }
        }

        // Excluded Genre (e.g. -genre:rpg)
        if let Some(ref exc_g) = parsed.excluded_genre {
            if g.genre.to_lowercase().contains(exc_g) {
                return (false, 0.0);
            }
        }

        // Required Developer / Publisher (e.g. dev:capcom)
        if let Some(ref req_d) = parsed.required_dev {
            let d = g.developer.to_lowercase();
            let p = g.publisher.to_lowercase();
            if !d.contains(req_d) && !p.contains(req_d) {
                return (false, 0.0);
            }
        }

        // Excluded Developer / Publisher (e.g. -dev:capcom)
        if let Some(ref exc_d) = parsed.excluded_dev {
            let d = g.developer.to_lowercase();
            let p = g.publisher.to_lowercase();
            if d.contains(exc_d) || p.contains(exc_d) {
                return (false, 0.0);
            }
        }

        // Required System (e.g. sys:c64)
        if let Some(ref req_s) = parsed.required_sys {
            if !g.system_code.to_lowercase().contains(req_s) && !g.platform.to_lowercase().contains(req_s) {
                return (false, 0.0);
            }
        }

        // Excluded System (e.g. -sys:mame)
        if let Some(ref exc_s) = parsed.excluded_sys {
            if g.system_code.to_lowercase().contains(exc_s) || g.platform.to_lowercase().contains(exc_s) {
                return (false, 0.0);
            }
        }

        // Excluded Multiplayer (e.g. -multi, -2p, single)
        let is_multi = Self::is_multiplayer_game(g);
        if parsed.no_multiplayer && is_multi {
            return (false, 0.0);
        }
        if parsed.only_multiplayer && !is_multi {
            return (false, 0.0);
        }

        // General Negative Terms
        for neg in &parsed.negative_terms {
            let t = g.title.to_lowercase();
            let d = g.developer.to_lowercase();
            let p = g.publisher.to_lowercase();
            let gen = g.genre.to_lowercase();
            let sys = g.system_code.to_lowercase();
            let plat = g.platform.to_lowercase();
            let y = g.year.to_string();

            if t.contains(neg) || d.contains(neg) || p.contains(neg) || gen.contains(neg) || sys.contains(neg) || plat.contains(neg) || y == *neg {
                return (false, 0.0);
            }
        }

        // Exact Terms from Quotes e.g. "lupin"
        for exact in &parsed.exact_terms {
            let t = g.title.to_lowercase();
            let d = g.developer.to_lowercase();
            let p = g.publisher.to_lowercase();
            let gen = g.genre.to_lowercase();
            let sys = g.system_code.to_lowercase();
            let plat = g.platform.to_lowercase();

            if !t.contains(exact) && !d.contains(exact) && !p.contains(exact) && !gen.contains(exact) && !sys.contains(exact) && !plat.contains(exact) {
                return (false, 0.0);
            }
        }

        // Required Title e.g. title:lupin or exact:lupin
        if let Some(ref req_t) = parsed.required_title {
            if !g.title.to_lowercase().contains(req_t) {
                return (false, 0.0);
            }
        }

        if parsed.positive_terms.is_empty() {
            return (true, 1.0);
        }

        let score = Self::calculate_score(g, &parsed.positive_terms);
        if score >= 0.38 {
            (true, score)
        } else {
            (false, 0.0)
        }
    }

    /// Filters and ranks a list of games based on the provided SearchFilter.
    pub fn filter_and_rank(games: &[Game], filter: &SearchFilter) -> Vec<Game> {
        let raw_query = filter.query.as_deref().unwrap_or("").trim();
        let subqueries: Vec<&str> = if raw_query.contains(" | ") {
            raw_query.split(" | ").collect()
        } else if raw_query.contains(" OR ") {
            raw_query.split(" OR ").collect()
        } else {
            vec![raw_query]
        };

        let parsed_list: Vec<ParsedQuery> = subqueries
            .iter()
            .map(|q| Self::parse_query(q.trim()))
            .collect();

        let has_query = !raw_query.is_empty();

        let mut scored_games: Vec<(Game, f64)> = games
            .iter()
            .filter_map(|g| {
                // UI Platform dropdown filter
                if let Some(ref plat) = filter.platform {
                    if plat != "ALL" && !g.system_code.eq_ignore_ascii_case(plat) {
                        return None;
                    }
                }

                // UI Genre dropdown filter
                if let Some(ref gen) = filter.genre {
                    if gen != "ALL" && !g.genre.to_lowercase().contains(&gen.to_lowercase()) {
                        return None;
                    }
                }

                // UI Developer dropdown filter
                if let Some(ref dev) = filter.developer {
                    if dev != "ALL" {
                        let target = dev.to_lowercase();
                        if !g.developer.to_lowercase().contains(&target) && !g.publisher.to_lowercase().contains(&target) {
                            return None;
                        }
                    }
                }

                // UI Decade dropdown filter
                if let Some(ref dec) = filter.decade {
                    if dec != "ALL" {
                        let y = g.year;
                        let match_decade = match dec.as_str() {
                            "1970s" => (1970..=1979).contains(&y),
                            "1980s" => (1980..=1989).contains(&y),
                            "1990s" => (1990..=1999).contains(&y),
                            "2000s" => (2000..=2009).contains(&y),
                            "2010s" => y >= 2010,
                            _ => true,
                        };
                        if !match_decade {
                            return None;
                        }
                    }
                }

                // UI Year Range filter (da / a)
                if let Some(min) = filter.year_from {
                    if g.year < min {
                        return None;
                    }
                }
                if let Some(max) = filter.year_to {
                    if g.year > max {
                        return None;
                    }
                }

                // Favorites only filter
                if filter.favorites_only && !g.is_favorite {
                    return None;
                }

                // Installed only filter (Zero-Disk Enciclopedia vs Giochi Locali)
                if filter.installed_only && !g.is_installed {
                    return None;
                }

                if !has_query {
                    return Some((g.clone(), 1.0));
                }

                // Evaluate subqueries (OR logic across branches)
                let mut best_score = 0.0;
                let mut matched = false;

                for pq in &parsed_list {
                    let (m, s) = Self::matches_parsed(g, pq);
                    if m {
                        matched = true;
                        if s > best_score {
                            best_score = s;
                        }
                    }
                }

                if matched {
                    Some((g.clone(), best_score))
                } else {
                    None
                }
            })
            .collect();

        // Sort results
        let sort_by = filter.sort_by.as_deref().unwrap_or("title");
        let asc = filter.sort_asc;

        scored_games.sort_by(|(a_game, a_score), (b_game, b_score)| {
            if has_query {
                // In search mode, primary order is by relevance score descending
                let cmp = b_score.partial_cmp(a_score).unwrap_or(std::cmp::Ordering::Equal);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }

            match sort_by {
                "year" => {
                    if asc { a_game.year.cmp(&b_game.year) } else { b_game.year.cmp(&a_game.year) }
                }
                "play_count" => {
                    if asc { a_game.play_count.cmp(&b_game.play_count) } else { b_game.play_count.cmp(&a_game.play_count) }
                }
                "platform" => {
                    if asc { a_game.platform.cmp(&b_game.platform) } else { b_game.platform.cmp(&a_game.platform) }
                }
                _ => {
                    if asc {
                        a_game.title.to_lowercase().cmp(&b_game.title.to_lowercase())
                    } else {
                        b_game.title.to_lowercase().cmp(&a_game.title.to_lowercase())
                    }
                }
            }
        });

        scored_games.into_iter().map(|(g, _)| g).collect()
    }

    fn calculate_score(game: &Game, tokens: &[String]) -> f64 {
        let title_lower = game.title.to_lowercase();
        let title_words: Vec<&str> = title_lower.split_whitespace().collect();
        let dev_lower = game.developer.to_lowercase();
        let genre_lower = game.genre.to_lowercase();
        let pub_lower = game.publisher.to_lowercase();
        let year_str = game.year.to_string();

        let mut total_score = 0.0;

        for token in tokens {
            let mut token_max: f64 = 0.0;

            // 1. Substring in full title
            if title_lower.contains(token) {
                token_max = token_max.max(0.85);
            }

            // 2. Word-by-word title matching (exact or fuzzy)
            for word in &title_words {
                if *word == token {
                    token_max = token_max.max(1.0);
                } else if word.starts_with(token) {
                    token_max = token_max.max(0.9);
                } else {
                    // Only perform fuzzy matching if the word and token are reasonably long (>= 4 chars)
                    // and similarity is high (>= 0.90) to avoid matching unrelated words like 'lupo' and 'lupin'
                    if token.len() >= 4 && word.len() >= 4 {
                        let sim = jaro_winkler(word, token);
                        if sim >= 0.90 {
                            token_max = token_max.max(sim * 0.85);
                        }
                    }
                }
            }

            // 3. Developer / Publisher match
            if dev_lower.contains(token) || pub_lower.contains(token) {
                token_max = token_max.max(0.7);
            }

            // 4. Genre match
            if genre_lower.contains(token) {
                token_max = token_max.max(0.65);
            }

            // 5. Year match
            if year_str == *token {
                token_max = token_max.max(0.7);
            }

            // 6. Platform / System match
            let sys_lower = game.system_code.to_lowercase();
            let plat_lower = game.platform.to_lowercase();
            if sys_lower.contains(token) || plat_lower.contains(token) {
                token_max = token_max.max(0.75);
            }

            total_score += token_max;
        }

        total_score / (tokens.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_game(id: i64, title: &str, sys: &str, year: u16, genre: &str, dev: &str, controls: &str) -> Game {
        Game {
            id,
            title: title.to_string(),
            platform: sys.to_string(),
            system_code: sys.to_string(),
            year,
            genre: genre.to_string(),
            developer: dev.to_string(),
            publisher: dev.to_string(),
            controls: controls.to_string(),
            launch_target: "dummy.lnk".to_string(),
            cover_url: None,
            is_favorite: false,
            play_count: 0,
            last_played_at: None,
            is_installed: true,
        }
    }

    #[test]
    fn test_compound_query_year_range_and_negative() {
        let games = vec![
            dummy_game(1, "Street Fighter II", "MAME", 1991, "Picchiaduro", "Capcom", "Start 1P / 2P vs"),
            dummy_game(2, "Fatal Fury", "MAME", 1991, "Picchiaduro", "SNK", "Start 1P / 2P vs"),
            dummy_game(3, "Mortal Kombat", "MAME", 1992, "Picchiaduro", "Midway", "Start 1P / 2P vs"),
            dummy_game(4, "Kung-Fu Master", "MAME", 1984, "Picchiaduro", "Irem", "Single Player"),
        ];

        // Query: 1990..1995 Picchiaduro -Capcom
        let filter = SearchFilter {
            query: Some("1990..1995 Picchiaduro -Capcom".to_string()),
            sort_asc: true,
            ..Default::default()
        };

        let res = SearchEngine::filter_and_rank(&games, &filter);
        assert_eq!(res.len(), 2);
        assert!(res.iter().any(|g| g.title == "Fatal Fury"));
        assert!(res.iter().any(|g| g.title == "Mortal Kombat"));
        assert!(!res.iter().any(|g| g.title == "Street Fighter II")); // Excluded by -Capcom
        assert!(!res.iter().any(|g| g.title == "Kung-Fu Master")); // Excluded by year range
    }

    #[test]
    fn test_compound_query_exclude_multiplayer() {
        let games = vec![
            dummy_game(1, "1942", "C64", 1986, "Shmup", "Capcom", "Joystick Porta 2 (Single)"),
            dummy_game(2, "1943 2-Player", "C64", 1988, "Shmup", "Capcom", "Start 1P / 2P Coop"),
        ];

        // Exclude multiplayer: -multi
        let filter = SearchFilter {
            query: Some("194 -multi".to_string()),
            sort_asc: true,
            ..Default::default()
        };

        let res = SearchEngine::filter_and_rank(&games, &filter);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, 1);
    }

    #[test]
    fn test_prefix_syntax() {
        let games = vec![
            dummy_game(1, "Zelda Majora", "RECOMP", 2000, "Action RPG", "Nintendo", "XInput 1P"),
            dummy_game(2, "Zelda Ocarina", "RECOMP", 1998, "Action RPG", "Nintendo", "XInput 1P"),
            dummy_game(3, "Final Fantasy VII", "PSX", 1997, "JRPG", "Square", "1P Pad"),
        ];

        // Query: genre:rpg dev:nintendo 1999..2005
        let filter = SearchFilter {
            query: Some("genre:rpg dev:nintendo 1999..2005".to_string()),
            sort_asc: true,
            ..Default::default()
        };

        let res = SearchEngine::filter_and_rank(&games, &filter);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, 1); // Zelda Majora
    }

    #[test]
    fn test_exact_quotes_and_fuzzy_separation() {
        let games = vec![
            dummy_game(1, "Lupin Sansei - Pandora no Isan", "NES", 1987, "Platform", "Namco", "1P"),
            dummy_game(2, "Lupo Alberto: The Videogame", "C64", 1990, "Platform", "Idea", "1P"),
        ];

        // Search "lupin" with quotes must match ONLY Lupin
        let filter_quoted = SearchFilter {
            query: Some("\"lupin\"".to_string()),
            sort_asc: true,
            ..Default::default()
        };
        let res1 = SearchEngine::filter_and_rank(&games, &filter_quoted);
        assert_eq!(res1.len(), 1);
        assert_eq!(res1[0].id, 1);
        assert_eq!(res1[0].title, "Lupin Sansei - Pandora no Isan");

        // Search lupin without quotes should also NOT match Lupo Alberto
        let filter_unquoted = SearchFilter {
            query: Some("lupin".to_string()),
            sort_asc: true,
            ..Default::default()
        };
        let res2 = SearchEngine::filter_and_rank(&games, &filter_unquoted);
        assert_eq!(res2.len(), 1);
        assert_eq!(res2[0].id, 1);

        // Search exact title: title:lupin
        let filter_title = SearchFilter {
            query: Some("title:lupin".to_string()),
            sort_asc: true,
            ..Default::default()
        };
        let res3 = SearchEngine::filter_and_rank(&games, &filter_title);
        assert_eq!(res3.len(), 1);
        assert_eq!(res3[0].id, 1);
    }
}
