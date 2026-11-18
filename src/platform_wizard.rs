use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformWizard {
    pub selected_platforms: HashSet<String>,
    pub show_all: bool,
}

impl PlatformWizard {
    pub fn new() -> Self {
        Self {
            selected_platforms: HashSet::new(),
            show_all: false,
        }
    }

    pub fn toggle_platform(&mut self, platform: &str) {
        if !self.selected_platforms.insert(platform.to_string()) {
            self.selected_platforms.remove(platform);
        }
    }

    pub fn select_all(&mut self, platforms: &[String]) {
        for p in platforms {
            self.selected_platforms.insert(p.clone());
        }
        self.show_all = true;
    }

    pub fn clear_all(&mut self) {
        self.selected_platforms.clear();
        self.show_all = false;
    }

    pub fn is_selected(&self, platform: &str) -> bool {
        self.show_all || self.selected_platforms.contains(platform)
    }

    pub fn get_selected(&self) -> Vec<String> {
        self.selected_platforms.iter().cloned().collect()
    }
}