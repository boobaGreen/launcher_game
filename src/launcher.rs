use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Launcher;

impl Launcher {
    /// Launches the given target path or command asynchronously in the foreground.
    pub fn launch(target: &str) -> Result<String, String> {
        Self::launch_target_with_system(target, "")
    }

    /// Launches a game model with its associated system_code for dedicated emulator resolution.
    pub fn launch_game(game: &crate::models::Game) -> Result<String, String> {
        Self::launch_target_with_system(&game.launch_target, &game.system_code)
    }

    /// Finds the matching emulator executable given a system_code and/or file extension.
    fn find_matching_emulator(system_code: &str, ext: &str) -> Option<PathBuf> {
        let roots = [
            PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu"),
            PathBuf::from("emulators"),
            PathBuf::from("emu"),
            PathBuf::from("."),
        ];

        let sys_upper = system_code.to_uppercase();
        let ext_lower = ext.to_lowercase();

        let relative_candidates: &[&str] = if sys_upper.contains("SNES") || ext_lower == "sfc" || ext_lower == "smc" {
            &["SNES/snes9x-x64.exe", "SNES/snes9x.exe", "snes9x/snes9x-x64.exe", "snes9x/snes9x.exe"]
        } else if sys_upper.contains("NES") || ext_lower == "nes" {
            &["NES/Mesen/Mesen.exe", "NES/Mesen.exe", "NES/fceux.exe", "mesen/Mesen.exe", "fceux/fceux.exe"]
        } else if sys_upper.contains("C64") || ext_lower == "d64" || ext_lower == "t64" || ext_lower == "prg" {
            &["C64/VICE/GTK3VICE-3.10-win64/bin/x64sc.exe", "C64/x64sc.exe", "vice/x64sc.exe", "C64/VICE/x64sc.exe"]
        } else if sys_upper.contains("DREAMCAST") || sys_upper.contains("NAOMI") || ext_lower == "cdi" || ext_lower == "gdi" {
            &["Dreamcast/flycast.exe", "NAOMI - Flycast/flycast.exe", "flycast/flycast.exe"]
        } else if sys_upper.contains("GAMECUBE") || ext_lower == "gcm" || ext_lower == "rvz" {
            &["GameCube/Dolphin.exe", "dolphin/Dolphin.exe"]
        } else if sys_upper.contains("PSX") || sys_upper.contains("PS1") || ext_lower == "chd" || ext_lower == "cue" {
            &["PSX/DuckStation/duckstation-qt-x64-ReleaseLTCG.exe", "PSX/duckstation.exe", "duckstation/duckstation.exe"]
        } else if sys_upper.contains("N64") || ext_lower == "z64" || ext_lower == "n64" || ext_lower == "v64" {
            &["Nintendo64/RMG.exe", "Nintendo64/Project64.exe", "RMG/RMG.exe"]
        } else if sys_upper.contains("SCUMMVM") {
            &["ScummVM/app/scummvm.exe", "ScummVM/scummvm.exe", "scummvm/scummvm.exe"]
        } else if sys_upper.contains("DOS") {
            &["DOS/dosbox/dosbox.exe", "DOS/dosbox.exe", "dosbox/dosbox.exe"]
        } else {
            &[]
        };

        for root in &roots {
            for cand in relative_candidates {
                let full = root.join(cand);
                if full.exists() {
                    return Some(full);
                }
            }
        }
        None
    }

    /// Core dispatcher with multi-candidate path resolution and emulator auto-discovery.
    pub fn launch_target_with_system(target: &str, system_code: &str) -> Result<String, String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Err("Target di lancio non specificato".to_string());
        }

        // Avvio nativo giochi PC Indie e LocoMalito con working directory dedicata
        if let Some(rest) = trimmed.strip_prefix("indie://") {
            let indie_roots = [
                PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\LocoMalito & Indie"),
                PathBuf::from("roms/indie"),
                PathBuf::from("emu/LocoMalito & Indie"),
                PathBuf::from("indie"),
            ];
            for root in &indie_roots {
                let full = root.join(rest);
                if full.exists() {
                    let dir = full.parent().unwrap_or(Path::new("."));
                    let escaped = full.to_string_lossy().replace('\'', "''");
                    let dir_escaped = dir.to_string_lossy().replace('\'', "''");
                    let script = format!(
                        r#"$p = Start-Process -FilePath '{}' -WorkingDirectory '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 400; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                        escaped, dir_escaped
                    );
                    Command::new("powershell")
                        .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                        .spawn()
                        .map_err(|e| format!("Errore lancio Indie: {}", e))?;
                    return Ok(format!("Avviato gioco Indie: {}", full.display()));
                }
            }
        }

        // Avvio nativo MAME con BGFX renderer e attivazione finestra in primo piano
        if let Some(rom) = trimmed.strip_prefix("MAME:") {
            let mame_dirs = [
                PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame"),
                PathBuf::from("emulators/mame"),
                PathBuf::from("emu/mame"),
                PathBuf::from("mame"),
            ];
            let mame_dir = mame_dirs.into_iter().find(|d| d.join("mame.exe").exists())
                .unwrap_or_else(|| PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame"));

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
                PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu").join(trimmed),
                PathBuf::from("emu").join(trimmed),
                PathBuf::from("roms").join(trimmed),
                PathBuf::from("downloads").join(trimmed),
            ];
            candidates.into_iter().find(|p| p.exists())
        };

        if let Some(target_file) = resolved_path {
            let ext = target_file.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            let is_direct_executable = matches!(ext.as_str(), "lnk" | "bat" | "cmd" | "exe");

            if is_direct_executable {
                // Avvio garantito di collegamenti .lnk, script .bat e file eseguibili
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

            // Se è un file ROM, cerca l'emulatore dedicato
            if let Some(emu_exe) = Self::find_matching_emulator(system_code, &ext) {
                let emu_escaped = emu_exe.to_string_lossy().replace('\'', "''");
                let rom_escaped = target_file.to_string_lossy().replace('\'', "''");
                let emu_dir = emu_exe.parent().unwrap_or(Path::new(".")).to_string_lossy().replace('\'', "''");

                let arg_list = if emu_escaped.to_lowercase().contains("dolphin") {
                    format!(r#"-b -e "{rom_escaped}""#)
                } else if emu_escaped.to_lowercase().contains("duckstation") {
                    format!(r#"-batch "{rom_escaped}""#)
                } else {
                    format!(r#""{rom_escaped}""#)
                };

                let script = format!(
                    r#"$p = Start-Process -FilePath '{}' -ArgumentList '{}' -WorkingDirectory '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 500; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                    emu_escaped,
                    arg_list,
                    emu_dir
                );
                Command::new("powershell")
                    .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                    .spawn()
                    .map_err(|e| format!("Errore lancio emulatore ({}): {}", emu_exe.display(), e))?;
                return Ok(format!("Avviato con {}: {}", emu_exe.file_name().unwrap_or_default().to_string_lossy(), target_file.display()));
            }

            // Fallback diretto Windows
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
