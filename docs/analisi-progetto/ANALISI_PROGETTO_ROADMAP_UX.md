# 🎮 RetroHub — Report di Analisi Strategica di Progetto
**Ruolo:** Project Manager & Lead Architect  
**Data Aggiornamento:** 19 Novembre 2026 (Revisione v2.1)  
**Target Progetto:** `C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game`  
**Documento di Riferimento:** [ROADMAP.md](file:///C:/Users/ClaudioDall'Ara/Desktop/Clod/MySide/launcher_game/ROADMAP.md)

---

## 📌 1. Macro Visione & Sintesi Esecutiva

**RetroHub** è un launcher desktop retrogaming in **Rust** ideato per unire la reattività estrema di un binario compilato nativo (<50ms di avvio, RAM <25MB) con l'eleganza di una *10-Foot User Interface* moderna da divano (in stile Steam Deck e RecompLauncher).

### Il Nuovo Paradigma di Prodotto Concordato:
1. **Regola Aurea: Piattaforma Sbloccata = Copertine Locali HD + ROMs 100% On-Demand:**
   * Allo sblocco di una piattaforma, il pacchetto completo delle copertine HD viene scaricato e memorizzato in locale (`covers/<console>/`), garantendo navigazione a 60 FPS nativi a 0ms e consultazione 100% offline.
   * **ROM a 0 Byte su disco:** Nessun romset gigante scaricato preventivamente.
2. **Download ROM "On-The-Fly" al Click:**
   * Al click su **▶ GIOCA**, se la ROM non è presente in locale, il client la scarica al volo in background (`retro_downloads/`), ne verifica l'integrità e avvia istantaneamente l'emulatore.
3. **Dialog Post-Partita Salva-Spazio:**
   * Alla chiusura del gioco, l'utente sceglie se conservare la ROM sul disco (`[ 💾 TIENI SUL PC ]`) o eliminarla per mantenere pulito lo storage (`[ 🗑️ ELIMINA PER FARE SPAZIO ]`).
4. **MAME Curato 1G1R (1 Game 1 ROM Parent-Only):**
   * Default pulito su ~950 capolavori storici unici senza duplicati regionali, cloni o slot machine, con opzione per sbloccare la visualizzazione dei cloni.
5. **Gestione Modulare degli Emulatori (Standard Batocera / RetroBat):**
   * Transizione dall'avvio statico tramite scorciatoie `.lnk`/`.bat` all'architettura standard a doppio binario: **Core Libretro (RetroArch)** per l'80% delle console classiche + **Emulatori Standalone Ottimizzati** per arcade 3D e sistemi pesanti.

---

## ⚙️ 2. Confronto Architetturale: RetroHub vs Batocera vs RetroBat

Come gestiscono gli emulatori e i core i principali standard del settore e come si posiziona RetroHub:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                            FRONTEND (UI / MENU)                             │
│       Batocera: EmulationStation | RetroBat: EmulationStation (Win)         │
│                 RetroHub: Rust Webview (Edge App Mode)                      │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
                    Dispatching per Piattaforma/Console
                                       │
        ┌──────────────────────────────┴──────────────────────────────┐
        ▼                                                             ▼
┌──────────────────────────────┐              ┌──────────────────────────────┐
│     RETROARCH (LIBRETRO)     │              │    EMULATORI STANDALONE      │
│  80% delle Console Classiche │              │   Sistemi 3D Pesanti/Arcade  │
├──────────────────────────────┤              ├──────────────────────────────┤
│ • NES  -> Mesen / FCEUMM     │              │ • PS2       -> PCSX2         │
│ • SNES -> Snes9x / BSNES     │              │ • PS3       -> RPCS3         │
│ • MD   -> Genesis Plus GX    │              │ • GameCube  -> Dolphin       │
│ • GB/A -> mGBA / Gambatte    │              │ • Dreamcast -> Flycast/Redream│
│ • PS1  -> SwanStation        │              │ • Arcade 3D -> Supermodel/MAME│
│ • C64  -> VICE x64sc         │              │ • Xbox/360  -> Xemu / Xenia  │
└──────────────────────────────┘              └──────────────────────────────┘
```

### Tabella Comparativa di Gestione:

| Caratteristica | Batocera.linux | RetroBat (Windows) | RetroHub (Attuale) | RetroHub (Target Roadmap v2.0) |
| :--- | :--- | :--- | :--- | :--- |
| **Architettura Host** | OS dedicato Linux (boot USB/SSD) | Applicazione Windows | Singolo eseguibile Rust (Win) | Singolo eseguibile Rust (Win) |
| **Tempo di Avvio** | 20 - 45 secondi (boot completo) | 5 - 10 secondi | **< 0.1 secondi (istantaneo)** | **< 0.1 secondi (istantaneo)** |
| **Consumo RAM Base** | ~800 MB - 1.2 GB | ~400 - 600 MB | **< 25 MB** | **< 35 MB** |
| **Console Classiche (8/16-bit)** | RetroArch Libretro Cores | RetroArch Libretro Cores | Misto (.bat, .lnk, MAME) | **RetroArch Libretro Cores unificato** |
| **Console 3D e Sistemi Speciali** | Standalone Linux (Dolphin, PCSX2) | Standalone Windows configurati | Standalone parzialmente cablati | **Modulo Plugin Standalone Dinamico** |
| **Configurazione Piattaforme** | `es_systems.cfg` XML | `es_systems.cfg` XML | Hardcoded in `catalog.rs` / `launcher.rs` | **`platforms_manifest.json` modulare** |
| **Consultazione Senza ROM** | Mostra solo sistemi con ROM presenti | Mostra solo sistemi con ROM presenti | Mostra catalogo unificato locale | **Enciclopedia Completa + Sblocco Immagini** |
| **Acquisizione Giochi** | Copia manuale su disco | Copia manuale su disco | ROM locali già presenti | **Download ROM "On-The-Fly" al click** |

---

## 🎨 3. Focus UX/UI: Esperienza "Enciclopedia + On-Demand"

### Flusso Utente Ridisegnato:
1. **Selezione Piattaforma:** L'utente seleziona ad esempio *Super Nintendo* dalla sidebar.
2. **Sblocco Boxart:** L'interfaccia mostra tutte le schede con poster in alta definizione, anno, genere e mappatura controller Xbox/PlayStation.
3. **Stato Gioco:**
   * 🟢 *Se la ROM è già presente:* Pulsante verde **▶ GIOCA SUBITO** (avvio istantaneo a 0ms).
   * ☁️ *Se la ROM non è presente:* Pulsante ciano **☁️ SCARICA & GIOCA** (avvia il download "on-the-fly" con progress bar circolare, verifica e lancia).
4. **Prompt Post-Partita:** Alla chiusura dell'emulatore compare la finestra per decidere se tenere il gioco nei Preferiti permanenti o eliminarlo per risparmiare spazio su disco.

---

## 🗺️ 4. Stato Reale della Roadmap e Prossimi Passi

1. **Fase 1 (Client Locale & Ricerca Fuzzy):** Completata al 100%. Il motore di ricerca Jaro-Winkler e la UI Dark Mode sono operativi.
2. **Fase 2 (Netplay P2P):** Funzionante in rete locale LAN; l'estensione Internet tramite NAT Traversal/STUN è pianificata per la fase finale.
3. **Fase 4 (Personalizzazione Libreria & Download On-The-Fly):** In corso di completamento. L'obiettivo immediato è:
   * Rendere operativo il download delle ROM al click.
   * Integrare il toggle visivo per sbloccare/sfogliare tutte le copertine.
   * Aggiungere la modale post-partita per la gestione dello spazio.
4. **Fase 5 (Modularità 200+ Piattaforme stile Batocera/RetroBat):**
   * Adozione della cartella universale `roms/<sistema>`.
   * Integrazione del dispatcher a doppio binario (RetroArch Core per il retrogaming classico + Standalone per il 3D moderno).

---
*Report archiviato in `docs/analisi-progetto/ANALISI_PROGETTO_ROADMAP_UX.md`.*
