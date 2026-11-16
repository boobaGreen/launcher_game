# 🕹️ RetroHub (Rust)

Client nativo ultra-veloce scritto in **Rust** per la gestione, consultazione multi-tag e lancio della collezione retrogaming unificata (1.149+ titoli).

## 🚀 Piattaforme Supportate
- **MAME Arcade** (596 giochi)
- **LaserDisc Arcade / Hypseus Singe** (Dragon's Lair, Space Ace, Time Warp...)
- **Recomp Native PC** (45 progetti a 60/120 FPS: Zelda, Mario 64, OutRun, Forza Horizon Xbox 360...)
- **Commodore 64** (224 giochi con autostart warp ultra-veloce)
- **MS-DOS** (100 classici calibrati per controller Xbox)
- **ScummVM** (64 avventure grafiche in italiano)
- **Sega NAOMI & Sammy Atomiswave** (Flycast)
- **Sega Hikaru** (DEmul: Planet Harriers, Brave Firefighters...)
- **Super Nintendo, Nintendo 64, GameCube, Sega Dreamcast, PlayStation, NES**
- **LocoMalito & Giochi Indie PC** (33 giochi)

## ⚡ Caratteristiche
- **Ricerca Fuzzy Istantanea**: Tolleranza agli errori di battitura (Levenshtein) e scoring ponderato.
- **Filtri Multi-Tag Avanzati**: Intersezione booleana (Piattaforma, Genere, Decade/Anno, Tipo Controller).
- **Integrazione Copertine**: Supporto per ScreenScraper.fr e Libretro Thumbnails.
- **Supporto Pad Xbox**: Navigazione completa tramite gamepad e avvio con un tasto.
- **Single Binary**: Compilabile in un singolo file `.exe` da ~15 MB, zero dipendenze e avvio in 0.1s.

## 🛠️ Compilazione
```bash
cargo build --release
```
L'eseguibile ottimizzato sarà generato in `target/release/retro-hub.exe`.
