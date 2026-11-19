use crate::models::Game;
use std::path::PathBuf;

pub trait EmulatorPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect_installed(&self) -> bool;
    fn launch_game(&self, game: &Game) -> Result<String, String>;
    fn get_default_path(&self) -> Option<PathBuf>;
    fn get_supported_extensions(&self) -> Vec<&'static str>;
    fn get_platform_name(&self) -> &'static str;
}

#[allow(dead_code)]
pub struct EmulatorResult {
    pub success: bool,
    pub message: String,
    pub executable: Option<PathBuf>,
}