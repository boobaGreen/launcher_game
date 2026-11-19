# 🏗️ RetroHub — Specifiche Architetturali e Tecniche di Dettaglio
**Autore:** Claudio Dall'Ara & Lead Architecture Team  
**Versione Documento:** 2.0.0 (Architettura Modulare a Doppio Binario & On-Demand)  
**Target Iniziale:** Windows 10 / 11 (x86_64) — *Progettato per portabilità nativa Linux e macOS*

---

## 📌 1. Visione Generale & Principi Architetturali

**RetroHub** è un launcher desktop ad altissime prestazioni per collezioni retrogaming ed arcade, sviluppato in **Rust**.
A differenza dei frontend tradizionali (come RetroBat o Batocera) che si basano su framework pesanti in Python/C++ con lunghi tempi di caricamento, RetroHub persegue 5 principi non negoziabili:

1. **Reattività Istantanea (< 50ms):** Avvio del processo a 0 secondi, footprint RAM inferiore a 30 MB in idle, zero garbage collection.
2. **Single Binary Executable (`retro-hub.exe`):** Tutta la logica di business, il server HTTP locale, il motore di ricerca fuzzy, il database SQLite e l'interfaccia embedded risiedono in un unico binario compatto.
3. **Enciclopedia Visiva Sbloccata al 100% (Zero-Disk Footprint):** L'utente può sfogliare l'intero archivio storico mondiale (copertine 3D, generi, anni, sviluppatori, comandi gamepad) per ciascuna console selezionata senza dover possedere in anticipo le ROM sul disco.
4. **Download Trasparente "On-The-Fly" & Dialog Salva-Spazio:** Download della ROM al click con barra di avanzamento, avvio immediato a schermo intero e prompt post-partita per decidere se conservare il gioco o eliminare il file per mantenere lo storage pulito.
5. **Doppio Binario per Emulatori & Core (Ibrido Flessibile):**
   * **RetroArch Libretro Cores (Headless / Invisibile):** Un unico gestore universale a riga di comando per coprire l'80% delle console vintage a 8, 16 e 32 bit senza installare 40 programmi diversi.
   * **Emulatori Standalone Ottimizzati:** Esecutori dedicati per le piattaforme 3D complesse e l'arcade pesante (MAME nativo BGFX, Dolphin per GameCube/Wii, Flycast per Dreamcast/Naomi, DuckStation per PS1, PCSX2 per PS2, Hypseus per LaserDisc, Recomp Native PC).

---

## 🏛️ 2. Mappa Architetturale dei Componenti

```
                                    ┌─────────────────────────────────────────────────────────────┐
                                    │                     RETROHUB APPLICATION                    │
                                    │               (Rust Native Process: PID xxxx)               │
                                    └──────────────────────────────┬──────────────────────────────┘
                                                                   │
                                ┌──────────────────────────────────┴──────────────────────────────────┐
                                ▼                                                                     ▼
                 ┌─────────────────────────────┐                                       ┌─────────────────────────────┐
                 │     FRONTEND PRESENTATION   │                                       │       BACKEND SERVICES      │
                 │   Webview Dedicated Window  │                                       │       tiny_http :39281      │
                 ├─────────────────────────────┤                                       ├─────────────────────────────┤
                 │ • Dark Mode Cyberpunk/Neon  │                                       │ • REST Controller API       │
                 │ • Hero Spotlight Carousel   │◀═════════ HTTP REST (JSON) ══════════▶│ • SQLite Database Engine    │
                 │ • Virtual Scroll Grid       │           Async Fetch                 │ • Jaro-Winkler Fuzzy Search │
                 │ • Gamepad Navigator (XInput)│                                       │ • On-Demand Downloader      │
                 │ • Post-Game Save/Del Dialog │                                       │ • Process Lifecycle Manager │
                 └─────────────────────────────┘                                       └──────────────┬──────────────┘
                                                                                                      │
                                                       ┌──────────────────────────────────────────────┴──────────────────────────────┐
                                                       ▼                                                                             ▼
                                        ┌──────────────────────────────┐                                              ┌──────────────────────────────┐
                                        │  LIBRETRO RUNTIME DISPATCHER │                                              │ STANDALONE EMULATOR PLUGINS  │
                                        │    (RetroArch Headless CLI)  │                                              │      (Dedicated Engines)     │
                                        ├──────────────────────────────┤                                              ├──────────────────────────────┤
                                        │ • NES   -> mesen / fceumm    │                                              │ • MAME Native Arcade (BGFX)  │
                                        │ • SNES  -> snes9x / bsnes    │                                              │ • Dolphin (GC / Wii)         │
                                        │ • MD    -> genesis_plus_gx   │                                              │ • Flycast (Dreamcast/Naomi)  │
                                        │ • GB/A  -> mgba / gambatte   │                                              │ • DuckStation (PS1 1080p/4K) │
                                        │ • C64   -> vice_x64sc        │                                              │ • Hypseus Singe (LaserDisc)  │
                                        │ • PCE   -> beetle_pce_fast   │                                              │ • PCSX2 (PlayStation 2)      │
                                        │ • ATARI -> stella / proys    │                                              │ • PC Recomp (Zelda, SM64...) │
                                        └──────────────────────────────┘                                              └──────────────────────────────┘
```

---

## 📁 3. Standardizzazione del Filesystem Portabile

Per eliminare ogni traccia di hardcoding e garantire la portabilità su qualsiasi PC (e in futuro su Linux e macOS), RetroHub adotta la seguente gerarchia relativa standard:

```
RetroHub/
├── retro-hub.exe                   # Singolo binario eseguibile compilato in Rust
├── retrohub.json                   # File di configurazione utente (portabile)
├── retro_hub.db                    # Database SQLite locale con indici di ricerca
│
├── cores/                          # Core Libretro scaricati on-demand (.dll / .dylib / .so)
│   ├── snes9x_libretro.dll
│   ├── genesis_plus_gx_libretro.dll
│   ├── mgba_libretro.dll
│   └── vice_x64sc_libretro.dll
│
├── emulators/                      # Emulatori Standalone (gestiti o auto-scaricati)
│   ├── retroarch/                  # Runtime RetroArch headless minimale
│   │   └── retroarch.exe
│   ├── mame/                       # MAME Arcade ottimizzato
│   │   └── mame.exe
│   ├── dolphin/                    # Dolphin GameCube / Wii
│   │   └── Dolphin.exe
│   ├── duckstation/                # DuckStation PS1
│   │   └── duckstation-qt-x64-ReleaseLTCG.exe
│   └── flycast/                    # Flycast Dreamcast / Naomi
│       └── flycast.exe
│
├── roms/                           # Directory universale delle ROM (Standard Batocera/RetroBat)
│   ├── snes/                       # es. Super Mario World.sfc
│   ├── megadrive/                  # es. Sonic The Hedgehog.md
│   ├── nes/                        # es. Super Mario Bros.nes
│   ├── c64/                        # es. Turrican.d64
│   ├── mame/                       # es. mslug.zip
│   ├── psx/                        # es. Tekken 3.chd
│   └── dreamcast/                  # es. Crazy Taxi.chd
│
├── downloads/                      # Buffer per download on-the-fly temporanei
│   └── temp/
│
└── cache/                          # Cache locale persistente
    ├── covers/                     # Copertine boxart memorizzate (150x210 o HD)
    │   ├── snes/
    │   ├── mame/
    │   └── c64/
    └── metadata/                   # Manifest e indici sincronizzati
```

---

## ⚡ 4. Specifica Dettagliata del Flusso: Enciclopedia + On-Demand

### 4.1 Livello 1: Consultazione Enciclopedica (Zero-Disk Footprint)
1. All'avvio o alla selezione di una piattaforma, il client interroga l'endpoint `/api/games?platform=SNES`.
2. Il database locale SQLite (o il manifest di sistema) restituisce tutti i titoli storici associati (es. 200+ giochi per SNES), ciascuno corredato da:
   * Titolo pulito e titolo alternativo
   * Anno, Genere, Produttore, Numero giocatori
   * URL copertina remota (Libretro CDN, Steam CDN o IGDB)
   * Stato locale: `is_installed: true/false` (determinato verificando se la ROM esiste nella cartella `roms/<platform>/`).
3. L'interfaccia Webview renderizza le card nella griglia:
   * **Se installato:** Badge verde `DISPONIBILE` e pulsante **`▶ GIOCA`**.
   * **Se non installato:** Badge ciano `ON-DEMAND` e pulsante **`☁️ SCARICA & GIOCA`**.

### 4.2 Livello 2: Download ROM "On-The-Fly"
Quando l'utente clicca **`☁️ SCARICA & GIOCA`**:
1. La UI chiama in POST `/api/library/download` con `{ "game_id": 123 }`.
2. Il backend Rust (`download_manager.rs`):
   * Risolve l'URL CDN verificato per la ROM corrispondente.
   * Scarica il file in streaming asincrono dentro `downloads/temp/`.
   * Verifica l'integrità del file scaricato (calcolo checksum SHA1 o CRC32).
   * Sposta il file validato nella cartella finale di destinazione `roms/<platform>/<titolo>.<ext>`.
   * Aggiorna lo stato del gioco nel database SQLite (`is_installed = 1`, `launch_target = "roms/..."`).
3. La UI mostra un feedback visivo immediato (progress bar / spinner) e, alla ricezione del segnale di completamento, chiama automaticamente `/api/launch`.

### 4.3 Livello 3: Esecuzione del Gioco (Doppio Binario)
Il backend riceve `/api/launch` e attiva il dispatcher:
* **Caso A (Console Classica con Core Libretro):**
  RetroHub verifica la presenza di `cores/<core_name>.dll` e `emulators/retroarch/retroarch.exe`.
  * Se mancano: vengono scaricati al primo avvio in <3 secondi dal buildbot Libretro.
  * Esecuzione comando nativo:
    ```cmd
    emulators\retroarch\retroarch.exe -L cores\snes9x_libretro.dll "roms\snes\Super Mario World.sfc" -f
    ```
* **Caso B (Emulatore Standalone Dedicato):**
  RetroHub invoca l'eseguibile dedicato con i suoi argomenti specifici (es. `mame.exe rom -video bgfx`, `Dolphin.exe -b -e "rom.iso"`, `duckstation.exe -batch "rom.chd"`).
* **Tracciamento:**
  RetroHub attende la chiusura del processo del gioco (tramite `Child::wait()` o thread di sorveglianza), incrementa il contatore `play_count` e aggiorna `last_played_at`.

### 4.4 Livello 4: Dialog Salva-Spazio Post-Partita
Non appena l'emulatore termina:
1. RetroHub riprende il focus in primo piano.
2. Se il gioco era stato scaricato on-demand e non è contrassegnato come preferito ⭐:
   Viene mostrata una finestra di dialogo modale ad alto contrasto:
   ```
   ┌──────────────────────────────────────────────────────────┐
   │ 🎮 Partita a "The Legend of Zelda" conclusa!             │
   │                                                          │
   │ La ROM occupa 32 MB di spazio. Cosa vuoi fare?           │
   │                                                          │
   │  [ 💾 CONSERVA NEL PC ]     [ 🗑️ ELIMINA PER FARE SPAZIO ]│
   │                                                          │
   │  ☐ Ricorda questa scelta per tutti i giochi futuri       │
   └──────────────────────────────────────────────────────────┘
   ```
3. Se l'utente sceglie **ELIMINA**, la UI invia una chiamata POST a `/api/library/delete`:
   * Il file viene cancellato dal disco.
   * Il database reimposta `is_installed = 0`.
   * Lo spazio su disco torna immediatamente libero.

---

## 🧩 5. Specifiche dei Core Libretro & Mapping Piattaforme

Per supportare l'intero archivio di oltre 200 console senza disperdere risorse, la tabella seguente definisce l'associazione ufficiale tra codice sistema, core raccomandato e URL di download dal buildbot Libretro:

| Sistema | Codice Cartella | Core Libretro Raccomandato | File Core Windows (DLL) | File Core Linux (SO) | File Core macOS (DYLIB) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Super Nintendo** | `snes` | Snes9x | `snes9x_libretro.dll` | `snes9x_libretro.so` | `snes9x_libretro.dylib` |
| **Nintendo NES** | `nes` | Mesen / FCEUMM | `fceumm_libretro.dll` | `fceumm_libretro.so` | `fceumm_libretro.dylib` |
| **Game Boy / GBC** | `gb` / `gbc` | Gambatte | `gambatte_libretro.dll` | `gambatte_libretro.so` | `gambatte_libretro.dylib` |
| **Game Boy Advance** | `gba` | mGBA | `mgba_libretro.dll` | `mgba_libretro.so` | `mgba_libretro.dylib` |
| **Sega Mega Drive** | `megadrive` | Genesis Plus GX | `genesis_plus_gx_libretro.dll` | `genesis_plus_gx_libretro.so` | `genesis_plus_gx_libretro.dylib` |
| **Sega Master System** | `mastersystem` | Genesis Plus GX | `genesis_plus_gx_libretro.dll` | `genesis_plus_gx_libretro.so` | `genesis_plus_gx_libretro.dylib` |
| **Commodore 64** | `c64` | VICE x64sc | `vice_x64sc_libretro.dll` | `vice_x64sc_libretro.so` | `vice_x64sc_libretro.dylib` |
| **Amiga (OCS/AGA)** | `amiga` | PUAE | `puae_libretro.dll` | `puae_libretro.so` | `puae_libretro.dylib` |
| **Atari 2600** | `atari2600` | Stella | `stella_libretro.dll` | `stella_libretro.so` | `stella_libretro.dylib` |
| **Atari 7800** | `atari7800` | ProSystem | `prosystem_libretro.dll` | `prosystem_libretro.so` | `prosystem_libretro.dylib` |
| **PC Engine / TG-16** | `pcengine` | Beetle PCE Fast | `mednafen_pce_fast_libretro.dll` | `mednafen_pce_fast_libretro.so` | `mednafen_pce_fast_libretro.dylib` |
| **Neo Geo Pocket** | `ngp` / `ngpc` | Beetle NeoPop | `mednafen_ngp_libretro.dll` | `mednafen_ngp_libretro.so` | `mednafen_ngp_libretro.dylib` |
| **Bandai WonderSwan** | `wonderswan` | Beetle Cygne | `mednafen_wswan_libretro.dll` | `mednafen_wswan_libretro.so` | `mednafen_wswan_libretro.dylib` |

*Endpoint Ufficiale Buildbot:*  
`https://buildbot.libretro.com/nightly/windows/x86_64/latest/{core_name}.dll.zip`  
*(Compatibile con architettura automatica per macOS e Linux).*

---

## ⚡ 5.1 Il Terzo Binario: Gestione dei Port Nativi De-compilati (PC Recomp)

Accanto ai **Core Libretro** (Binario 1 per 8/16-bit) e agli **Emulatori Standalone Dedicati** (Binario 2 per 3D pesante), RetroHub introduce un'innovazione architetturale esclusiva rispetto a Batocera e RetroBat: il **Terzo Binario Nativo PC (Recomp / Decomp)**.

### A. Che cos'è un Recomp e perché è Rivoluzionario
Un Recomp non è un gioco emulato: è il codice sorgente originale disassemblato o de-compilato e ricompilato nativamente come eseguibile a 64 bit (x86_64 su Windows/Linux, Apple Silicon su macOS).
* **Vantaggi Tecnici:** Zero overhead di CPU, framerate nativo sbloccato a 60/120/144 FPS, rendering moderno via DirectX 11/12, Vulkan o Metal, supporto display Ultra-Wide (21:9 e 32:9), zero input-lag e supporto texture HD/Ray Tracing.
* **Esempi nel Progetto:** *Ship of Harkinian* (Zelda OoT), *2 Ship 2 Harkinian* (Zelda MM), *CannonBall DX* (OutRun Arcade), *OpenGOAL* (Jak & Daxter), *Mario 64 PC Port*, *Namco System 22 Decompile* (Ridge Racer 2, Rave Racer, Time Crisis), *SegaSonic Native*, *KOF '98 Recomp*.

### B. Risoluzione delle 4 Sfide di Omogeneità UI/UX
Per far convivere i Recomp in modo perfettamente uniforme con i giochi emulati classici, RetroHub adotta le seguenti soluzioni:

1. **Tag & Metadati Duali (Doppia Appartenenza):**
   * Nel database, un Recomp possiede:
     * `system_code = "RECOMP"` (per raggrupparli nella categoria "PC Recomp Nativo").
     * `original_system = "N64" | "ARCADE" | "PS1" | "SATURN"` (per visualizzarli anche all'interno della console di appartenenza con un badge dorato `4K RECOMP`).
   * Tag arricchiti dedicati: `Engine` (es. *CannonBall*, *OpenGOAL*, *N64Recomp*), `Graphics API` (*Vulkan*, *DirectX 12*), `Features` (*144 FPS, Ultrawide, Modding*).
2. **Immagini & Copertine Uniformi:**
   * Condividono le medesime proporzioni boxart verticali (300x420 px) del catalogo storico originale Libretro o le Steam vertical capsule ad alta definizione (600x900 px).
   * Badge visivo elegante in alto a destra: `⚡ PC NATIVE`.
3. **Gestione Asset & ROM Originale (Verifica SHA256):**
   * Per rispetto del copyright, i Recomp richiedono i file audio/grafica estratti dalla ROM originale.
   * RetroHub sfrutta il file `manifest.json` e `copie-verificate-sha256.json`:
     * Se gli asset estratti sono già presenti (es. `game.otr` o cartella `data/`): avvio immediato.
     * Se mancano: RetroHub scarica la ROM originale verificata e lancia silenziosamente il tool di asset extraction, rendendo il gioco pronto in meno di 3 secondi!
4. **Ciclo di Lancio Nativo:**
   * Il launcher riconosce il tipo Recomp ed esegue direttamente il processo `.exe` con parametri grafici nativi, senza passare da emulatori intermedi, forzando la working directory locale e il focus in primo piano.

---

## 🎮 5.2 Il Quarto Binario: PC Indie, LocoMalito & Fan Games Nativi

Oltre all'emulazione storica e ai Recomp, RetroHub integra in modo nativo e omogeneo il mondo del **Neo-Retro PC, dei capolavori Indie e dei Fan Games/Remake**.

### A. I Due Grandi Sottoinsiemi di Titoli PC
Dall'analisi del catalogo reale (33 titoli censiti), emergono due categorie distinte:
1. **Freeware & Fan Games Portatili (Download On-Demand Legale):**
   * *LocoMalito:* Maldita Castilla, Hydorah, The Curse of Issyos, Darkula, 8bit Killer, L'Abbaye des Morts.
   * *Fan Remakes & Classici Freeware:* AM2R (Another Metroid 2 Remake), Streets of Rage Remake v5.2, Cave Story (Doukutsu Monogatari), Spelunky Classic, Iji, Hero Core, An Untitled Story, Knytt Stories, Within a Deep Forest.
   * *Caratteristiche:* Pesi contenuti (2.4 MB – 88 MB, eccezione SoRR 600 MB), 100% legali da distribuire, eseguibili portatili no-installer, avvio plug & play.
2. **Capolavori Commerciali Indie (GOG / Steam / DRM-Free):**
   * *Titoli:* Hollow Knight (7.6 GB), Celeste (1.1 GB), The Messenger (1.1 GB), Hotline Miami (1 GB), Huntdown (700 MB), Broforce (527 MB), UFO 50 (473 MB), FEZ (429 MB), Super Meat Boy (262 MB), Katana Zero (97 MB), Blazing Chrome (116 MB), VVVVVV (69 MB).
   * *Caratteristiche:* Pesi medio-alti, soggetti a copyright commerciale; richiedono il collegamento della copia locale dell'utente o il reindirizzamento allo store ufficiale.

### B. Risoluzione dei 5 Casi Limite per la Perfetta Omogeneità

1. **Il Problema del Multi-EXE (Evitare i Falsi Positivi):**
   * *Problema:* Una cartella indie contiene spesso `unins000.exe`, `vcredist_x86.exe`, `dxwebsetup.exe`, `UnityCrashHandler64.exe` o tool ausiliari (es. `SorMaker.exe` in SoRR, `DoConfig.exe` in Cave Story).
   * *Soluzione:*
     - **Blacklist Globale Automatica:** Ignora sempre file che contengono `unins*`, `*setup*`, `*redist*`, `*crash*`, `*dotNet*`, `*manager*`.
     - **Manifest Curato (`indie_manifest.json`):** Mappa puntuale dell'eseguibile primario per i titoli noti (es. `Doukutsu.exe` per Cave Story, con menu secondario contestuale `[ ⚙️ Configura ]` per `DoConfig.exe`).
     - **Working Directory Obbligatoria:** Il launcher imposta sempre `current_dir` alla cartella del gioco, condizione vitale affinché DLL, file `data.win` e salvataggi funzionino.

2. **Tassonomia e Navigazione UI:**
   * Codice Piattaforma Unificato: `pc_indie` (Colore Sociale: Viola Neon `#a371f7` o Corallo `#ff7b72`).
   * **Tag Pills Filtrabili nella UI:**
     - `#locomalito` (filtra la collezione completa dell'autore spagnolo)
     - `#fan-remake` (AM2R, SoRR, Cave Story)
     - `#commercial-indie` (Celeste, Hollow Knight, UFO 50)
     - `#pixel-8bit` / `#pixel-16bit` (per stile visivo)

3. **Boxart Standardizzate & Vertical Capsules:**
   * Per i titoli commerciali/Steam: Utilizzo automatico della CDN ufficiale Steam Vertical Capsule (formato 2:3 a 600x900 px): `https://cdn.akamai.steamstatic.com/steam/apps/<appid>/library_600x900.jpg`.
   * Per LocoMalito & Freeware: Boxart verticali ufficiali dell'autore o fornite localmente in `covers/pc_indie/<game_id>.webp`.
   * Modalità Placeholder Tipografica (Caso 3 Low-Storage): Nessun download di immagini; card vettoriale gradient con nome del titolo e tag di genere.

4. **On-Demand & Download Storage:**
   * *Freeware:* Download immediato dello zip pre-estratto in `roms/indie/<Titolo>/` con barra di avanzamento e lancio istantaneo.
   * *Commerciali Non Posseduti:* Mostrati in catalogo con metadati e recensione, ma con badge `[ 🛒 Acquista su Steam/GOG ]` e pulsante `[ 📂 Collega Cartella Locale ]` per mappare l'installazione locale dell'utente.

5. **Portabilità Cross-Platform (Windows ➔ Linux / Steam Deck ➔ macOS):**
   * Astrazione del launcher tramite `GameRunner::NativeExecutable`:
     - **Windows:** Processo nativo diretto (`Command::new`).
     - **Linux / Steam Deck:** Riconoscimento automatico: se esiste binario ELF nativo lancia nativo, altrimenti inoltro trasparente tramite `proton run` o `wine`.
     - **macOS:** Binario `.app` nativo o wrapper Wine/Whisky.

---

## 🌐 6. Specifiche API REST Backend (`server.rs`)

| Metodo | Endpoint | Parametri | Descrizione |
| :--- | :--- | :--- | :--- |
| `GET` | `/api/games` | `platform`, `query`, `genre`, `installed_only` | Restituisce la lista filtrata e ordinata dei giochi con punteggio fuzzy. |
| `GET` | `/api/platforms` | - | Restituisce l'elenco delle piattaforme con conteggio totale e giochi installati. |
| `POST` | `/api/launch` | `{ "game_id": 123 }` | Lancia il gioco identificato (avvio standalone o RetroArch core). |
| `POST` | `/api/favorite` | `{ "game_id": 123 }` | Attiva o disattiva lo stato di Preferito ⭐ nel database locale. |
| `GET` | `/api/library/config`| - | Restituisce la configurazione della libreria (percorsi, preferenze salva-spazio). |
| `POST` | `/api/library/config`| `LibraryConfig` (JSON) | Aggiorna e salva le preferenze in `retrohub.json`. |
| `POST` | `/api/library/scan` | - | Scansiona le cartelle `roms/` e rileva nuove ROM inserite manualmente. |
| `POST` | `/api/library/download`| `{ "game_id": 123 }` | Scarica on-the-fly la ROM e ne verifica l'integrità. |
| `POST` | `/api/library/delete`| `{ "game_id": 123 }` | Cancella la ROM locale per liberare spazio su disco. |
| `POST` | `/api/ping` | - | Heartbeat inviato dalla UI per mantenere attivo il micro-server. |
| `POST` | `/api/shutdown` | - | Chiusura ordinata del server e rilascio risorse. |

---

## 🚀 7. Roadmap di Portabilità Multipiattaforma

### 🥇 Fase Windows (Obiettivo Corrente - Produzione Immediata)
* Test e consolidamento su Windows 10 e Windows 11 a 64 bit.
* Eliminazione totale di script PowerShell con string-escaping fragile; uso esclusivo di `std::process::Command` e `winapi` per il focus delle finestre.
* Rilascio pacchetto distribuibile portabile autonomo (`RetroHub-v2.0-Win64.zip`).

### 🥈 Fase Linux (Prossima Tappa)
* Compilazione binaria ELF (`target/release/retro-hub`).
* Apertura finestra desktop tramite Chromium/Chrome in `--app=http://127.0.0.1:39281` o integrazione Webview nativa (`tao` / `wry`).
* Core Libretro in formato `.so` con supporto sia x86_64 che ARM64 (Raspberry Pi 4/5).

### 🥉 Fase macOS (Tappa Successiva)
* Compilazione binario universale Apple Silicon (ARM64 M1/M2/M3) + Intel x86_64.
* Generazione bundle applicativo macOS (`RetroHub.app`).
* Core Libretro in formato `.dylib` con firma ad-hoc.
