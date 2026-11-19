use std::path::Path;
use std::process::Command;

pub struct Launcher;

impl Launcher {
    /// Launches the given target path or command asynchronously in the foreground.
    pub fn launch(target: &str) -> Result<String, String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Err("Target di lancio non specificato".to_string());
        }

        // Avvio nativo MAME con BGFX renderer e attivazione finestra in primo piano
        if let Some(rom) = trimmed.strip_prefix("MAME:") {
            let mame_dirs = [
                std::path::PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame"),
                std::path::PathBuf::from("emulators/mame"),
                std::path::PathBuf::from("emu/mame"),
                std::path::PathBuf::from("mame"),
            ];
            let mame_dir = mame_dirs.into_iter().find(|d| d.join("mame.exe").exists())
                .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame"));

            let cmd = format!(
                r#"$p = Start-Process -FilePath ".\mame.exe" -ArgumentList "{} -video bgfx" -WorkingDirectory "{}" -PassThru; if ($p) {{ Start-Sleep -Milliseconds 400; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                rom.trim(),
                mame_dir.display()
            );
            Command::new("powershell")
                .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &cmd])
                .spawn()
                .map_err(|e| format!("Errore avvio MAME: {}", e))?;
            return Ok(format!("Avviato MAME (ROM: {})", rom));
        }

        let path = Path::new(trimmed);
        let resolved_path = if path.exists() {
            Some(path.to_path_buf())
        } else {
            let candidates = [
                std::path::PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu").join(trimmed),
                std::path::PathBuf::from("emu").join(trimmed),
                std::path::PathBuf::from("roms").join(trimmed),
                std::path::PathBuf::from("downloads").join(trimmed),
            ];
            candidates.into_iter().find(|p| p.exists())
        };

        if let Some(target_file) = resolved_path {
            // Avvio garantito di collegamenti .lnk, script .bat e file eseguibili
            // Rispetta la working directory interna del collegamento e forza il focus in primo piano
            let escaped = target_file.to_string_lossy().replace('\'', "''");
            let script = format!(
                r#"$p = Start-Process -FilePath '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 400; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                escaped
            );
            Command::new("powershell")
                .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                .spawn()
                .map_err(|e| format!("Errore lancio: {}", e))?;
            return Ok(format!("Avviato: {}", target_file.display()));
        }

        // Comando Windows generico di fallback
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
