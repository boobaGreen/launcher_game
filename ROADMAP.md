# 🗺️ RetroHub - Roadmap di Sviluppo

Questa roadmap definisce le priorità concordate per l'evoluzione di **RetroHub**, dal client locale all'esperienza multigiocatore online.

---

## 📌 FASE 1: Client Locale & Esperienza Utente (Priorità Attuale)
*Obiettivo: Interfaccia moderna, veloce, affidabile e impeccabile per l'intera collezione locale di 1.149 giochi su 16 piattaforme.*

- [x] **Backend Rust Ultra-rapido:** Catalogo indicizzato in memoria con avvio in <100ms e server HTTP integrato (porta 39281).
- [x] **Lancio Affidabile Giochi:** Supporto sicuro a tutti i 16 sistemi ed emulatori (PowerShell `Start-Process`, fix percorsi con spazi e caratteri speciali).
- [x] **Plugin System Refactoring (Branch: refactor-modular-rust):**
  - [x] **Plugin System per MAME, Gluiz, Snes9x, FCEUX, Project64, Flycast, Dolphin, VICE, DuckStation** - Gestione dinamica emulatori
  - [x] **Configurazione Dinamica** - File JSON per impostazioni
  - [x] **Database Indexes** - Query 4x più veloci
  - [x] **Struttura Modulare** - Codice organizzato in moduli
  - [x] **Auto-rilevamento** - Percorsi emulatori automatici
- [x] **Nuova UI/UX Ispirata a RecompLauncher & Steam:**
  - **Home Screen Cinematografica:** 
    - Hero Carousel in evidenza (Spotlight panoramico con copertina maxi, dettagli, comandi controller e pulsante **▶ GIOCA ORA**).
    - Galleria a scorrimento orizzontale *"Riprendi a Giocare / Preferiti"*.
    - Accesso rapido alle piattaforme con contatore giochi.
  - **Libreria Giochi Professionale:**
    - Card verticali a rapporto d'aspetto poster fisso (~175px x 270px) con angoli arrotondati e hover con glow ciano.
    - Sistema di visualizzazione copertine intelligente: risoluzione nomi Libretro (Atomiswave, Naomi, Dreamcast, Gamecube, MAME Snaps) con cascata automatica `(USA)` / `(Europe)`.
    - Fallback tipografico elegante (stile *"GTI Club 2"*): card scura con badge del sistema e titolo centrato nitido se l'artwork esterno non è reperibile.
  - **Motore di Ricerca Avanzato Multi-campo & Operatore NOT:**
    - Ricerca istantanea su Titolo, Anno, Genere, Sviluppatore e Piattaforma contemporaneamente.
    - Supporto per esclusione: prefisso `-` o parola chiave `NOT` (es: `mario -kart`, `arcade NOT racing`, `1995 capcom`).
  - **Sistema Preferiti & Statistiche:**
    - Salvataggio preferiti persistente su database SQLite locale.
    - Contatore avvii e data ultimo gioco avviato.

---

## 📌 FASE 2: Multiplayer Online Diretto con Codice (P2P Direct Connect)
*Obiettivo: Giocare online 1 contro 1 con un amico senza bisogno di server intermedi dedicati.*

- [x] **Room Code Generator:** Generazione di codici univoci di stanza (es. `RH-7821`).
- [x] **Sincronizzazione Diretta P2P:**
  - Connessione diretta Host/Guest via IP o codice d'invito.
  - Misurazione automatica della latenza (Ping RTT via UDP) per verificare la qualità della partita prima del lancio.
- [x] **Avvio Automatico Emulatore in Modalità Netplay:**
  - Integrazione parametri da riga di comando per emulatori con netplay nativo:
    - **Flycast** (Dreamcast / Naomi / Atomiswave)
    - **Snes9x** (Super Nintendo)
    - **DuckStation** (PlayStation 1)
    - **Dolphin** (GameCube)
    - **MAME / RetroArch**
    - **Daphne** (LaserDisc)

---

## 📌 FASE 3: Server Lobby & Bacheca Stanze Pubbliche (Cloud Matchmaking)
*Obiettivo: Creazione di una lobby online in cui i giocatori possono aprire stanze di attesa e chiunque può cercare le partite disponibili e unirsi con un click.*

> **Nota di sviluppo:** L'architettura esatta del server (VPS dedicato, microservizio serverless o relay WebRTC) verrà concordata insieme all'utente prima dell'implementazione.

- [x] **Server di Segnalazione & Matchmaking:**
  - Registro stanze attive (*"In attesa di sfidante"*).
  - Gestione heartbeat e rimozione automatica stanze inattive (timeout 30 minuti).
  - **API Endpoints Implementati:**
    - `GET /api/lobby/rooms` - Lista pubblica stanze
    - `POST /api/lobby/create` - Creazione stanza ospite
    - `POST /api/lobby/join` - Unione stanza ospite
- [x] **Bacheca Stanze nel Client RetroHub:**
  - Schermata dedicata *"Arena Online"*.
  - Elenco stanze in tempo reale con filtri per gioco, piattaforma, regione e latenza (ping stimato).
  - Tasto *"Entra nella partita"* con sincronizzazione automatica.
- [x] **NAT Traversal & Relay (opzionale):**
  - Protocollo STUN integrato per risoluzione IP pubblico
  - Supporto configurazione nat_traversal
  - STUN servers: Google (stun.l.google.com:19302), Cloudflare (stun.cloudflare.com:3478)

---

## 📌 FASE 4: Personalizzazione Libreria & Download On-Demand (Prossima Fase)

*Obiettivo: Esperienza utente raffinata con selezione piattaforme, download intelligente e gestione ROM.*

### **4.1 Selezione Piattaforme all'Inizio**
- [x] **Wizard Setup Iniziale:** All'avvio, mostra elenco 16+ piattaforme con checkbox
- [x] **Salvataggio Scelte:** Impostazioni salvate in `retrohub.json` (platform_whitelist)
- [x] **UI Dinamica:** Sidebar e filtri mostrano solo le piattaforme selezionate

### **4.2 Catalogo Completo Locali**
- [ ] **Visualizza Tutti i Giochi:** Database centrale con info cover tags per tutte le 1.149 voci
- [ ] **Filtro Piattaforme:** Mostra solo le piattaforme scelte, con impostazione "Mostra Tutte"
- [ ] **Cerca con Tags:** Anno genere sviluppatore con ricerca fuzzy Jaro-Winkler

### **4.3 QWizard Auto-Rilevamento ROM**
- [x] **Scansione Automatica:** Cerca ROM esistenti in C:\Users\*, D:\Emulators, E:\ROMS
- [x] **Mappatura Piattaforme:** Identifica file per estensione nome cartella
- [x] **Spostamento Smart:** Sposta ROM trovate nella struttura corretta RetroHub

### **4.4 Download On-Demand per Immagini**
- [x] **Mirror Libretro CDN:** Copertine boxart automatiche su richiesta
- [x] **Mirror IGDB/WikiTide:** Copertine per giochi Indie/ScummVM
- [x] **Cache Immagini:** 150x210px in cache locale, fallback grafico stilizzato

### **4.5 Download On-Demand per ROM**
- [x] **Database CDN:** URL download per ROM legittime (Abandonware, Internet Archive)
- [x] **Formato JSON:** `{ "id": 123, "rom_sha1": "abc123", "download_url": "https://..." }`
- [x] **Download Progress:** Barra avanzamento con verifica integrità SHA1

### **4.6 Prompt Post-Gioco**
- [ ] **Finestra DiALOG:** "Mantieni o Cancella ROM?" con pulsanti [TENERE] [CANCELLARE]
- [ ] **Imposta Predefinita:** Opzione globale "Tieni sempre" in configurazione
- [ ] **Cronologia:** File scaricati apposta eliminare appesano disco

### **4.7 Gestione Libreria Utente**
- [ ] **Pagina "La Mia Libreria":** Solo giochi con ROM scaricata o preferiti
- [ ] **Pulsante "Aggiorna":** Ricerca nuove ROM in cartelle configurati
- [ ] **Statistiche:** Quanti giochi giocabili, spazio totale, ultimi download
