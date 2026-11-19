# 🗺️ RetroHub — Roadmap di Sviluppo Strategica
**Versione Roadmap:** 3.2 (Target Prioritario: Windows Produzione Immediata | Clean Rebirth UI & Architettura Universale a 12 Macro-Famiglie)  
**Obiettivo:** Trasformare RetroHub nel launcher retrogaming definitivo per Windows: motore Rust a 60 FPS da soli 7 MB, enciclopedia on-demand per 200+ console organizzata in 12 macro-famiglie (con Laserdisc, Pinball, Game & Watch, Computer storici e PC Recomp/Indie), Hero Widescreen cinematografica con game art attiva a 1920x620, modalità vergine da nuovo utente (zero file toccati in locale), icone vettoriali SVG minimali e simulatore P2P per l'Arena. *(Linux e macOS pianificati per le fasi successive al rilascio Windows).*

---

## 📌 FASE 1: Client Locale & Core Engine (Completata 🟢)
*Obiettivo: Interfaccia desktop ultra-rapida, database SQLite indicizzato e motore di ricerca avanzato.*

- [x] **Backend Rust Ultra-rapido:** Server HTTP locale integrato (`tiny_http` porta 39281), avvio istantaneo in <50ms e consumo RAM <25MB.
- [x] **Database SQLite Locale con Indici:** Tabelle `games`, `stats` e preferiti con query ottimizzate a 0ms.
- [x] **Motore di Ricerca Avanzato Multi-campo & Fuzzy:**
  - Punteggio pesato Jaro-Winkler e distanza Levenshtein.
  - Supporto per operatori booleani (esclusione `-` / `NOT`, frasi esatte tra virgolette `"..."`, OR `|`, filtri `genre:`, `dev:`, `sys:`, intervalli anni `1990..1995`).
- [x] **UI Cinematografica Dark Mode (Ispirata a Steam & RecompLauncher):**
  - Hero Spotlight Carousel con artwork panoramico e comandi gamepad.
  - Griglia poster con proporzioni boxart fisse e glow ciano.
  - Fallback tipografico elegante per titoli privi di boxart esterna.
  - Navigazione nativa da controller Xbox e adattatori USB (DirectInput / XInput).

---

## 📌 FASE 2: Arena Multiplayer P2P & Leaderboard Standardizzata (In Sviluppo 🟡)
*Obiettivo: Sfide 1 contro 1 a distanza con amici senza server intermedi e classifica globale dei record Arcade.*

- [x] **Generatore Codici Stanza:** Creazione token univoci di invito (es. `RH-7821`).
- [x] **Handshake P2P Diretto (Rete Locale / LAN):** Connessione diretta tra Host e Guest via socket UDP.
- [x] **Avvio Sincronizzato Emulatori Netplay:**
  - Parametri da riga di comando per Flycast, Snes9x, DuckStation, Dolphin e MAME.
- [x] **Simulatore Bot P2P One-Click (Test Sviluppo & Collaudo):**
  - Pulsante dedicato `[ 🤖 Simula Match con Bot P2P ]` nel modale Arena.
  - Inietta un finto avversario virtuale (*RetroBot_99*, ping 18ms), simula handshake, countdown 3.. 2.. 1.. e lancia il gioco in prova.
  - Incapsulato sotto il flag `ARENA_DEV_SIMULATION` (facilmente escludibile per la release finale).
- [ ] **Dashboard Interattiva Arena Online:**
  - Schermata grafica a schede per Creazione Stanza, Unione tramite Codice e Monitoraggio Latenza (Ping RTT in tempo reale).
- [ ] **Leaderboard Standardizzata & Hall of Fame (High Scores):**
  - Estrazione automatica punteggio record al Game Over per Arcade (MAME / Neo-Geo) e classici a punti.
  - Classifica locale offline su PC + invio sicuro alla Leaderboard Globale RetroHub del mese.
- [ ] **NAT Traversal Pubblico Reale (WAN / Internet):**
  - Integrazione resolver STUN reale (Google / Cloudflare) per bypass router senza port forwarding manuale.

---

## 📌 FASE 3: Server Lobby & Bacheca Stanze Pubbliche (Cloud Matchmaker ⚪)
*Obiettivo: Matchmaking pubblico centralizzato con bacheca "In attesa di sfidante".*

- [ ] **Microservizio di Segnalazione Remoto:** Server cloud leggero per coordinamento sessioni attive.
- [ ] **Bacheca Stanze in-App:** Schermata "Arena Online" collegata a stanze pubbliche reali con filtro per latenza e regione.

---

## 📌 FASE 4: Personalizzazione Libreria, Enciclopedia Immagini & Onboarding Vergine (In Corso 🚀)
*Obiettivo: Separazione netta tra Scheletro visivo (copertine HD in locale a 60 FPS offline) e Muscolo (ROM a 0 Byte sul disco, scaricate solo ed esclusivamente al click `▶ GIOCA`).*

### **4.1 Paradigma Architetturale Senza Duplicati & Separazione Ruoli (Utente al Centro)**
- [x] **Il Romset di Riferimento Ufficiale è il Set Mondiale Senza Duplicati (Standard 1G1R):**
  - Standard universale: 1 solo gioco per titolo, escludendo rigorosamente doppioni regionali ridondanti (JAP/USA/EUR), cloni pirata, BIOS e diagnostica hardware.
  - Microcopy UX approvata: label utente **`Senza Duplicati`** (`No Duplicates` / `Sin Duplicados`), con dicitura tecnica *"Standard 1G1R"* confinata ai tooltip di aiuto.
  - Il catalogo di riferimento mondiale conta **~11.500 titoli commerciali autentici** (~2.500 Arcade, 725 SNES, 715 NES, 1.050 GBA, 1.450 PSX, ecc.).
- [x] **Nessuna Selezione Arbitraria Imposta: La Selezione la Fa l'Utente:**
  - Rimossa qualsiasi divisione artificiale rigida ("Hall of Fame vs 1G1R") che generava confusione e numeri identici.
  - L'utente esplora l'intera collezione pulita ed estrae le SUE selezioni d'élite tramite il motore di ricerca avanzato (Anni, Generi Multipli, 2 Giocatori Co-op vs 1v1, Produttori) salvandole istantaneamente in **Raccolte Custom** e **Preferiti ⭐**.
- [x] **Gestione Download Immagini & Info (Opzioni vs Home):**
  - **Metadati (Info, Tag, Anni, Generi, Controlli):** Sempre tutti presenti e istantanei in locale nel database SQLite `retro_hub.db` (~21 MB per tutti gli 11.500 titoli, zero richieste di rete).
  - **Immagini (Boxart WebP, Screenshot Snaps, Loghi):**
    - *Default:* Cache Dinamica On-Demand leggera (30–100 MB), scarica solo per i titoli visualizzati.
    - *Pre-Download Piattaforma (Nelle Opzioni):* Nelle `⚙️ Opzioni`, l'utente ha il toggle/pulsante dedicato per pre-scaricare con 1 clic l'intero pacchetto immagini per la singola piattaforma gestita (es. MAME ~280 MB in WebP) o globale, per navigazione 100% offline a 60 FPS.
    - *Home / Libreria Principale:* Rimane focalizzata al 100% sulla giocabilità quotidiana: switch limpido `[ 🔓 Tutto l'Archivio Cloud ]` vs `[ 💾 Solo Scaricati ]`, senza pulsanti di manutenzione che appesantiscono la barra.
- [x] **ROMs a 0 Byte Preventivi su Disco:**
  - Nessun romset pesante scaricato a monte; le ROM rimangono interamente in cloud/on-demand, scaricate al click `▶ GIOCA` con gestione salva-spazio post-partita.
  - Disconnessione sicura software di `emu_root` senza mai toccare, spostare o cancellare file utente.

### **4.2 Download ROM "On-The-Fly" al Click**
- [x] **Download Trasparente al Lancio:**
  - Al click **☁️ SCARICA & GIOCA**, il client scarica la singola ROM in background in `retro_downloads/` ed avvia il gioco a schermo intero.
- [x] **Dialog Intelligente Post-Partita (Salva-Spazio):**
  - Prompt di chiusura integrato: `[ 💾 TIENI SUL PC ]` vs `[ 🗑️ ELIMINA PER FARE SPAZIO ]`.

### **4.3 Wizard di Onboarding & Strategie di Storage (I 4 Profili Ufficiali)**
- [x] **Setup Wizard Obbligatorio al Primo Avvio (Bloccante Nuovo Utente):**
  - Se non configurato (`!retrohub_wizard_completed`), il wizard non è chiudibile via tasto ✕, Annulla o ESC: l'utente deve selezionare un profilo per impostare RetroHub.
  - Rieseguibile in qualunque momento dal pannello `⚙️ Opzioni`.
- [x] **Default del Pre-Download Copertine nei 4 Profili:**
  - **Caso 1: Smart On-Demand Standard (Default Consigliato):** Pre-Download Copertine = **ATTIVO** (~350-450 MB per 5 console scelte a 60 FPS offline); **0 Byte ROM su disco** a monte con download al click `▶ GIOCA` ed eliminazione opzionale post-partita.
  - **Caso 2: Collezione Esistente (Zero Spostamenti):** Pre-Download Copertine = **ATTIVO** (per rivestire con artwork HD i file trovati); scansione in sola lettura senza duplicare o spostare file utente.
  - **Caso 3: Low Storage / Handheld (Zero Byte Immagini):** Pre-Download Copertine = **DISATTIVO** (0 Byte all'inizio, card tipografiche vettoriali ad alta efficienza; copertina reale scaricata solo per Preferiti ⭐ o on-demand).
  - **Caso 4: Full Pack Offline (Console Scelte):** Pre-Download Copertine & ROM = **ATTIVO** (copertine HD + set ROM completo scaricato in anticipo per giocare 100% offline).
- [x] **Toggle Pre-Download Automatico nelle Opzioni:**
  - Switch esplicito in `⚙️ Opzioni` (`#settingsPreDownloadToggle`) per personalizzare il comportamento in qualsiasi momento.

### **4.4 Strategia MAME Curato (1G1R Parent-Only vs Cloni)**
- [x] **Default Pulito 1G1R (~950 Titoli Parent):** Esclusione di cloni, bootleg, mahjong e slot machine; 105 MB copertine, ~28 GB ROM on-demand.
- [x] **Switch Cloni & Varianti Regionali:** Filtro opzionale per consultare ed avviare versioni alternative e set completi.

---

## 📌 FASE 5: Architettura a 12 Macro-Famiglie & Rebirth UI (In Corso 🧩)
*Obiettivo: Supportare con dignità e chiarezza tutti i ~200 sistemi storici via Sidebar sinistra collassabile e Hero cinematografica.*

### **5.1 Tassonomia Universale delle 12 Macro-Famiglie (~200 Sistemi)**
1. **🕹️ Arcade Coin-Op:** MAME, FBNeo, Capcom CPS-1/2/3, Neo Geo MVS, Sega Naomi 1/2, Model 2/3, Hikaru, Atomiswave, Namco System.
2. **📀 Laserdisc & FMV:** Hypseus Singe / Daphne (*Dragon's Lair*, *Space Ace*), American Laser Games, Pioneer LaserActive.
3. **🎯 Pinball & Flipper:** Visual Pinball X (VPX), Future Pinball, Visual PinMAME, Pro Pinball.
4. **📟 LCD & Game & Watch:** Nintendo Game & Watch (Silver, Multi-Screen, Tabletop), Tiger Handhelds, Bandai LCD, Coleco Tabletop.
5. **🍄 Nintendo Universe:** NES, FDS, SNES, N64, 64DD, GameCube, Wii, Wii U, Game Boy, GBC, GBA, DS, 3DS, Virtual Boy.
6. **🌀 Sega Universe:** SG-1000, Master System, Mega Drive, Sega CD, 32X, Saturn, Dreamcast, Game Gear, Nomad.
7. **🎮 Sony PlayStation:** PlayStation (PS1/PSX), PlayStation 2, PlayStation 3, PSP, PS Vita.
8. **🦅 Atari & Vintage Pioneers:** Atari 2600, 5200, 7800, Lynx, Jaguar, Vectrex (grafica vettoriale), ColecoVision, Intellivision, Odyssey 2.
9. **⚔️ SNK & NEC Engine:** Neo Geo AES, Neo Geo CD, Neo Geo Pocket / Color, PC Engine / TurboGrafx-16, SuperGrafx, PC-FX.
10. **💾 Home Computer Occidentali:** Commodore (C64, C128, Amiga 500/1200/CD32), ZX Spectrum, Amstrad CPC, BBC Micro, Apple II/Mac, MS-DOS, ScummVM.
11. **🗾 Nippon Micro Computers:** Sharp X68000, MSX / MSX2 / Turbo-R, FM Towns / Marty, NEC PC-8801, PC-9801.
12. **⚡ PC Moderno, Recomp & Indie:** PC Recomp 60FPS (*Zelda, Mario 64, Star Wars*), PC Indie (*LocoMalito, AM2R, Ori, Celeste, Hollow Knight*), Fantasy Consoles (PICO-8, TIC-80).

### **5.2 Nuova Navigazione: Sidebar Sinistra Collassabile (Stile Steam / Playnite)**
- **Rail Compatto (64px):** Icone vettoriali SVG minimali per le 12 macro-famiglie + Preferiti + Raccolte Custom. Non spreca spazio orizzontale.
- **Drawer Espanso (260px):** Albero espandibile per famiglia con conteggio reale dei giochi per ciascuna console.
- Risolto definitivamente il problema del taglio orizzontale ("Game Boy Advance tagliato a destra").

### **5.3 Hero Spotlight Cinematografica Re-engineered**
- **Widescreen Game Art Dinamica:** Per Steam Indie usiamo l'artwork ufficiale 1920x620 `library_hero.jpg`; per i retrogame usiamo lo snap/boxart panoramico con illuminazione console e gradiente scuro direzionale a sinistra per la massima leggibilità di testo e pulsanti.
- **Pulsantiera delle 4 Sorgenti Reali (Addio RetroShuffle):**
  1. 👑 *Proposta del Giorno*
  2. ⭐ *I Tuoi Preferiti*
  3. ⏱️ *Continua a Giocare*
  4. 📁 *Collezione Custom* (proietta in vetrina una raccolta personale a scelta).
- **Icone Vettoriali SVG Minimali:** Sostituzione integrale delle emoji OS ("cheap") con set geometrico ispirato a Lucide/Phosphor.

---

## 📌 FASE 6: Universal Scraper & Database Harvester (Post-Lancio / Fase Finale 🌐)
*Obiettivo: Automatizzare al 100% l'arricchimento di metadati e cover per ROM locali, librerie digitali PC e progetti Recomp.*

- [ ] **Scraper Automatico No-Intro / Redump / ScreenScraper:** Caching locale permanente delle boxart 3D e screenshot in-game.
- [ ] **Steam & GOG Auto-Discovery:** Rilevamento automatico delle installazioni locali PC.
- [ ] **Recomp Asset Hash Verifier:** Validazione automatica SHA-256 dei file ROM estratti.
