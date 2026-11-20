---
name: retrohub-launcher
description: Procedure di diagnostica per i moduli di lancio emulatori, gestione multi-monitor OSD, percorsi 8.3 Win32 e integrazione MAME, DOSBox, ScummVM e Daphne.
---

# RetroHub Launcher Diagnostics Skill

Questa skill definisce le procedure per testare e manutenere i moduli di lancio (`src/launcher.rs`), la configurazione degli emulatori (`src/config.rs`) e i parametri multi-monitor.

## 1. Topologia Multi-Monitor & OSD
RetroHub rileva dinamicamente i display tramite Win32 API (`EnumDisplayMonitors` e `GetMonitorInfoW`).
Per testare la numerazione fisica dei display e l'identificazione visiva a schermo intero:
```powershell
powershell -ExecutionPolicy Bypass -File "scripts/identify_displays.ps1"
```
Parametri MAME passati dinamicamente in `src/launcher.rs`:
```rust
-numscreens 1 -screen "{monitor_scelto}" -screen0 "{monitor_scelto}" -ui_mouse 0 -nomouse -skip_gameinfo
```

## 2. Percorsi Brevi DOSBox (8.3 Short Path)
Per evitare crash o problemi di spazi con percorsi contenenti caratteri speciali (es. `C:\Users\ClaudioDall'Ara\...`):
- Usare sempre la funzione helper `Launcher::to_short_path(path)` che invoca Win32 `GetShortPathNameW`.
- DOSBox Staging viene avviato con argomenti formattati:
  `-c "mount c <short_dir>" -c "c:" -c "<main_exe>"`

## 3. Scansione Directory Ricorsiva (ScummVM, C64, Console)
`Launcher::match_lnk_or_rom_in_dir` effettua una scansione a due livelli:
1. Radice della cartella emulatore (`dir`)
2. Sottocartelle canoniche: `dir/games/` e `dir/roms/`
Estensioni riconosciute: `lnk`, `bat`, `cmd`, `exe`, `chd`, `cue`, `iso`, `bin`, `d64`, `prg`, `crt`, `nes`, `sfc`, `smc`, `z64`, `n64`.
