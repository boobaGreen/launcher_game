# 🗺️ RetroHub — Roadmap di Sviluppo Strategica
**Versione Roadmap:** 3.5 (Release v0.47.0 | Multi-Monitor Dynamic OSD, Universal Bidirectional Sorting, 6 Authentic G&W Engines, Commodore & Amiga Integration)  
**Obiettivo:** Trasformare RetroHub nel launcher retrogaming definitivo per Windows: motore Rust a 60 FPS da 9 MB, enciclopedia on-demand per 1.699 titoli storici verificati organizzati in 12 macro-famiglie (con Laserdisc, Pinball, Game & Watch, Computer storici Commodore/Amiga e PC Recomp/Indie), Hero Widescreen cinematografica con progressive disclosure ("A Sorpresa" permanente, preferiti/continua/custom solo se reali), zero file toccati in locale, icone vettoriali minimali, avvio MAME quote-immune con Win32 OSD multi-monitor dinamico, configurazione emulatori in Opzioni (RetroArch, VPX, MAME, DOSBox) con fallback simulatore web e simulatore P2P per l'Arena. *(Linux e macOS pianificati per le fasi successive al rilascio Windows).*

---

## 📌 FASE 1: Client Locale & Core Engine (Completata 🟢)
*Obiettivo: Interfaccia desktop ultra-rapida, database SQLite indicizzato e motore di ricerca avanzato.*

- [x] **Backend Rust Ultra-rapido:** Server HTTP locale integrato (`tiny_http` porta 39281), avvio istantaneo in <50ms e consumo RAM <25MB.
- [x] **Database SQLite Locale con Indici:** Tabelle `games`, `stats` e preferiti con query ottimizzate a 0ms (1.699 titoli canonicamente indicizzati).
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
  - Catalogo espanso a **1.699 titoli commerciali autentici** su 35 piattaforme senza duplicati ridondanti.
- [x] **Nessuna Selezione Arbitraria Imposta: La Selezione la Fa l'Utente:**
  - L'utente esplora l'intera collezione pulita ed estrae le SUE selezioni d'élite tramite il motore di ricerca avanzato (Anni, Generi Multipli, Co-op vs 1v1, Produttori) salvandole in **Raccolte Custom** e **Preferiti ⭐**.
- [x] **Gestione Raccolte Custom Genuina & Persistente (v0.44 - v0.47):**
  - Eliminazione di tutte le raccolte mock/fake di default.
  - Modale di gestione playlist con pulsanti "📁 Nel Catalogo" (filtro griglia), "🌟 In Vetrina" (proietta su Hero) e "🗑️ Elimina" con modale OLED.
  - Salvataggio rapido con 1 clic ("Salva Selezione") dai filtri attivi.
- [x] **Gestione Download Immagini & Info (Opzioni vs Home):**
  - **Metadati (Info, Tag, Anni, Generi, Controlli):** Sempre tutti presenti e istantanei in locale nel database SQLite `retro_hub.db` (~21 MB per tutti i titoli, zero richieste di rete).
  - **Immagini (Boxart WebP, Screenshot Snaps, Loghi):**
    - *Default:* Cache Dinamica On-Demand leggera, scarica solo per i titoli visualizzati.
    - *Pre-Download Piattaforma (Nelle Opzioni):* Nelle `⚙️ Opzioni`, l'utente ha il toggle/pulsante dedicato per pre-scaricare con 1 clic l'intero pacchetto immagini per la singola piattaforma o globale per navigazione offline.
- [x] **ROMs a 0 Byte Preventivi su Disco:**
  - Nessun romset pesante scaricato a monte; le ROM rimangono interamente in cloud/on-demand, scaricate al click `▶ GIOCA` con gestione salva-spazio post-partita.
  - Disconnessione sicura software di `emu_root` senza mai toccare, spostare o cancellare file utente.

### **4.2 Download ROM "On-The-Fly", Gestione Emulatori & Lancio Robusto (v0.47.0)**
- [x] **Download Trasparente al Lancio con Intercettazione Falsi Positivi:**
  - Al click **☁️ SCARICA & GIOCA**, il client scarica la singola ROM in background in `retro_downloads/` ed avvia il gioco a schermo intero.
  - Se il server cloud risponde `success: false`, la barra si arresta subito e la scheda viene ripristinata mostrando il toast informativo per l'inserimento manuale del file.
- [x] **Hub Configurazione Emulatori Standalone & Multi-Core in Opzioni:**
  - Sezione dedicata **`🕹️ Emulatori & Core di Sistema`** nelle Opzioni con input percorsi, badge di stato live (`🟢 Rilevato` / `🟡 Non Configurato`) e pulsante di auto-rilevamento globale (`🔍 Rileva Automaticamente`).
  - Salvataggio e caricamento persistente in `retrohub.json` per RetroArch, Visual Pinball X (VPX), DOSBox e MAME Arcade.
- [x] **Sistema Multi-Monitor Dinamico & OSD a Tutto Schermo:**
  - Rilevamento in tempo reale della topologia display tramite Win32 `EnumDisplayMonitors` e `GetMonitorInfoW`.
  - Ordinamento spaziale da sinistra a destra dei monitor fisici con coordinate reali.
  - Modalità "🔄 Segui Finestra RetroHub (Dinamico)" per seguire automaticamente il monitor attivo.
  - Identificazione OSD visiva con flash HUD scuro a schermo intero per 2,2 secondi.
  - Parametri MAME dedicati: `-numscreens 1 -screen "{display}" -ui_mouse 0 -nomouse -skip_gameinfo`.
- [x] **Lancio Diretto MAME & DOSBox 8.3 Short Path:**
  - Sostituito lo script PowerShell con `std::process::Command::new` nativo Rust (immune a virgolette e apostrofi).
  - Conversione automatica in percorsi brevi 8.3 (`to_short_path`) per DOSBox Staging.
  - Scansione automatica a due livelli (radice, `games/` e `roms/`) per ScummVM e C64.

### **4.3 Wizard di Onboarding & Dialog di Conferma OLED**
- [x] **Setup Wizard Obbligatorio al Primo Avvio (Bloccante Nuovo Utente):**
  - Se non configurato (`!retrohub_wizard_completed`), il wizard guida l'utente nella scelta tra i 4 profili di archiviazione.
- [x] **I 4 Profili Utente Operativi:**
  1. *Smart On-Demand* (Default consigliato: cover HD locali, ROM a 0-byte su disco).
  2. *Collezione Esistente Locale* (Auto-discovery cartella standard `emu/roms`, zero duplicati).
  3. *Low Storage / Handheld* (0-Byte a monte, card tipografiche vettoriali).
  4. *Full Pack Offline* (Copertine + ROM scaricate in locale per gioco 100% offline).

---

## 📌 FASE 5: Architettura a 12 Macro-Famiglie & Rebirth UI (Completata 🟢)
*Obiettivo: Supportare con dignità e chiarezza tutti i ~200 sistemi storici via Sidebar sinistra collassabile e Hero cinematografica.*

### **5.1 Tassonomia Universale delle 12 Macro-Famiglie (35 Piattaforme Pulite Senza Duplicati)**
1. **🕹️ Arcade Coin-Op:** MAME (568), NEOGEO (120), NAOMI (27), ATOMISWAVE (8), HIKARU (4).
2. **💿 Laserdisc & FMV:** HYPSEUS (22 titoli, 100% locandine originali WebP/PNG).
3. **🎯 Pinball & Flipper:** PINBALL (40 tavoli, 100% backglass originali HD).
4. **📟 LCD & Game & Watch:** GAMEWATCH (15 titoli con 6 motori simulatore autentici).
5. **🍄 Nintendo Universe:** SNES (86), NES (54), GBA (51), N64 (20), GAMECUBE (5).
6. **🌀 Sega Universe:** MEGADRIVE (58), DREAMCAST (5) — con filtro combinato Mega Drive / Genesis.
7. **🎮 Sony PlayStation:** PSX (60 titoli classici 32-bit).
8. **🦅 Atari & Vintage Pioneers:** ATARI2600 (15), ATARI7800 (4), INTELLIVISION (4), JAGUAR (4), LYNX (4), COLECO (3), VECTREX (3).
9. **⚔️ SNK & NEC Engine:** Neo Geo AES/MVS (120) + Espansioni PC Engine.
10. **💾 Home Computer Occidentali:** C64 (223), DOS (100), SCUMMVM (64), AMIGA (25 classici storici con cover HD).
11. **🗾 Nippon Micro Computers:** MSX2 (8), X68000 (8), MSX (4), PC98 (4), FMTOWNS (3), PC88 (2).
12. **⚡ PC Moderno, Recomp & Indie:** RECOMP (45 port 60 FPS), INDIE (33 titoli).

### **5.2 Ordinamento Moderno Universale (v0.47.0)**
- **Separazione Criterio e Direzione:**
  - Selettore del criterio: Titolo alfabetico, Anno, Sviluppatore, Genere, Valutazione ⭐, Frequenza di gioco 🔥.
  - Pulsante toggle interattivo `[ ⬆ / ⬇ ]` con etichette semantiche chiare (*A → Z* vs *Z → A*, *Più Recenti* vs *Più Storici*).
  - Ordinamento secondario alfabetico garantito su tutti i campi.

### **5.3 Simulatore LCD Game & Watch a 6 Motori Autentici (v0.47.0)**
- Motori dedicati con scenari LCD e logica indipendente:
  - *Ball (1980)*: Giocoliere centrale a 3 posizioni con traiettorie paraboliche.
  - *Fire*: Pompieri con telo elastico tra edificio in fiamme e ambulanza.
  - *Parachute*: Elicottero con rotore, paracadutisti, vento e squalo.
  - *Chef*: Cuoco con padella, cibo volteggiante e gatto dispettoso.
  - *Donkey Kong*: Travi rosse d'acciaio, barili rotolanti e Jumpman.
  - *Octopus*: Sommozzatori, tentacoli estensibili del polpo e forziere del tesoro.

---

## 📌 FASE 6: Universal Scraper & Database Harvester (In Corso 🟡)
*Obiettivo: Automatizzare al 100% l'arricchimento di metadati e cover per ROM locali, librerie digitali PC e progetti Recomp.*

- [x] **Risolutore Locandine MAME Locali:** Corretto endpoint `/api/cover/mame/<rom>` con parsing pulito dell'estensione.
- [x] **Ingestion & Servizio Copertine 100% Pinball, Laserdisc & Game & Watch:**
  - **Pinball (40/40 - 100%):** Tutte le 40 backglass originali HD WebP servite con successo.
  - **Laserdisc & FMV (22/22 - 100%):** Tutte le 22 locandine e marquee servite correttamente.
  - **Game & Watch (15/15 - 100%):** Tutte le 15 foto autentiche dei modellini originali LCD.
  - **Commodore Amiga (25/25 - 100%):** Tutte le 25 copertine originali in `covers/amiga/`.
  - **ScummVM & PC Home:** Collegate le copertine per Broken Sword 1, 2, 2.5, The Settlers II, Simon the Sorcerer, ecc.
- [ ] **Scraper Automatico No-Intro / Redump / ScreenScraper:** Caching locale permanente delle boxart 3D rimanenti.
- [ ] **Steam & GOG Auto-Discovery:** Rilevamento automatico delle installazioni locali PC.
