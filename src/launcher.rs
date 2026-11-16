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

        // Avvio nativo MAME: avvia direttamente l'eseguibile MAME con la ROM specifica e working dir corretta
        if let Some(rom) = trimmed.strip_prefix("MAME:") {
            let mame_dir = Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame");
            let mame_exe = mame_dir.join("mame.exe");
            Command::new(&mame_exe)
                .arg(rom.trim())
                .current_dir(mame_dir)
                .spawn()
                .map_err(|e| format!("Errore avvio MAME: {}", e))?;
            return Ok(format!("Avviato MAME (ROM: {})", rom));
        }

        let path = Path::new(trimmed);

        if path.exists() {
            // Usa il meccanismo nativo Windows Shell (ShellExecuteW con SW_SHOWNORMAL):
            // garantisce che la finestra dell'emulatore sia visibile in primo piano (MAI nascosta),
            // rispetta la working directory interna dei collegamenti .lnk e degli script .bat
            if let Err(e) = open::that(path) {
                let parent = path.parent().unwrap_or(Path::new("."));
                Command::new("cmd")
                    .args(&["/C", "start", "", trimmed])
                    .current_dir(parent)
                    .spawn()
                    .map_err(|e2| format!("Errore lancio (open: {}, cmd: {})", e, e2))?;
            }
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
