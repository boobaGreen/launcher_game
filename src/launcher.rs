use std::path::Path;
use std::process::Command;

pub struct Launcher;

impl Launcher {
    /// Launches the given target path or command asynchronously.
    pub fn launch(target: &str) -> Result<String, String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Err("Target di lancio non specificato".to_string());
        }

        let path = Path::new(trimmed);

        if path.exists() {
            // Using PowerShell Start-Process handles all shortcuts (.lnk), batch files (.bat),
            // and executables with complete immunity to apostrophes ('), ampersands (&),
            // parentheses, and spaces.
            let escaped = trimmed.replace('\'', "''");
            let script = format!("Start-Process -FilePath '{}'", escaped);
            Command::new("powershell")
                .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                .spawn()
                .map_err(|e| format!("Errore esecuzione comando: {}", e))?;
            return Ok(format!("Avviato: {}", trimmed));
        }

        // Try running through cmd in case it's an inline command
        Command::new("cmd")
            .args(["/C", trimmed])
            .spawn()
            .map_err(|e| format!("Errore lancio comando Windows: {}", e))?;
        Ok(format!("Comando eseguito: {}", trimmed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_target() {
        assert!(Launcher::launch("").is_err());
    }
}
