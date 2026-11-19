# 🗺️ RetroHub — Roadmap di Sviluppo Strategica
**Versione Roadmap:** 3.3 (Release v0.45.0 | Progressive Disclosure Hero, Quote-Immune Native Launch, Custom Collections & OLED Modals)  
**Obiettivo:** Trasformare RetroHub nel launcher retrogaming definitivo per Windows: motore Rust a 60 FPS da 9 MB, enciclopedia on-demand per 1.734+ titoli storici organizzati in 12 macro-famiglie (con Laserdisc, Pinball, Game & Watch, Computer storici e PC Recomp/Indie), Hero Widescreen cinematografica con progressive disclosure ("A Sorpresa" permanente, preferiti/continua/custom solo se reali), zero file toccati in locale, icone vettoriali minimali, avvio MAME quote-immune e simulatore P2P per l'Arena. *(Linux e macOS pianificati per le fasi successive al rilascio Windows).*

---

## 📌 FASE 1: Client Locale & Core Engine (Completata 🟢)
*Obiettivo: Interfaccia desktop ultra-rapida, database SQLite indicizzato e motore di ricerca avanzato.*

- [x] **Backend Rust Ultra-rapido:** Server HTTP locale integrato (`tiny_http` porta 39281), avvio istantaneo in <50ms e consumo RAM <25MB.
- [x] **Database SQLite Locale con Indici:** Tabelle `games`, `stats` e preferiti con query ottimizzate a 0ms (1.734 titoli indicizzati).
- [x] **Motore di Ricerca Avanzato Multi-campo & Fuzzy:**
  - Punteggio pesato Jaro-Winkler e distanza Levenshtein.
  - Supporto per operatori booleani (esclusione `-` / `NOT`, frasi esatte tra virgolette `"..."`, OR `|`, filtri `genre:`, `dev:`, `sys:`, intervalli anni `1990..1995`).
- [x] **UI Cinematografica Dark Mode (Ispirata a Steam & RecompLauncher):**
  - Hero Spotlight Carousel con artwork panoramico e comandi gamepad.
  - Griglia poster con proporzioni boxart fisse e glow dinamico per console.
  - Fallback tipografico elegante per titoli privi di boxart esterna.
  - Navigazione nativa da controller Xbox e adattatori USB (DirectInput / XInput).
- [x] **Single-Instance Guard & Window Manager:** Controllo istanza singola all'avvio con attivazione automatica finestra app Edge e zero duplicati di processo.

---

## 📌 FASE 2: Arena Multiplayer P2P & Leaderboard Standardizzata (In Sviluppo 🟡)
*Obiettivo: Sfide 1 contro 1 a distanza con amici senza server intermedi e classifica globale dei record Arcade.*

- [x] **Generatore Codici Stanza:** Creazione token univoci di invito (es. `RH-7821`).
- [x] **Handshake P2P Diretto (Rete Locale / LAN):** Connessione diretta tra Host e Guest via socket UDP.
- [x] **Avvio Sincronizzato Emulatori Netplay:**
  - Parametri da riga di comando per Flycast, Snes9x, DuckStation, Dolphin e MAME.
- [x] **Simulatore Bot P2P One-Click (Test Sviluppo & Collaudo):**
  - Pulsante dedicato `[ 🤖 Simula Match con Bot P2P ]` nel modale Arena.
  - Inietta un avversario virtuale (*RetroBot_99*, ping 18ms), simula handshake, countdown 3.. 2.. 1.. e lancia il gioco in prova.
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

## 📌 FASE 4: Personalizzazione Libreria, Enciclopedia Immagini & Onboarding Vergine (Completata 🟢)
*Obiettivo: Separazione netta tra Scheletro visivo (copertine HD in locale a 60 FPS offline) e Muscolo (ROM a 0 Byte sul disco, scaricate solo ed esclusivamente al click `▶ GIOCA`).*

### **4.1 Paradigma Architetturale Senza Duplicati & Separazione Ruoli (Utente al Centro)**
- [x] **Il Romset di Riferimento Ufficiale è il Set Mondiale Senza Duplicati (Standard 1G1R):**
  - Standard universale: 1 solo gioco per titolo, escludendo rigorosamente doppioni regionali ridondanti (JAP/USA/EUR), cloni pirata, BIOS e diagnostica hardware.
  - Microcopy UX approvata: label utente **`Senza Duplicati`** (`No Duplicates` / `Sin Duplicados`), con dicitura tecnica *"Standard 1G1R"* confinata ai tooltip di aiuto.
  - Catalogo espanso a **1.734 titoli commerciali autentici** su 41 piattaforme senza duplicati ridondanti.
- [x] **Nessuna Selezione Arbitraria Imposta: La Selezione la Fa l'Utente:**
  - L'utente esplora l'intera collezione pulita ed estrae le SUE selezioni d'élite tramite il motore di ricerca avanzato (Anni, Generi Multipli, Co-op vs 1v1, Produttori) salvandole in **Raccolte Custom** e **Preferiti ⭐**.
- [x] **Gestione Raccolte Custom Genuina & Persistente (v0.44 - v0.45):**
  - Eliminazione di tutte le raccolte mock/fake di default (es. vecchia "Capcom Arcade").
  - Modale di gestione playlist con pulsanti "📂 Nel Catalogo" (filtro griglia), "🌟 In Vetrina" (proietta su Hero) e "🗑️ Elimina" con modale OLED.
  - Salvataggio rapido con 1 clic ("Salva Selezione") dai filtri attivi.
- [x] **Gestione Download Immagini & Info (Opzioni vs Home):**
  - **Metadati (Info, Tag, Anni, Generi, Controlli):** Sempre tutti presenti e istantanei in locale nel database SQLite `retro_hub.db` (~21 MB per tutti i titoli, zero richieste di rete).
  - **Immagini (Boxart WebP, Screenshot Snaps, Loghi):**
    - *Default:* Cache Dinamica On-Demand leggera, scarica solo per i titoli visualizzati.
    - *Pre-Download Piattaforma (Nelle Opzioni):* Nelle `⚙️ Opzioni`, l'utente ha il toggle/pulsante dedicato per pre-scaricare con 1 clic l'intero pacchetto immagini per la singola piattaforma o globale per navigazione offline.
- [x] **ROMs a 0 Byte Preventivi su Disco:**
  - Nessun romset pesante scaricato a monte; le ROM rimangono interamente in cloud/on-demand, scaricate al click `▶ GIOCA` con gestione salva-spazio post-partita.
  - Disconnessione sicura software di `emu_root` senza mai toccare, spostare o cancellare file utente.

### **4.2 Download ROM "On-The-Fly" & Lancio Robusto (v0.45.0)**
- [x] **Download Trasparente al Lancio:**
  - Al click **☁️ SCARICA & GIOCA**, il client scarica la singola ROM in background in `retro_downloads/` ed avvia il gioco a schermo intero.
- [x] **Risolto Falso Crash e Rimozione Timer Cieco a 4s:**
  - Rimosso il pop-up arbitrario a 4 secondi post-lancio che rubava il focus durante l'apertura dell'emulatore.
  - Comportamento post-partita predefinito su **Conserva sempre su disco** (`always_keep`), con dialog gentile al rientro (`focus` window dopo >25s) solo se configurato su `ask`.
- [x] **Lancio Diretto MAME Quote-Immune:**
  - Sostituito lo script PowerShell con `std::process::Command::new` nativo Rust.
  - Immunità totale a caratteri speciali e percorsi utente con apostrofi (`ClaudioDall'Ara`).
  - Flag `-skip_gameinfo` per bypassare schermate disclaimer hardware non bloccanti.
  - Preservazione della cartella di lavoro per link e wrapper (.lnk, .bat, Hypseus Singe).

### **4.3 Wizard di Onboarding & Dialog di Conferma OLED**
- [x] **Setup Wizard Obbligatorio al Primo Avvio (Bloccante Nuovo Utente):**
  - Se non configurato (`!retrohub_wizard_completed`), il wizard guida l'utente nella scelta tra i 4 profili di archiviazione.
- [x] **Correzione Dialog di Conferma Custom (`#confirmModal`):**
  - Risolto bug classe CSS (`.open` anziché `.active`), ripristinando la piena visibilità di conferma per "Azzera e Riavvia Wizard" e cancellazione raccolte.
- [x] **Azzera e Riavvia Wizard Istantaneo:**
  - Pulizia stato e riapertura immediata del wizard di onboarding senza ricaricamento forzato della pagina.

---

## 📌 FASE 5: Architettura a 12 Macro-Famiglie & Rebirth UI (Completata 🟢)
*Obiettivo: Supportare con dignità e chiarezza tutti i ~200 sistemi storici via Sidebar sinistra collassabile e Hero cinematografica.*

### **5.1 Tassonomia Universale delle 12 Macro-Famiglie (41 Piattaforme Pulite Senza Duplicati)**
1. **🕹️ Arcade Coin-Op:** MAME, FBNeo, Capcom CPS-1/2/3, Neo Geo MVS, Sega Naomi 1/2, Model 2/3, Hikaru, Atomiswave.
2. **📀 Laserdisc & FMV:** Hypseus Singe / Daphne (*Dragon's Lair*, *Space Ace*), American Laser Games, Pioneer LaserActive.
3. **🎯 Pinball & Flipper:** Visual Pinball X (VPX), Future Pinball, Visual PinMAME.
4. **📟 LCD & Game & Watch:** Nintendo Game & Watch (Silver, Multi-Screen, Tabletop), Tiger Handhelds.
5. **🍄 Nintendo Universe:** NES, FDS, SNES, N64, 64DD, GameCube, Game Boy, GBC, GBA.
6. **🌀 Sega Universe:** Master System, Mega Drive, Sega CD, 32X, Saturn, Dreamcast, Game Gear.
7. **🎮 Sony PlayStation:** PlayStation (PS1/PSX), PlayStation 2, PSP.
8. **🦅 Atari & Vintage Pioneers:** Atari 2600, 5200, 7800, Lynx, Jaguar, Vectrex, ColecoVision, Intellivision.
9. **⚔️ SNK & NEC Engine:** Neo Geo AES, Neo Geo CD, Neo Geo Pocket / Color, PC Engine, SuperGrafx, PC-FX.
10. **💾 Home Computer Occidentali:** Commodore (C64, C128, Amiga 500), MS-DOS, ScummVM.
11. **🗾 Nippon Micro Computers:** Sharp X68000, MSX / MSX2, FM Towns, NEC PC-8801, PC-9801.
12. **⚡ PC Moderno, Recomp & Indie:** PC Recomp 60FPS (*Zelda, Mario 64, OutRun*), PC Indie (*LocoMalito, AM2R, Ori, Celeste, Hollow Knight*), Fantasy Consoles (PICO-8, TIC-80).

### **5.2 Navigazione & Carousel Piattaforme**
- **Eliminazione Pillole Duplicate:** Risolte sovrapposizioni (Laserdisc unificato sotto `HYPSEUS`, Arcade unificato sotto `MAME`), portando l'elenco a 41 piattaforme canoniche.
- **Scroll e Selezione Fluida:** Evidenziazione attiva corretta e scorrimento fluido con rotellina e frecce direzionali.

### **5.3 Hero Spotlight a Rivelazione Progressiva (Progressive Disclosure)**
- **`🎲 A Sorpresa` Permanente:** Modalità predefinita sempre visibile che estrae a rotazione giochi autentici dal catalogo.
- **`⭐ Preferiti` Dinamico:** Appare solo se l'utente ha almeno un gioco tra i preferiti.
- **`⏱️ Continua` Dinamico:** Appare solo se l'utente ha giocato ad almeno un titolo.
- **`📁 Custom` Dinamico:** Appare solo se l'utente ha creato raccolte personali contenenti giochi.
- **Fallback Automatico:** Commutazione automatica su *A Sorpresa* se l'utente rimuove tutti i preferiti o elimina l'ultima raccolta.

---

## 📌 FASE 6: Universal Scraper & Database Harvester (In Corso 🌐)
*Obiettivo: Automatizzare al 100% l'arricchimento di metadati e cover per ROM locali, librerie digitali PC e progetti Recomp.*

- [x] **Risolutore Locandine MAME Locali:** Corretto endpoint `/api/cover/mame/<rom>` con parsing pulito dell'estensione, collegando oltre 530 flyers, titles e snaps già presenti su disco.
- [x] **Generatore Copertine Vettoriali SVG:** Creazione e servizio di poster vettoriali in tempo reale per flipper, Game & Watch e Laserdisc in `/api/cover/file/`.
- [ ] **Scraper Automatico No-Intro / Redump / ScreenScraper:** Caching locale permanente delle boxart 3D e screenshot in-game per le copertine rimanenti.
- [ ] **Steam & GOG Auto-Discovery:** Rilevamento automatico delle installazioni locali PC.
