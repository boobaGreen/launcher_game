---
name: retrohub-catalog
description: Procedure per la gestione del catalogo giochi 1G1R, sincronizzazione tra master_cloud_catalog.json e retro_hub.db, validazione metadati e gestione copertine.
---

# RetroHub Catalog Skill

Questa skill fornisce all'agente le linee guida per manipolare, validare e sincronizzare il catalogo di giochi di **RetroHub**.

## 1. Struttura dei Dati
- **File Cloud Sorgente**: `data/master_cloud_catalog.json` (codificato in UTF-8 con o senza BOM).
- **Database Locale**: `retro_hub.db` (tabella `games`).
- **Standard**: 1G1R (1 Game 1 ROM, no duplicati regionali, zero BIOS/dispositivi hardware).

## 2. Campi Obbligatori per Ogni Gioco
Ogni oggetto JSON deve contenere:
- `id`: intero progressivo univoco
- `title`: titolo autentico del gioco
- `platform`: nome esteso della piattaforma (es. `MAME Arcade`, `Commodore Amiga`, `Laserdisc Arcade`)
- `system_code`: codice standard (es. `MAME`, `AMIGA`, `HYPSEUS`, `C64`, `SNES`, `DOS`, `SCUMMVM`)
- `year`: anno di pubblicazione (intero a 4 cifre)
- `genre`: macro-genere principale (es. `Platform`, `Shoot 'em Up`, `RPG`)
- `developer`: casa di sviluppo
- `publisher`: editore
- `controls`: descrizione controlli (es. `🎮 Gamepad / Arcade Stick`)
- `launch_target`: parametro di avvio (`MAME:<rom>`, percorso `.lnk`, o stringa vuota per on-demand)
- `cover_url`: percorso copertina locale `/api/cover/file/covers/...` o URL CDN

## 3. Script di Verifica & Normalizzazione
Per verificare l'integrità del catalogo ed eliminare incongruenze di denominazione:
```powershell
py scripts/normalize_platforms.py
```

## 4. Linee Guida per le Copertine
- Cartella di destinazione: `covers/<sistema>/<nome_file>.png` (o `.webp`).
- Formato consigliato: WebP / PNG verticale con proporzioni boxart (300x420 px o 600x900 px per Indie/Steam).
- Risoluzione route: Gestita da `src/server.rs` tramite endpoint `/api/cover/file/...`.
