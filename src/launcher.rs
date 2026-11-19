use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Launcher;

impl Launcher {
    /// Launches the given target path or command asynchronously in the foreground.
    pub fn launch(target: &str) -> Result<String, String> {
        Self::launch_target_with_system(target, "")
    }

    /// Launches a game model with dynamic auto-resolution and dedicated emulator execution.
    pub fn launch_game(game: &crate::models::Game) -> Result<String, String> {
        let mut target = game.launch_target.clone();
        if target.trim().is_empty() {
            if let Some(resolved) = Self::auto_resolve_target(game) {
                target = resolved;
            } else {
                let sys = game.system_code.to_uppercase();
                let config = crate::config::RetroHubConfig::load();
                if sys == "PINBALL" {
                    if config.find_vpx().is_none() {
                        return Err("NOT_CONFIGURED:PINBALL:Visual Pinball (VPX) non configurato. Configura il percorso di VPinballX.exe nelle Opzioni (⚙️) per giocare questo tavolo.".to_string());
                    }
                    return Err(format!("Tavolo flipper non trovato sul disco per '{}'. Scarica il file .vpx prima di avviarlo.", game.title));
                }
                if sys == "GAMEWATCH" || sys == "LCD" {
                    if config.find_retroarch().is_none() {
                        return Err("NOT_CONFIGURED:GAMEWATCH:Gioco Game & Watch non configurato. Configura RetroArch (core Handheld) nelle Opzioni (⚙️) per giocare questo titolo.".to_string());
                    }
                    return Err(format!("ROM non trovata sul disco per '{}'. Scarica il gioco prima di avviarlo con RetroArch.", game.title));
                }
                return Err(format!("ROM non trovata sul disco per '{}'. Scarica il gioco prima di avviarlo.", game.title));
            }
        }
        Self::launch_target_with_system(&target, &game.system_code)
    }

    /// Tenta di risolvere automaticamente il file ROM o l'eseguibile sul disco in base a titolo e sistema.
    pub fn auto_resolve_target(game: &crate::models::Game) -> Option<String> {
        let sys_upper = game.system_code.to_uppercase();
        let clean_title = game.title.replace([':', '/', '\\', '?', '*', '"', '<', '>', '|'], "").trim().to_string();

        // 1. MAME / Arcade
        if sys_upper == "MAME" || sys_upper == "ARCADE" {
            let map_str = include_str!("mame_rom_map.json");
            if let Ok(map) = serde_json::from_str::<std::collections::HashMap<String, String>>(map_str) {
                if let Some(rom) = map.get(&game.title.to_lowercase()) {
                    return Some(format!("MAME:{}", rom));
                }
            }
            let candidates = [
                PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame\roms").join(format!("{}.zip", clean_title)),
                PathBuf::from("downloads/mame").join(format!("{}.zip", clean_title)),
            ];
            for cand in &candidates {
                if cand.exists() {
                    let rom = cand.file_stem().and_then(|s| s.to_str()).unwrap_or(&clean_title);
                    return Some(format!("MAME:{}", rom));
                }
            }
        }

        // 2. Hypseus / Laserdisc
        if sys_upper == "HYPSEUS" || sys_upper == "LASERDISC" || sys_upper == "DAPHNE" {
            let base = PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\Hypseus Singe");
            let cands = [
                base.join(format!("Gioca - {}.lnk", clean_title)),
                base.join(format!("Avvia_{}.bat", clean_title.replace(' ', ""))),
                base.join("Gioca.lnk"),
            ];
            for c in &cands {
                if c.exists() {
                    return Some(c.to_string_lossy().to_string());
                }
            }
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_lowercase();
                    let t_lower = game.title.to_lowercase();
                    if name.ends_with(".lnk") || name.ends_with(".bat") {
                        if name.contains(&t_lower) || t_lower.split_whitespace().any(|word| word.len() > 3 && name.contains(word)) {
                            return Some(entry.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
        }

        // 3. PC Indie & LocoMalito
        if sys_upper == "INDIE" {
            let base = PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\LocoMalito & Indie");
            let cands = [
                base.join(format!("Gioca - {}.lnk", clean_title)),
                base.join(&clean_title).join("Gioca.lnk"),
                base.join(&clean_title).join(format!("{}.exe", clean_title)),
            ];
            for c in &cands {
                if c.exists() {
                    return Some(c.to_string_lossy().to_string());
                }
            }
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        let dir_name = entry.file_name().to_string_lossy().to_lowercase();
                        if dir_name.contains(&clean_title.to_lowercase()) {
                            let gioca = p.join("Gioca.lnk");
                            if gioca.exists() {
                                return Some(gioca.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }

        // 4. PC Recomp
        if sys_upper == "RECOMP" {
            let base = PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\Recomp Library");
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_lowercase();
                    let t_lower = game.title.to_lowercase();
                    if name.ends_with(".lnk") && (name.contains(&t_lower) || t_lower.split_whitespace().any(|w| w.len() > 3 && name.contains(w))) {
                        return Some(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }

        // 5. Console & Computer ROMs (downloads/<platform>, emu/<platform>, roms/<platform>)
        let search_dirs = [
            PathBuf::from(format!("downloads/{}", game.system_code.to_lowercase())),
            PathBuf::from(format!("downloads/{}", sys_upper.to_lowercase())),
            PathBuf::from(format!(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\{}", game.system_code)),
            PathBuf::from(format!(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\{}\roms", game.system_code)),
            PathBuf::from(format!("roms/{}", game.system_code.to_lowercase())),
        ];

        let valid_extensions: &[&str] = match sys_upper.as_str() {
            "SNES" => &["sfc", "smc", "zip"],
            "NES" => &["nes", "zip"],
            "N64" => &["z64", "n64", "v64", "zip"],
            "GBA" => &["gba", "zip"],
            "MEGADRIVE" | "GENESIS" => &["md", "bin", "gen", "zip"],
            "PSX" | "PS1" => &["chd", "cue", "iso", "bin", "pbp"],
            "GAMECUBE" => &["rvz", "gcm", "iso"],
            "DREAMCAST" | "NAOMI" => &["cdi", "gdi", "chd", "zip"],
            "C64" => &["d64", "t64", "prg", "crt", "zip"],
            "DOS" => &["exe", "bat", "com", "conf"],
            "SCUMMVM" => &["scummvm", "ini"],
            "GAMEWATCH" | "LCD" => &["mgw", "zip", "7z"],
            "PINBALL" => &["vpx", "fpt"],
            _ => &["zip", "7z", "iso", "rom"],
        };

        for dir in &search_dirs {
            if !dir.exists() {
                continue;
            }
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                        if valid_extensions.contains(&ext.as_str()) {
                            let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
                            let t_lower = game.title.to_lowercase();
                            if file_name == t_lower || file_name.starts_with(&t_lower) || t_lower.starts_with(&file_name) {
                                return Some(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Finds the matching emulator executable given a system_code and/or file extension.
    fn find_matching_emulator(system_code: &str, ext: &str) -> Option<PathBuf> {
        let config = crate::config::RetroHubConfig::load();
        let sys_upper = system_code.to_uppercase();
        let ext_lower = ext.to_lowercase();

        // 1. RetroArch per Game & Watch / LCD Handhelds
        if sys_upper.contains("GAMEWATCH") || sys_upper.contains("LCD") || ext_lower == "mgw" {
            if let Some(ra) = config.find_retroarch() {
                return Some(ra);
            }
        }

        // 2. Visual Pinball per tavoli Flipper
        if sys_upper.contains("PINBALL") || ext_lower == "vpx" || ext_lower == "fpt" {
            if let Some(vpx) = config.find_vpx() {
                return Some(vpx);
            }
        }

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

        // Avvio simulatore Web nel browser (es. Game & Watch o web game)
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") || trimmed.starts_with("web://") {
            let url = trimmed.strip_prefix("web://").unwrap_or(trimmed);
            let script = format!(r#"Start-Process '{}'"#, url.replace('\'', "''"));
            Command::new("powershell")
                .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                .spawn()
                .map_err(|e| format!("Errore apertura browser: {}", e))?;
            return Ok(format!("Aperto simulatore web: {}", url));
        }

        // Avvio dedicato RetroArch
        if let Some(rest) = trimmed.strip_prefix("RETROARCH:") {
            let config = crate::config::RetroHubConfig::load();
            if let Some(ra_exe) = config.find_retroarch() {
                let parts: Vec<&str> = rest.splitn(2, ':').collect();
                let (core, rom) = if parts.len() == 2 {
                    (parts[0], parts[1])
                } else {
                    ("gw", parts[0])
                };
                let ra_dir = ra_exe.parent().unwrap_or(Path::new("."));
                let ra_escaped = ra_exe.to_string_lossy().replace('\'', "''");
                let rom_escaped = rom.replace('\'', "''");
                let core_dll = if core.ends_with(".dll") {
                    core.to_string()
                } else {
                    format!("{}_libretro.dll", core)
                };
                let script = format!(
                    r#"$p = Start-Process -FilePath '{}' -ArgumentList '-L "cores\{}" "{}"' -WorkingDirectory '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 500; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                    ra_escaped, core_dll, rom_escaped, ra_dir.to_string_lossy().replace('\'', "''")
                );
                Command::new("powershell")
                    .args(["-NoProfile", "-WindowStyle", "Hidden", "-Command", &script])
                    .spawn()
                    .map_err(|e| format!("Errore avvio RetroArch: {}", e))?;
                return Ok(format!("Avviato con RetroArch: {}", rom));
            } else {
                return Err("NOT_CONFIGURED:GAMEWATCH:RetroArch non configurato. Imposta il percorso dell'eseguibile RetroArch nelle Opzioni (⚙️).".to_string());
            }
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

        // Avvio nativo PC Recompilations con working directory dedicata
        if let Some(rest) = trimmed.strip_prefix("recomp://") {
            let recomp_roots = [
                PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\Recomp ROMs"),
                PathBuf::from(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\recomp"),
                PathBuf::from("roms/recomp"),
                PathBuf::from("recomp"),
            ];
            for root in &recomp_roots {
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
                        .map_err(|e| format!("Errore lancio Recomp: {}", e))?;
                    return Ok(format!("Avviato Recomp: {}", full.display()));
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

            let dl_mame = std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("downloads/mame");
            let rompaths = format!(r#"roms;{};{}"#, dl_mame.display(), mame_dir.join("roms").display());

            let mame_exe = mame_dir.join("mame.exe");
            let mut cmd = Command::new(&mame_exe);
            cmd.arg(rom.trim())
                .arg("-video")
                .arg("bgfx")
                .arg("-skip_gameinfo")
                .arg("-rompath")
                .arg(&rompaths)
                .current_dir(&mame_dir);

            cmd.spawn().map_err(|e| format!("Errore avvio MAME: {}", e))?;
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
                let dir = target_file.parent().unwrap_or(Path::new("."));
                let escaped = target_file.to_string_lossy().replace('\'', "''");
                let dir_escaped = dir.to_string_lossy().replace('\'', "''");
                let script = format!(
                    r#"$p = Start-Process -FilePath '{}' -WorkingDirectory '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 400; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                    escaped, dir_escaped
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
                } else if emu_escaped.to_lowercase().contains("vpinball") {
                    format!(r#"-play "{rom_escaped}""#)
                } else if emu_escaped.to_lowercase().contains("retroarch") {
                    let core_flag = if system_code.to_uppercase() == "GAMEWATCH" || system_code.to_uppercase() == "LCD" || ext == "mgw" {
                        r#"-L "cores\gw_libretro.dll" "#
                    } else {
                        ""
                    };
                    format!(r#"{core_flag}"{rom_escaped}""#)
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
            let dir = target_file.parent().unwrap_or(Path::new("."));
            let escaped = target_file.to_string_lossy().replace('\'', "''");
            let dir_escaped = dir.to_string_lossy().replace('\'', "''");
            let script = format!(
                r#"$p = Start-Process -FilePath '{}' -WorkingDirectory '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 400; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
                escaped, dir_escaped
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
