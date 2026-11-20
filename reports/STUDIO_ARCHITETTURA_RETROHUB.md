# 📊 Studio Architettura RetroHub - Report Completo
**Data:** 20 Novembre 2026  
**Versione:** v0.47.0  
**Autore:** Studio Sistematico  

---

## 📌 Executive Summary

RetroHub è un launcher per retrogaming scritto interamente in **Rust**, progettato per gestire **1.699 giochi** su **35 piattaforme**. L'architettura utilizza un server HTTP locale (tiny_http) per servire un'interfaccia web embedded.

---

## 📊 Statistiche di Base

### 📁 Struttura del Progetto
- **File Rust:** 32 file
- **File Documentazione:** 19 file MD
- **File JSON Configurazione:** 9 file
- **Lines of Code:** ~7,413 righe

### 📊 Database
- **Nome File:** retro_hub.db
- **Dimensione:** 0.78 MB
- **Giochi Registrati:** 1,699
- **Piattaforme:** 35

### 🔌 API Endpoints
- **GET:** 17 endpoint
- **POST:** 23 endpoint
- **Totale:** 40 endpoint

---

## 🧩 Architettura Moduli

### 📍 File Rust Principali (per dimensione)

| File | Dimensione | Linee | Descrizione |
|------|------------|-------|-------------|
| catalog.rs | 80.5 KB | 1,378 | Gestione catalogo giochi 1G1R |
| server.rs | 68.6 KB | 1,275 | Server API HTTP + routing |
| launcher.rs | 39.1 KB | 789 | Lanciatore emulatori |
| search.rs | 25.7 KB | 704 | Motore ricerca fuzzy |
| db.rs | 18.1 KB | 544 | DAO database SQLite |
| netplay.rs | 12.5 KB | 369 | Logiche multiplayer P2P |
| config.rs | 12.4 KB | 424 | Configurazione utente |
| main.rs | 5.6 KB | 161 | Entry point applicazione |

### 🧩 Moduli Principali

```
src/
├── main.rs                    # Entry point, inizializzazione
├── config.rs                  # Configurazione utente e emulatori
├── models.rs                  # Strutture dati Game, Platform, ecc.
├── db.rs                      # Operazioni database SQLite
├── server.rs                  # Server HTTP e API endpoints
├── launcher.rs                # Lancio giochi e emulatori
├── search.rs                  # Ricerca fuzzy Jaro-Winkler
├── catalog.rs                 # Catalogo 1G1R e sincronizzazione
├── netplay.rs                 # Multiplayer connettersi P2P
├── lobby.rs                   # Gestione stanze lobby
├── download_manager.rs        # Download ROM on-demand
├── downloader.rs              # Downloader file
├── rom_scanner.rs             # Scanner ROM
├── master_catalog.rs          # Catalogo cloud
├── library_config.rs          # Config libreria
├── platform_wizard.rs         # Wizard setup utente
├── core.rs                    # Core utilities
├── search.rs                  # Motore ricerca
├── netplay.rs                 # Network play
├── catalog.rs                 # Catalog management
├── mame_rom_map.json          # Mapping ROM MAME
├── duckstation.rs             # DuckStation wrapper
├── flycast.rs                 # Flycast wrapper
├── dolphin.rs                 # Dolphin wrapper
└── downloader.rs              # Download manager
```

---

## 🎮 Top Piattaforme per Volume

| Posizione | Piattaforma | Giochi | % del Catalogo |
|-----------|-------------|--------|----------------|
| 1 | MAME | 568 | 33.4% |
| 2 | Commodore 64 | 223 | 13.1% |
| 3 | Neo Geo | 120 | 7.1% |
| 4 | DOS | 100 | 5.9% |
| 5 | SNES | 86 | 5.1% |
| 6 | ScummVM | 64 | 3.8% |
| 7 | PlayStation 1 | 60 | 3.5% |
| 8 | Mega Drive | 58 | 3.4% |
| 9 | NES | 54 | 3.2% |
| 10 | GBA | 51 | 3.0% |

---

## 🔌 API Endpoints Dettagliati

### 🎮 Game Management
- `GET /api/games` - Lista giochi con filtri
- `POST /api/launch` - Lanciaggio gioco
- `POST /api/favorite` - Toggle preferito
- `GET /api/stats` - Statistiche utente

### 🏛️ Library Management
- `GET /api/library/config` - Config libreria
- `POST /api/library/config` - Aggiorna config
- `POST /api/library/scan` - Scansione ROM
- `POST /api/library/download` - Download ROM
- `POST /api/library/delete` - Delete ROM

### 🌐 Network & Multiplayer
- `POST /api/netplay/create` - Crea stanza
- `POST /api/netplay/join` - Unisci stanza
- `GET /api/netplay/status` - Stato sessione
- `POST /api/netplay/start` - Inizia partita
- `POST /api/netplay/leave` - Lascia stanza
- `GET /api/lobby/rooms` - Lista stanze pubbliche
- `POST /api/lobby/create` - Crea stanza lobby
- `POST /api/lobby/join` - Unisci stanza lobby

### ⚙️ Configurazione
- `GET /api/config/emulators` - Stato emulatori
- `POST /api/config/emulators` - Salva percorsi
- `POST /api/config/emulators/detect` - Auto-detect

### 🖥️ Sistema
- `GET /` - UI principale
- `GET /api/platforms` - Lista piattaforme
- `GET /api/genres` - Lista generi
- `GET /api/developers` - Lista developer
- `GET /api/decades` - Lista decadi

### 📸 Cover Art
- `GET /api/cover/file/...` - Servizio copertine
- `GET /api/cover/mame/...` - Copertine MAME
- `GET /api/cover/naomi/...` - Copertine NAOMI

### 📥 Download
- `GET /api/emulators/install-vpx` - Install VPX
- `GET /api/emulators/install-status` - Stato download

### ⚠️ Note critiche
- **Hardcoded paths**: molti percorsi hardcodati in launcher.rs, server.rs
- **PowerShell**: utilizzo frequente per lancio giochi
- **Timeout server**: 300 secondi (5 min) per heartbeat

---

## 🗃️ Catalogo Dati

### 📋 File Principali
- **Database:** retro_hub.db (0.78 MB)
- **Catalogo Cloud:** data/master_cloud_catalog.json (0.98 MB)
- **Giochi:** 1,699 (0 duplicati, 0 BIOS)

### 📊 Metadati per Gioco (media)
- **Tag per gioco:** 10.3
- **Metadati completi:** 13 campi obbligatori

**Campi obbligatori:**
1. id
2. title
3. platform
4. system_code
5. year
6. genre
7. developer
8. publisher
9. controls
10. launch_target
11. cover_url
12. is_favorite
13. play_count
14. last_played_at

---

## 🏗️ Architettura Tecnica

### ✅ Strengths
1. **Mono-binario Rust:** <10MB, veloce avvio
2. **Database SQLite:** persistenza local, queries efficienti
3. **Ricerca fuzzy:** Jaro-Winkler per ricerca flessibile
4. **1G1R strategy:** nessun duplicato regionale
5. **Multi-monitor:** rilevamento display Windows API

### ⚠️ Issues Critici
1. **Hardcoded paths:** percentuale elevata di percorsi hardcodati
2. **PowerShell dependency:** fallback lento per lancio giochi
3. **Configurazione limitata:** mancano opzioni avanzate
4. **UI monolitica:** ui.html ~4000 righe

---

## 📋 Raccomandazioni

### Priorità Alta
1. Estrarre hardcoded paths in configurazione
2. Implementare CLI per argomenti riga di comando
3. Separare UI in file CSS/JS dedicati

### Priorità Media
1. Aggiungere logging strutturato
2. Migliorare gestione errori
3. Documentazione contribution guide

### Priorità Bassa
1. Supporto multipla piattaforme (Linux/macOS)
2. Plugin system per emulatori
3. Theme personalizzabili

---

**Fine dello studio**