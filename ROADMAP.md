# 🗺️ RetroHub — Roadmap di Sviluppo Strategica
**Versione Roadmap:** 2.0 (Architettura Modulare & On-Demand)  
**Obiettivo:** Trasformare RetroHub in un hub retrogaming definitivo: leggero come un client Rust nativo, completo ed enciclopedico come Batocera/RetroBat, con gestione modulare di oltre 200 piattaforme e download ROM "on-the-fly".

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

## 📌 FASE 2: Multiplayer Online & Netplay P2P (LAN Funzionante 🟡 / WAN in Sviluppo ⚪)
*Obiettivo: Sfide 1 contro 1 a distanza con amici senza server intermedi a pagamento.*

- [x] **Generatore Codici Stanza:** Creazione token univoci di invito (es. `RH-7821`).
- [x] **Handshake P2P Diretto (Rete Locale / LAN):** Connessione diretta tra Host e Guest via socket UDP.
- [x] **Avvio Sincronizzato Emulatori Netplay:**
  - Parametri da riga di comando per Flycast, Snes9x, DuckStation, Dolphin e MAME.
- [ ] **NAT Traversal Pubblico Reale (WAN / Internet):**
  - Integrazione resolver STUN reale (Google / Cloudflare) per bypass router senza port forwarding manuale.
  - Misurazione RTT effettiva (Ping reale al posto del valore stimato).

---

## 📌 FASE 3: Server Lobby & Bacheca Stanze Pubbliche (Cloud Matchmaker ⚪)
*Obiettivo: Matchmaking pubblico centralizzato con bacheca "In attesa di sfidante".*

- [ ] **Microservizio di Segnalazione Remoto:** Server cloud leggero per coordinamento sessioni attive.
- [ ] **Bacheca Stanze in-App:** Schermata "Arena Online" collegata a stanze pubbliche reali con filtro per latenza e regione.

---

## 📌 FASE 4: Personalizzazione Libreria, Enciclopedia Immagini & Download ROM "On-The-Fly" (Completata 🟢)
*Obiettivo: Nessun bisogno di riempire l'hard disk con terabyte di file: consultazione visiva al 100% dell'intero catalogo e download automatico della ROM al click.*

### **4.1 Enciclopedia & Sblocco Immagini Completo per Piattaforma**
- [x] **Modalità Catalogo Completo (Zero-Disk Footprint):**
  - L'utente seleziona una console o piattaforma e può **sbloccare/sfogliare tutte le copertine e metadati al 100%**, anche se le ROM non sono ancora presenti sul PC.
  - Switch rapido in UI: `[ Mostra Tutto l'Archivio ]` vs `[ Solo Giochi Scaricati ]`.
- [x] **Risoluzione Copertine Multi-Mirror:**
  - Download asincrono boxart ad alta risoluzione da Libretro CDN, Steam CDN e screen snapshot locali.
  - Caching locale automatico in `cache/covers/` per consultazione offline.

### **4.2 Download ROM "On-The-Fly" al Click**
- [x] **Download Trasparente al Lancio:**
  - Se l'utente clicca **☁️ SCARICA & GIOCA** su un titolo non presente in locale, il client scarica la ROM "al volo" tramite `DownloadManager` con feedback visivo e notifica.
  - Caching nella cartella dedicata `downloads/`.
- [x] **Avvio Immediato Post-Download:**
  - Non appena il download termina, la ROM viene registrata come installata e l'emulatore o core di sistema si avvia istantaneamente a schermo intero.

### **4.3 Dialog Intelligente Post-Partita (Salva-Spazio)**
- [x] **Prompt di Chiusura e Gestione Spazio:**
  - Finestra di dialogo modale post-partita integrata in UI:
    ```
    🎮 Partita Conclusa!
    Cosa desideri fare con la ROM scaricata on-the-fly?
    [ 💾 TIENI SUL PC ]    [ 🗑️ ELIMINA PER FARE SPAZIO ]
    ```
  - Rimozione sicura del file scaricato via `POST /api/library/delete` con aggiornamento immediato della libreria.

---

## 📌 FASE 5: Gestione Modulare di Emulatori & Core (Standard Batocera / RetroBat 🧩)
*Obiettivo: Adottare l'architettura modulare di Batocera e RetroBat, supportando l'intero ecosistema di oltre 200 console, arcade e computer d'epoca.*

### **5.1 Architettura Ibrida a Doppio Binario (RetroArch Core + Standalone)**
Come Batocera e RetroBat, la gestione emulatori viene standardizzata in due categorie:
1. **Piattaforme 8/16/32-bit & Computer Classici (80% dei sistemi):**
   - Gestione unificata tramite **RetroArch Libretro Cores** (es. `snes9x_libretro.dll`, `genesis_plus_gx_libretro.dll`, `fceumm_libretro.dll`, `puae_libretro.dll`, `vice_x64sc_libretro.dll`).
   - *Vantaggi:* Shader video CRT uniformi, configurazione controller centralizzata, savestate e rewind garantiti.
2. **Piattaforme 3D Complesse & Arcade Speciali (Standalone Dedicati):**
   - Esecutori dedicati ottimizzati: Dolphin (GameCube/Wii), PCSX2 (PS2), RPCS3 (PS3), DuckStation (PS1), Flycast (Dreamcast/Naomi), Model 2 Emulator, Supermodel (Model 3), Xemu (Xbox), Cemu (Wii U).

### **5.2 Struttura Cartelle ROM Standardizzata**
- Supporto nativo della nomenclatura universale delle cartelle (interscambiabile con RetroBat e Batocera):
  - `roms/nes/`, `roms/snes/`, `roms/megadrive/`, `roms/psx/`, `roms/c64/`, `roms/amiga/`, `roms/mame/`, `roms/neogeo/`, ecc.
- File di configurazione modulare `platforms_manifest.json`:
  - Mappa ogni codice console (`snes`, `psx`, `c64`...) con il relativo nome esteso, estensioni file supportate, emulatore predefinito e core alternativi.

### **5.3 Wizard di Configurazione Percorsi & Auto-Discovery**
- Rilevamento automatico delle installazioni esistenti di RetroArch, Batocera mounts, o cartelle RetroBat su drive `C:`, `D:`, `E:`.
- Nessun percorso personale hardcoded: tutte le directory sono relative o configurabili da interfaccia grafica in-app.

---

## 📊 Matrice di Avanzamento e Priorità Operativa

```mermaid
gantt
    title Piano di Rilascio RetroHub Modulare
    dateFormat  YYYY-MM-DD
    section Fase 1
    Core Rust & Ricerca Fuzzy       :done, 2026-10-01, 2026-11-10
    UI Cinematografica & Gamepad   :done, 2026-11-01, 2026-11-17
    section Fase 4 (Focus Attuale)
    Sblocco Immagini Enciclopedia  :active, 2026-11-18, 2026-11-25
    Download ROM On-The-Fly        :active, 2026-11-20, 2026-11-30
    Prompt Dialog Post-Partita     :2026-11-25, 2026-12-05
    section Fase 5 (Modularità)
    Standardizzazione Cartelle     :2026-12-01, 2026-12-15
    Doppio Binario Libretro/Core   :2026-12-10, 2026-12-30
```
