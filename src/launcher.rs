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

        // Check if target is a file path
        let path = Path::new(trimmed);

        if trimmed.ends_with(".lnk") || trimmed.ends_with(".bat") {
            if !path.exists() {
                return Err(format!("File non trovato sul disco: {}", trimmed));
            }
            // Launch shortcut or batch via Windows shell
            Command::new("cmd")
                .args(["/C", "start", "", trimmed])
                .spawn()
                .map_err(|e| format!("Errore esecuzione comando: {}", e))?;
            return Ok(format!("Avviato: {}", trimmed));
        }

        // Direct executable or custom launch command
        if trimmed.contains("mame.exe") {
            // Split command and arguments
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if let Some((exe, args)) = parts.split_first() {
                let exe_path = Path::new(exe);
                let work_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));
                Command::new(exe)
                    .args(args)
                    .current_dir(work_dir)
                    .spawn()
                    .map_err(|e| format!("Errore esecuzione MAME: {}", e))?;
                return Ok(format!("MAME avviato con successo: {}", trimmed));
            }
        }

        // Generic launch via open crate
        if path.exists() {
            open::that(trimmed).map_err(|e| format!("Impossibile aprire il file: {}", e))?;
            Ok(format!("Avviato con successo: {}", trimmed))
        } else {
            // Try running through cmd in case it's a command line with parameters
            Command::new("cmd")
                .args(["/C", trimmed])
                .spawn()
                .map_err(|e| format!("Errore lancio comando Windows: {}", e))?;
            Ok(format!("Comando eseguito: {}", trimmed))
        }
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
