# 🕹️ RetroHub (Rust Edition)

[![Rust Version](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011%20(x86__64)-blue.svg)]()
[![Status](https://img.shields.io/badge/status-active%20development%20v2.0-brightgreen.svg)]()

Client desktop nativo ultra-veloce scritto interamente in **Rust** per la gestione, consultazione multi-tag, sblocco enciclopedico delle copertine e lancio istantaneo dell'intera storia dei videogiochi (oltre **1.149 titoli** su oltre **16 piattaforme**, architettura predisposta a oltre 200 sistemi).

---

## ⚡ Caratteristiche Principali

* **Avvio Istantaneo (< 50ms):** Zero tempi di caricamento, consumo di CPU pari allo 0% in idle e footprint di memoria RAM inferiore a 30 MB.
* **Single Binary Executable (`retro-hub.exe`):** Tutta l'applicazione (micro-server HTTP, SQLite database, interfaccia Webview embedded, motore di ricerca fuzzy e netcode) compilata in un singolo file autonomo e portabile.
* **Enciclopedia Visiva Sbloccata al 100% (Zero-Disk Footprint):** Sfoglia tutte le copertine boxart in alta definizione, gli anni, i generi, le case produttrici e i comandi gamepad anche senza avere le ROM scaricate in anticipo.
* **Download ROM "On-The-Fly":** Clicchi su qualsiasi titolo e RetroHub ne scarica la ROM al volo in background, ne verifica l'integrità tramite checksum e lancia subito l'emulatore.
* **Prompt Salva-Spazio Post-Partita:** Alla chiusura del gioco, decidi con un click se conservare il titolo nei tuoi Preferiti locali o eliminarlo per liberare spazio su disco.
* **Doppio Binario Emulatori & Core (Stile Batocera / RetroBat):**
  * **RetroArch Libretro Cores (Invisibile / Headless):** Gestione unificata e automatica per l'80% delle console classiche vintage (NES, SNES, Genesis, Game Boy, Atari, C64).
  * **Emulatori Standalone Dedicati:** MAME nativo con BGFX, Dolphin (GameCube/Wii), Flycast (Dreamcast/Naomi), DuckStation (PS1), PCSX2 (PS2), Hypseus (LaserDisc), Recomp Native PC.
* **Ricerca Fuzzy Istantanea con Operatori Booleani:** Punteggio pesato Jaro-Winkler e distanza Levenshtein, supporto frasi esatte tra virgolette (`"..."`), esclusione (`-` / `NOT`), OR (`|`), filtri per campo (`genre:`, `dev:`, `sys:`) e intervalli di anni (`1990..1995`).
* **Supporto Joypad Nativo:** Navigazione da salotto ("10-foot UI") ottimizzata per controller Xbox e adattatori PlayStation USB.

---

## 🎮 Piattaforme Supportate

* **MAME Arcade** (596 titoli classici da sala giochi)
* **LaserDisc Arcade / Hypseus Singe** (Dragon's Lair, Space Ace, Time Warp...)
* **Recomp Native PC** (45 progetti nativi a 60/120 FPS: Zelda OoT, Mario 64, OutRun, Forza Horizon Xbox 360...)
* **Commodore 64 & Amiga** (Collezione C64 con autostart warp ultra-veloce)
* **MS-DOS PC** (100 classici calibrati per controller)
* **ScummVM** (64 avventure grafiche in italiano)
* **Sega NAOMI & Sammy Atomiswave** (Flycast)
* **Sega Hikaru & Model Arcade** (Planet Harriers, Brave Firefighters...)
* **Super Nintendo, Nintendo 64, GameCube, Dreamcast, PlayStation 1, NES**
* **Giochi Indie PC & LocoMalito** (33 giochi)
* *Predisposto per l'integrazione modulare di oltre 200 sistemi (Standard Batocera & RetroBat)*

---

## 🛠️ Compilazione e Sviluppo

### Prerequisiti
* [Rust & Cargo](https://rustup.rs/) (Edizione 2021 raccomandata)
* Windows 10 o 11 (64-bit)

### Compilazione Release Ottimizzata
```bash
cargo build --release
```
L'eseguibile ultra-compatto verrà generato in `target/release/retro-hub.exe`.

### Esecuzione Test Unitari
```bash
cargo test
```

---

## 📚 Documentazione del Progetto

* 📖 **[Manuale Utente Ufficiale](docs/MANUAL.md):** Guida all'uso dell'interfaccia, controlli joypad, sintassi di ricerca e netplay.
* 🏗️ **[Specifiche Architetturali e Tecniche](docs/ARCHITETTURA_TECNICA.md):** Analisi dettagliata dell'architettura a doppio binario, API REST, standardizzazione filesystem e portabilità.
* 🗺️ **[Roadmap di Sviluppo](ROADMAP.md):** Fasi concordate, progressi in tempo reale e priorità operative.
* 🔍 **[Report Strategico di Revisione](docs/analisi-progetto/ANALISI_PROGETTO_ROADMAP_UX.md):** Analisi da Project Manager su debiti tecnici, UX/UI e deliverable.
