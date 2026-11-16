# 🗺️ RetroHub - Roadmap di Sviluppo

Questa roadmap definisce le priorità concordate per l'evoluzione di **RetroHub**, dal client locale all'esperienza multigiocatore online.

---

## 📌 FASE 1: Client Locale & Esperienza Utente (Priorità Attuale)
*Obiettivo: Interfaccia moderna, veloce, affidabile e impeccabile per l'intera collezione locale di 1.149 giochi su 16 piattaforme.*

- [x] **Backend Rust Ultra-rapido:** Catalogo indicizzato in memoria con avvio in <100ms e server HTTP integrato (porta 39281).
- [x] **Lancio Affidabile Giochi:** Supporto sicuro a tutti i 16 sistemi ed emulatori (PowerShell `Start-Process`, fix percorsi con spazi e caratteri speciali).
- [ ] **Nuova UI/UX Ispirata a RecompLauncher & Steam:**
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

- [ ] **Room Code Generator:** Generazione di codici univoci di stanza (es. `RH-7821`).
- [ ] **Sincronizzazione Diretta P2P:**
  - Connessione diretta Host/Guest via IP o codice d'invito.
  - Misurazione automatica della latenza (Ping RTT via UDP) per verificare la qualità della partita prima del lancio.
- [ ] **Avvio Automatico Emulatore in Modalità Netplay:**
  - Integrazione parametri da riga di comando per emulatori con netplay nativo:
    - **Flycast** (Dreamcast / Naomi / Atomiswave)
    - **Snes9x** (Super Nintendo)
    - **DuckStation** (PlayStation 1)
    - **Dolphin** (GameCube)
    - **MAME / RetroArch**

---

## 📌 FASE 3: Server Lobby & Bacheca Stanze Pubbliche (Cloud Matchmaking)
*Obiettivo: Creazione di una lobby online in cui i giocatori possono aprire stanze di attesa e chiunque può cercare le partite disponibili e unirsi con un click.*

> **Nota di sviluppo:** L'architettura esatta del server (VPS dedicato, microservizio serverless o relay WebRTC) verrà concordata insieme all'utente prima dell'implementazione.

- [ ] **Server di Segnalazione & Matchmaking:**
  - Registro stanze attive (*"In attesa di sfidante"*).
  - Gestione heartbeat e rimozione automatica stanze inattive.
- [ ] **Bacheca Stanze nel Client RetroHub:**
  - Schermata dedicata *"Arena Online"*.
  - Elenco stanze in tempo reale con filtri per gioco, piattaforma, regione e latenza (ping stimato).
  - Tasto *"Entra nella partita"* con sincronizzazione automatica.
- [ ] **NAT Traversal & Relay (opzionale):**
  - Protocollo STUN / TURN per consentire il gioco anche dietro router domestici senza dover aprire porte manuali.
