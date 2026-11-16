use crate::models::{Game, SearchFilter};
use strsim::jaro_winkler;

pub struct SearchEngine;

impl SearchEngine {
    /// Filters and ranks a list of games based on the provided SearchFilter.
    pub fn filter_and_rank(games: &[Game], filter: &SearchFilter) -> Vec<Game> {
        let query_tokens: Vec<String> = filter
            .query
            .as_ref()
            .map(|q| {
                q.to_lowercase()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let mut scored_games: Vec<(Game, f64)> = games
            .iter()
            .filter(|g| {
                // Platform filter
                if let Some(ref plat) = filter.platform {
                    if plat != "ALL" && !g.system_code.eq_ignore_ascii_case(plat) {
                        return false;
                    }
                }

                // Genre filter
                if let Some(ref gen) = filter.genre {
                    if gen != "ALL" && !g.genre.to_lowercase().contains(&gen.to_lowercase()) {
                        return false;
                    }
                }

                // Decade filter
                if let Some(ref dec) = filter.decade {
                    if dec != "ALL" {
                        let y = g.year;
                        let match_decade = match dec.as_str() {
                            "1980s" => (1980..=1989).contains(&y),
                            "1990s" => (1990..=1999).contains(&y),
                            "2000s" => (2000..=2009).contains(&y),
                            "2010s" => (2010..=2019).contains(&y),
                            "2020s" => y >= 2020,
                            _ => true,
                        };
                        if !match_decade {
                            return false;
                        }
                    }
                }

                // Favorites only filter
                if filter.favorites_only && !g.is_favorite {
                    return false;
                }

                true
            })
            .map(|g| {
                if query_tokens.is_empty() {
                    return (g.clone(), 1.0);
                }

                let score = Self::calculate_score(g, &query_tokens);
                (g.clone(), score)
            })
            .filter(|(_, score)| query_tokens.is_empty() || *score >= 0.40)
            .collect();

        // Sort results
        let sort_by = filter.sort_by.as_deref().unwrap_or("title");
        let asc = filter.sort_asc;

        scored_games.sort_by(|(a_game, a_score), (b_game, b_score)| {
            if !query_tokens.is_empty() {
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
                    let sim = jaro_winkler(word, token);
                    if sim > 0.70 {
                        token_max = token_max.max(sim * 0.85);
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
                token_max = token_max.max(0.6);
            }

            total_score += token_max;
        }

        total_score / (tokens.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_game(id: i64, title: &str, sys: &str, year: u16, genre: &str) -> Game {
        Game {
            id,
            title: title.to_string(),
            platform: sys.to_string(),
            system_code: sys.to_string(),
            year,
            genre: genre.to_string(),
            developer: "TestDev".to_string(),
            publisher: "TestPub".to_string(),
            controls: "Pad".to_string(),
            launch_target: "dummy.lnk".to_string(),
            cover_url: None,
            is_favorite: false,
            play_count: 0,
            last_played_at: None,
        }
    }

    #[test]
    fn test_exact_search() {
        let games = vec![
            dummy_game(1, "Street Fighter II", "MAME", 1991, "Picchiaduro"),
            dummy_game(2, "Super Mario World", "SNES", 1990, "Platform"),
            dummy_game(3, "Metal Slug", "MAME", 1996, "Run & Gun"),
        ];

        let filter = SearchFilter {
            query: Some("Street".to_string()),
            platform: None,
            genre: None,
            decade: None,
            favorites_only: false,
            sort_by: None,
            sort_asc: true,
        };

        let res = SearchEngine::filter_and_rank(&games, &filter);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, 1);
    }

    #[test]
    fn test_fuzzy_search() {
        let games = vec![
            dummy_game(1, "Street Fighter II", "MAME", 1991, "Picchiaduro"),
            dummy_game(2, "OutRun Arcade", "MAME", 1986, "Driving"),
        ];

        // Misspelled query "stret fiter"
        let filter = SearchFilter {
            query: Some("stret fiter".to_string()),
            platform: None,
            genre: None,
            decade: None,
            favorites_only: false,
            sort_by: None,
            sort_asc: true,
        };

        let res = SearchEngine::filter_and_rank(&games, &filter);
        assert!(!res.is_empty(), "Dovrebbe trovare Street Fighter anche con errori di battitura");
        assert_eq!(res[0].id, 1);
    }

    #[test]
    fn test_decade_filter() {
        let games = vec![
            dummy_game(1, "Pac-Man", "MAME", 1980, "Arcade"),
            dummy_game(2, "Sonic Adventure", "DREAMCAST", 1998, "Platform"),
            dummy_game(3, "Luigi's Mansion", "GAMECUBE", 2001, "Action"),
        ];

        let filter = SearchFilter {
            query: None,
            platform: None,
            genre: None,
            decade: Some("1990s".to_string()),
            favorites_only: false,
            sort_by: None,
            sort_asc: true,
        };

        let res = SearchEngine::filter_and_rank(&games, &filter);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, 2);
    }
}
