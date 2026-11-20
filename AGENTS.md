# Regole e Linee Guida per Agenti — Progetto RetroHub

Questo documento contiene le direttive operative e architetturali obbligatorie per qualsiasi agente AI che collabora sullo sviluppo di **RetroHub** all'interno di questo workspace.

---

## 1. Architettura & Stack Tecnologico

- **Progetto**: RetroHub — Launcher e Enciclopedia Retrogaming Desktop per Windows (con porting futuro su Linux/SteamOS e macOS).
- **Backend**: Rust 2021 (motore nativo standalone < 10 MB, server HTTP integrato, memoria RAM < 25 MB).
- **Frontend**: Vanilla JS moderno (ES2022+) + HTML5 + CSS/Tailwind embedded direttamente in `src/ui.html`.
- **Database**: SQLite nativo (`retro_hub.db`) gestito tramite `rusqlite`.
- **Catalogo Cloud On-Demand**: `data/master_cloud_catalog.json` (1.699 titoli puliti 1G1R, senza BIOS né duplicati).
- **Configurazione**: `retrohub.json` (percorsi emulatori, multi-monitor, opzioni server).

---

## 2. Principi Fondamentali (Regole Auree)

1. **Paradigma 1G1R (Senza Duplicati)**:
   - 1 solo gioco per titolo mondiale. Mai introdurre cloni regionali (JAP/USA/EUR ridondanti), BIOS, o schede diagnostiche.
2. **Separazione Scheletro vs Muscolo (0-Byte su Disco)**:
   - I metadati e le copertine HD risiedono in locale per garantire 60 FPS offline.
   - I file ROM rimangono in cloud e vengono scaricati **esclusivamente su richiesta** al clic `▶ GIOCA`.
   - **MAI cancellare o alterare file utente** nella cartella `emu_root`.
3. **Sicurezza dei Comandi Windows & String Escaping**:
   - Mai usare concatenazioni di stringhe vulnerabili nei comandi shell.
   - Gestire sempre percorsi con spazi e caratteri speciali (es. apostrofi `ClaudioDall'Ara`) utilizzando `std::process::Command` nativo in Rust o percorsi brevi 8.3 (`to_short_path` tramite Win32 `GetShortPathNameW`).
4. **Codifica File**:
   - `data/master_cloud_catalog.json` e i file JSON possono avere UTF-8 BOM (`utf-8-sig`). Gestire sempre sia UTF-8 standard che BOM senza fallire.

---

## 3. Struttura del Catalogo & Macro-Famiglie

Il catalogo è strutturato su **12 Macro-Famiglie** canoniche:
1. `Arcade Coin-Op` (MAME, NEOGEO, NAOMI, ATOMISWAVE, HIKARU)
2. `Laserdisc & FMV` (HYPSEUS)
3. `Pinball & Flipper` (PINBALL)
4. `LCD & Game & Watch` (GAMEWATCH)
5. `Nintendo Universe` (NES, SNES, N64, GAMECUBE, GBA)
6. `Sega Universe` (MEGADRIVE, DREAMCAST)
7. `Sony PlayStation` (PSX)
8. `Atari & Vintage Pioneers` (ATARI2600, ATARI7800, INTELLIVISION, JAGUAR, LYNX, COLECO, VECTREX)
9. `SNK & NEC Engine` (NEOGEO, PC-Engine)
10. `Home Computer Occidentali` (C64, AMIGA, DOS, SCUMMVM)
11. `Nippon Micro Computers` (MSX, MSX2, X68000, PC88, PC98, FMTOWNS)
12. `PC Moderno, Recomp & Indie` (RECOMP, INDIE)

---

## 4. Flusso di Compilazione, Test & Rilascio

Quando modifichi codice Rust o asset:
1. Verifica compilazione: `cargo check` o `cargo build --release`.
2. Se crei o aggiorni feature:
   - Incrementa la versione semantica in `Cargo.toml` e `src/ui.html` (es. `v0.47.0` → `v0.47.1` o `v0.48.0`).
   - Compila l'eseguibile standalone finale: `retro-hub.exe`.
   - Verifica lo stato git prima del commit: `git status`.
   - Fai commit strutturati convenzionali (`feat:`, `fix:`, `refactor:`) e pusha sul branch attivo (`refactor-modular-rust`).

---

## 5. Strumenti e Skill di Progetto Disponibili

- `.agents/skills/retrohub-builder`: Compilazione ottimizzata, build LTO, semver bump e rilascio.
- `.agents/skills/retrohub-catalog`: Manutenzione catalogo, validazione 1G1R, sincronizzazione DB e scraping cover.
- `.agents/skills/retrohub-launcher`: Diagnostica emulatori, test OSD multi-monitor (`scripts/identify_displays.ps1`), DOSBox e ScummVM.
