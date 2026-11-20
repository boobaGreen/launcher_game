# 📊 REPORT ANALISI DATABASE GIOCHI — TUTTE LE PIATTAFORME / CONSOLE
## Progetto RetroHub (Side-Project Claudio)

| Campo | Valore |
|-------|--------|
| **Oggetto** | Analisi completa DB giochi, ROM, immagini, tag, metadati, sviluppatori, publisher, competitor RetroBat/Batocera e strategia 1G1R per RetroHub |
| **Autore** | LLM — Hermes Agent (profilo `side-code`) |
| **Data** | 20 Novembre 2026 |
| **Versione** | 1.0.0 |
| **Stato** | Report di analisi e ricerca (non codice esecutivo) |
| **Cartella di destinazione** | `C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game\reports\ANALISI_DB_GIOCHI_LLM_20261120_RETROHUB\` |

---

## 1. EXECUTIVE SUMMARY

RetroHub è un launcher retrogaming scritto in **Rust** con server HTTP locale (`tiny_http`), UI web embedded e database **SQLite** (`retro_hub.db`). Il catalogo attivo contiene **1.699 giochi** su **36 piattaforme attive** (di 41 definite nel `platforms_manifest.json`), secondo lo standard **1G1R** (One Game One ROM — nessun duplicato regionale, cloni, BIOS o diagnostica).

Il report incrocia:
1. **Dati reali estratti** da `data/master_cloud_catalog.json` e `retro_hub.db` (DB locale).
2. **Stime storage e immagini** da `docs/STIME_STORAGE_PIATTAFORME_IMMAGINI.md`.
3. **Analisi competitor** da `reports/COMPETITOR_ANALYSIS_RETROHUB.md` (RetroBat vs Batocera vs RetroHub).
4. **Conoscenza di settore** su No-Intro, Redump, MAME, Libretro-Thumbnails, ScreenScraper (etichettata come riferimento esterno, non output di tool).

**Punti critici emersi:**
- Nessun campo separato per `composer` (musicista) o `programmer` (programmatore): solo `developer` e `publisher` come stringhe (talvolta multiple con separatore `/`).
- `launch_target` mancante su **1.136/1.699 giochi (66,9%)**: i giochi non sono lanciabili direttamente senza configurazione manuale.
- Immagini locali quasi assenti: solo poche decine di file PNG nelle cartelle `covers/` (es. SNES 1, Arcade 4, Amiga 25, Pinball 82). Il restante 100% delle cover punta a URL esterni (CDN `libretro-thumbnails`) o `/api/cover/file/...`.
- I tag non sono presenti nel JSON/catalogo attuale (0 tag per gioco): vanno aggiunti come tabella separata o array nel modello dati.

---

## 2. METODOLOGIA E FONTI

| Fonte | Tipo | Contenuto | Affidabilità |
|-------|------|-----------|--------------|
| `data/master_cloud_catalog.json` | DB locale JSON | 1.699 giochi, 14 campi obbligatori | ✅ Reale, estratto con script Python |
| `retro_hub.db` | SQLite | Tabella `games`, 1.699 righe, stessa struttura | ✅ Reale, ispezionato con `sqlite3` |
| `src/platforms_manifest.json` | Config | 41 piattaforme definite (codice, nome, categoria, produttore, anno, core, estensioni) | ✅ Reale |
| `docs/STIME_STORAGE_PIATTAFORME_IMMAGINI.md` | Stime | Peso boxart, snaps, hero, ROM per piattaforma (Top Classici vs Full Set) | ✅ Reale, interno al progetto |
| `reports/COMPETITOR_ANALYSIS_RETROHUB.md` | Report interno | Confronto RetroBat vs Batocera vs RetroHub (20/11/2026) | ✅ Reale, interno al progetto |
| No-Intro / Redump / MAME / Libretro-Thumbnails / ScreenScraper | Riferimenti di settore | Fonti citate nei report esistenti | ⚠️ Conoscenza generale, non verificata via tool in questa sessione |

**Nota:** Non è stato possibile accedere a fonti web esterne in questa sessione. Tutti i numeri di mercato (es. "NES ha ~716 giochi licenziati") sono riferimenti di settore generici, da confermare con fonti primarie prima di impegni di spesa o produzione.

---

## 3. ANATOMIA DEL DATABASE LOCALE RETROHUB

### 3.1 Schema tabella `games`

| Campo | Tipo | Obbligatorio | Note |
|-------|------|--------------|------|
| `id` | INTEGER | Sì | PK |
| `title` | TEXT | Sì | Titolo gioco |
| `platform` | TEXT | Sì | Nome piattaforma (es. "Commodore 64") |
| `system_code` | TEXT | Sì | Codice piattaforma (es. `C64`, `MAME`, `SNES`) |
| `year` | INTEGER | Sì | Anno di pubblicazione |
| `genre` | TEXT | Sì | Genere (stringa singola) |
| `developer` | TEXT | Sì | Sviluppatore (stringa, a volte multipla `/`) |
| `publisher` | TEXT | Sì | Publisher (spesso uguale a developer per homebrew) |
| `controls` | TEXT | Sì | Descrizione comandi (con emoji e HTML entities) |
| `launch_target` | TEXT | No | Percorso eseguibile/link (mancante su 1.136 titoli) |
| `cover_url` | TEXT | Sì | URL o percorso locale copertina |
| `is_favorite` | INTEGER | Sì | 0/1 |
| `play_count` | INTEGER | Sì | Contatore partite |
| `last_played_at` | TEXT | No | Timestamp ultimo gioco |

**Limite rilevato:** non esistono tabelle o campi per `composer` (musicista), `programmer` (programmatore), `designer`, `tag` (array), `rating`, `region`, `revisione ROM`, `hash CRC/MD5/SHA1`. Per un prodotto commerciale è consigliabile aggiungerli.

### 3.2 Conteggio giochi per piattaforma (dati reali dal catalogo)

| # | Piattaforma | Codice | Categoria | Titoli (1G1R) | % catalogo | Anni min/max |
|---|-------------|--------|-----------|---------------|------------|--------------|
| 1 | Arcade (MAME/FBNeo) | `MAME` | Arcade | 568 | 33,4% | 1978–2024 |
| 2 | Commodore 64 | `C64` | Computer | 223 | 13,1% | 1982–2023 |
| 3 | SNK Neo Geo | `NEOGEO` | Arcade | 120 | 7,1% | 1990–2004 |
| 4 | MS-DOS | `DOS` | Computer | 100 | 5,9% | 1981–2020 |
| 5 | Super Nintendo | `SNES` | Nintendo | 86 | 5,1% | 1990–1995 |
| 6 | ScummVM | `SCUMMVM` | Computer | 64 | 3,8% | 1987–2005 |
| 7 | Sony PlayStation | `PSX` | Sony | 60 | 3,5% | 1994–2006 |
| 8 | Sega Mega Drive | `MEGADRIVE` | Sega | 58 | 3,4% | 1988–1996 |
| 9 | Nintendo NES | `NES` | Nintendo | 54 | 3,2% | 1983–1993 |
| 10 | Game Boy Advance | `GBA` | Nintendo | 51 | 3,0% | 2001–2008 |
| 11 | Recomp PC Native | `RECOMP` | Native PC | 45 | 2,6% | 1996–2026 |
| 12 | Virtual Pinball | `PINBALL` | Pinball | 40 | 2,4% | 1990–2024 |
| 13 | LocoMalito & Indie | `INDIE` | Native PC | 33 | 1,9% | 2004–2024 |
| 14 | Sega NAOMI | `NAOMI` | Arcade | 27 | 1,6% | 1998–2002 |
| 15 | Commodore Amiga | `AMIGA` | Computer | 25 | 1,5% | 1985–1994 |
| 16 | Laserdisc Arcade | `HYPSEUS` | Laserdisc | 22 | 1,3% | 1983–1993 |
| 17 | Nintendo 64 | `N64` | Nintendo | 20 | 1,2% | 1996–1999 |
| 18 | Atari 2600 | `ATARI2600` | Atari | 15 | 0,9% | 1977–1984 |
| 19 | Nintendo Game & Watch | `GAMEWATCH` | LCD | 15 | 0,9% | 1980–1990 |
| 20 | Sammy Atomiswave | `ATOMISWAVE` | Arcade | 8 | 0,5% | 2003–2006 |
| 21 | Sharp X68000 | `X68000` | Nippon | 8 | 0,5% | 1987–1993 |
| 22 | Microsoft MSX2 | `MSX2` | Nippon | 8 | 0,5% | 1985–1991 |
| 23 | Nintendo GameCube | `GAMECUBE` | Nintendo | 5 | 0,3% | 2001–2002 |
| 24 | Sega Dreamcast | `DREAMCAST` | Sega | 5 | 0,3% | 1999–2001 |
| 25 | Sega Hikaru | `HIKARU` | Arcade | 4 | 0,2% | 1999–2001 |
| 26 | Atari 7800 | `ATARI7800` | Atari | 4 | 0,2% | 1986–1989 |
| 27 | Atari Lynx | `LYNX` | Atari | 4 | 0,2% | 1989–1992 |
| 28 | Atari Jaguar | `JAGUAR` | Atari | 4 | 0,2% | 1993–1995 |
| 29 | Mattel Intellivision | `INTELLIVISION` | Pioneers | 4 | 0,2% | 1979–1982 |
| 30 | Microsoft MSX | `MSX` | Nippon | 4 | 0,2% | 1983–1987 |
| 31 | NEC PC-98 | `PC98` | Nippon | 4 | 0,2% | 1982–1995 |
| 32 | GCE Vectrex | `VECTREX` | Atari | 3 | 0,2% | 1982–1983 |
| 33 | ColecoVision | `COLECO` | Pioneers | 3 | 0,2% | 1982–1984 |
| 34 | Fujitsu FM Towns | `FMTOWNS` | Nippon | 3 | 0,2% | 1989–1993 |
| 35 | NEC PC-88 | `PC88` | Nippon | 2 | 0,1% | 1981–1987 |
| | **TOTALE** | | | **1.699** | **100%** | **1978–2026** |

**Piattaforme definite nel manifest ma a 0 titoli nel catalogo attivo:** PS2, PSP, Sega Saturn, PC Engine/TurboGrafx, Mega Drive (già presente), Game Gear, ecc. Vanno popolate se si vuole espandere oltre il 1G1R attuale.

### 3.3 Sviluppatori e Publisher (top 20 reali)

| Sviluppatore / Publisher | N° giochi | Note |
|--------------------------|-----------|------|
| Arcade (generico) | 560 | MAME/arcade senza singolo sviluppatore |
| PC MS-DOS | 98 | DOS generico |
| SNK | 50 | Neo Geo |
| Recomp Community | 44 | Porting PC nativi |
| Capcom | 44 | C64, Arcade, SNES |
| Konami | 39 | Arcade, NES, SNES |
| Nintendo | 20 | NES, SNES, N64, GameBoy |
| Williams | 19 | Arcade/Pinball |
| Classico ScummVM | 18 | Avventure grafiche |
| Nintendo R&D1 | 18 | Game & Watch, NES |
| Square | 16 | SNES, PSX |
| Megastyle / Megastyle | 13 | Homebrew C64 |
| Sega | 13 | Mega Drive, Arcade, C64 |
| Nintendo EAD | 12 | N64, SNES |
| Bally | 12 | Pinball/Arcade |
| Atari | 11 | Atari 2600/7800/Lynx/Jaguar |
| Data East | 11 | Arcade |
| Jeff Minter / Llamasoft | 10 | C64, Arcade |
| Rare | 9 | NES, SNES |
| Visco | 9 | Arcade |

**Nota:** publisher e developer coincidono spesso (specialmente homebrew e arcade generico). Non c'è distinzione tra programmatore, designer e musicista.

### 3.4 Generi (top 20 reali)

| Genere | N° giochi |
|--------|-----------|
| Arcade Classic | 560 |
| Classico MS-DOS | 98 |
| Avventura Grafica Punta e Clicca | 65 |
| Picchiaduro | 50 |
| Platform Action | 46 |
| Platform | 34 |
| RPG | 30 |
| Run and Gun | 29 |
| Action Adventure | 25 |
| Picchiaduro a Scorrimento | 20 |
| Shoot 'em up | 19 |
| Action RPG | 11 |
| Platformer | 10 |
| Platform 3D | 10 |
| Cinematic Platformer | 9 |
| Puzzle Arcade | 9 |
| Strategia RPG | 9 |
| Shmup Orizzontale | 8 |
| Puzzle Platformer | 8 |
| Sport / Calcio | 8 |

### 3.5 Qualità dei dati (campi mancanti)

| Campo | Mancanti / Vuoti | % |
|-------|------------------|---|
| `launch_target` | 1.136 | 66,9% |
| `cover_url` | 0 | 0% |
| `developer` | 0 | 0% |
| `publisher` | 0 | 0% |
| `genre` | 0 | 0% |
| `year` | 0 | 0% |
| `platform` | 0 | 0% |
| `title` | 0 | 0% |

**Problema critico:** oltre i 2/3 dei giochi non hanno un `launch_target` valorizzato, quindi non possono essere avviati automaticamente dal launcher senza intervento manuale dell'utente.

### 3.6 Peso stimato dati (metadati + immagini + tag)

Basandosi su `docs/STIME_STORAGE_PIATTAFORME_IMMAGINI.md`:

| Elemento | Peso unitario | Note |
|----------|---------------|------|
| Metadati gioco (SQLite) | ~1 KB | Record indicizzato |
| Boxart frontale 2:3 (400×560) | ~110 KB | PNG/WebP |
| Screenshot / Title Screen | ~95 KB | WebP |
| Hero Widescreen (1920×620) | ~300 KB | Asset cinematografico |
| Tag per gioco | 10 tag (media) | Non attualmente presenti nel DB |

Per l'intero catalogo 1G1R (1.699 giochi):
- **Metadati:** ~1,7 MB
- **Boxart solo:** ~187 MB
- **Boxart + Snaps:** ~350 MB
- **Boxart + Snaps + Hero:** ~850 MB

Vedi sezione 8 per il dettaglio per piattaforma.

---

## 4. CONFRONTO COMPETITOR: RETROBAT vs BATOCERA vs RETROHUB

Dati dal report interno `COMPETITOR_ANALYSIS_RETROHUB.md` (20/11/2026).

### 4.1 Arcade

| Piattaforma | RetroBat | Batocera | RetroHub |
|-------------|----------|----------|----------|
| MAME | 568 | 568 | 568 |
| Neo Geo | 120 | 120 | 120 |
| Atomiswave | 8 | 8 | 8 |
| Naomi | 27 | 25 | 27 |
| FinalBurn Neo | – | 200+ | – |
| **Totale Arcade** | **723** | **921+** | **723** |

### 4.2 Home Computer

| Piattaforma | RetroBat | Batocera | RetroHub |
|-------------|----------|----------|----------|
| C64 | 223 | 223 | 223 |
| Amiga | 25 | 25 | 25 |
| DOS | 100 | 100 | 100 |
| MSX | 8 | 8 | 8 |
| X68000 | 8 | 8 | – |
| **Totale HC** | **364** | **364** | **356** |

### 4.3 Console Nintendo

| Piattaforma | RetroBat | Batocera | RetroHub |
|-------------|----------|----------|----------|
| NES | 54 | 54 | 54 |
| SNES | 86 | 86 | 86 |
| N64 | 20 | 20 | 20 |
| GameCube | 5 | 5 | 5 |
| **Totale Nintendo** | **165** | **165** | **165** |

### 4.4 Fonti dati

| Fonte | Tipo | Utilizzo | Note |
|-------|------|----------|------|
| No-Intro | Hash ROM | Integrità | Database verificato |
| Redump | ISO/CD | Computer/Console | Verifica CD-ROM |
| MAME Official | Dati Tecnici | Arcade | CHD, device info |
| Libretro-Thumbnails | Cover Art | Boxart/Snaps | CDN gratuita |
| ScreenScraper | Metadati | Multi-language | ~150 MB |

RetroHub usa esclusivamente `master_cloud_catalog.json` + Libretro-Thumbnails + MAME Official. Non integra ScreenScraper né metadati multi-lingua.

### 4.5 Strategia 1G1R raccomandata (da report interno)

1. MAME Arcade → 568 titoli (1G1R)
2. C64 → 223 titoli (1G1R)
3. Neo Geo → 120 titoli (1G1R)
4. DOS → 100 titoli (1G1R)
5. SNES → 86 titoli (1G1R)
6. ScummVM → 64 titoli (1G1R)
7. PS1 → 60 titoli (1G1R)
8. Mega Drive → 58 titoli (1G1R)
9. NES → 54 titoli (1G1R)
10. GBA → 51 titoli (1G1R)
11. Wii → nuova aggiunta, 45 titoli (1G1R)

**Totale 1G1R aggiornato: ~625 titoli** (versione proposta nel report, inferiore ai 1.699 attuali perché il report parlava di una selezione "top").

---

## 5. Tassonomia e raggruppamento piattaforme

### 5.1 Come RetroHub raggruppa (dal `platforms_manifest.json`)

Il manifest definisce **categorie** di alto livello:

| Categoria | Piattaforme esemplificative |
|-----------|----------------------------|
| Nintendo | SNES, NES, N64, GameCube, GBA, GB, Game & Watch |
| Sega | Mega Drive, Master System, Saturn, Dreamcast |
| Sony | PSX, PS2, PSP |
| Arcade | MAME, Neo Geo, Naomi, Atomiswave, Hikaru |
| Computer | C64, Amiga, VIC20, Plus/4, DOS, ScummVM |
| NEC | PC Engine, PC-88, PC-98 |
| Nippon | MSX, MSX2, X68000, PC-88, PC-98, FM Towns |
| Atari | Atari 2600, 7800, Lynx, Jaguar, Vectrex |
| Pinball | PINBALL (VPX/Future) |
| Laserdisc | HYPSEUS (Daphne) |
| LCD | Game & Watch |
| Native PC | RECOMP, INDIE |
| Pioneers | ColecoVision, Intellivision |

### 5.2 Come si posizionano i competitor

- **RetroBat:** frontend Windows, raggruppa per "sistema" con romset No-Intro/Redump. Forte su Arcade (MAME/FBNeo) e console classiche. Tassonomia per generazione hardware.
- **Batocera:** distribuzione Linux embedded, sistema molto ampio (centinaia di sistemi), organizzazione per categoria (Arcade, Console, Computer, Handheld, Pinball). Usa EmulationStation con metadati da ScreenScraper.
- **RetroHub (attuale):** 12 macro-famiglie (Arcade, Home Computer, Nintendo, Sega, Sony, Atari, LCD/Game Watch, Laserdisc, Pinball, PC Recomp/Indie, SNK/NEC, Nippon PC). Tassonomia più ristretta e orientata ai "top titoli" selezionati a mano (1G1R curato).

### 5.3 Ipotesi di miglioramento tassonomia per RetroHub

Per un prodotto pronto al mercato, suggerisco di aggiungere un livello **Generazione + Regione**:

```
Console > Generazione (8-bit, 16-bit, 32-bit, 64-bit, 128-bit) > Piattaforma > Regione (JAP/USA/EUR) > Gioco
Computer > Home Computer (8-bit, 16-bit) > Piattaforma > Lingua (ITA/ENG/FRA/DEU) > Gioco
Arcade > Hardware (MAME, Neo Geo, Naomi, Atomiswave, Hikaru, Laserdisc) > Gioco
```

Questo consente filtri per "tutti i giochi 16-bit" o "tutti i giochi JPN" senza dover scorrere piattaforma per piattaforma.

---

## 6. MATRICE COMPLETA PER PIATTAFORMA (ROM totali, 1G1R, peso info/immagini/tag)

Le colonne "Full Set (rif. settore)" sono stime di mercato pubblico (No-Intro/Redump) e vanno confermate. Le colonne "RetroHub 1G1R attuale" sono dati reali estratti.

| Piattaforma | Codice | RetroHub 1G1R attuale | Full Set stimato (rif. settore) | Peso info (metadati) | Peso immagini (boxart solo) | Tag completi | Tag 1G1R | Note immagini |
|-------------|--------|----------------------|--------------------------------|----------------------|----------------------------|--------------|----------|---------------|
| Arcade MAME | `MAME` | 568 | ~950 (parent-only) / ~3.200 (full cloni) | ~0,6 MB | ~62 MB | 0 | 0 | Cover su CDN, locale solo 4 file |
| Commodore 64 | `C64` | 223 | ~3.000 | ~0,2 MB | ~25 MB | 0 | 0 | Cover su CDN, locale assente |
| SNK Neo Geo | `NEOGEO` | 120 | ~150 | ~0,1 MB | ~13 MB | 0 | 0 | Cover su CDN |
| MS-DOS | `DOS` | 100 | ~1.200 | ~0,1 MB | ~11 MB | 0 | 0 | Cover su CDN |
| Super Nintendo | `SNES` | 86 | ~780 | ~0,1 MB | ~9 MB | 0 | 0 | **1 solo file locale** |
| ScummVM | `SCUMMVM` | 64 | ~180 | ~0,1 MB | ~7 MB | 0 | 0 | 12 file locali |
| Sony PlayStation | `PSX` | 60 | ~1.450 | ~0,1 MB | ~7 MB | 0 | 0 | Cover su CDN |
| Sega Mega Drive | `MEGADRIVE` | 58 | ~880 | ~0,1 MB | ~6 MB | 0 | 0 | Cover su CDN |
| Nintendo NES | `NES` | 54 | ~710 | ~0,05 MB | ~6 MB | 0 | 0 | Cover su CDN |
| Game Boy Advance | `GBA` | 51 | ~1.050 | ~0,05 MB | ~6 MB | 0 | 0 | Cover su CDN |
| Recomp PC | `RECOMP` | 45 | ~45 | ~0,05 MB | ~5 MB | 0 | 0 | 1 file locale |
| Virtual Pinball | `PINBALL` | 40 | ~120 | ~0,04 MB | ~4 MB | 0 | 0 | **82 file locali** |
| LocoMalito & Indie | `INDIE` | 33 | ~60 | ~0,03 MB | ~4 MB | 0 | 0 | Cover su CDN |
| Sega NAOMI | `NAOMI` | 27 | ~27 | ~0,03 MB | ~3 MB | 0 | 0 | Cover su CDN |
| Commodore Amiga | `AMIGA` | 25 | ~1.800 | ~0,03 MB | ~3 MB | 0 | 0 | **25 file locali** |
| Laserdisc | `HYPSEUS` | 22 | ~45 | ~0,02 MB | ~2 MB | 0 | 0 | **36 file locali** |
| Nintendo 64 | `N64` | 20 | ~388 | ~0,02 MB | ~2 MB | 0 | 0 | Cover su CDN |
| Atari 2600 | `ATARI2600` | 15 | ~500 | ~0,02 MB | ~2 MB | 0 | 0 | Cover su CDN |
| Game & Watch | `GAMEWATCH` | 15 | ~60 | ~0,02 MB | ~2 MB | 0 | 0 | **28 file locali** |
| Sammy Atomiswave | `ATOMISWAVE` | 8 | ~8 | ~0,01 MB | ~1 MB | 0 | 0 | Cover su CDN |
| Sharp X68000 | `X68000` | 8 | ~420 | ~0,01 MB | ~1 MB | 0 | 0 | Cover su CDN |
| Microsoft MSX2 | `MSX2` | 8 | ~8 | ~0,01 MB | ~1 MB | 0 | 0 | Cover su CDN |
| Nintendo GameCube | `GAMECUBE` | 5 | ~650 | ~0,01 MB | ~1 MB | 0 | 0 | Cover su CDN |
| Sega Dreamcast | `DREAMCAST` | 5 | ~620 | ~0,01 MB | ~1 MB | 0 | 0 | Cover su CDN |
| Sega Hikaru | `HIKARU` | 4 | ~4 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| Atari 7800 | `ATARI7800` | 4 | ~500 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| Atari Lynx | `LYNX` | 4 | ~200 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| Atari Jaguar | `JAGUAR` | 4 | ~200 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| Mattel Intellivision | `INTELLIVISION` | 4 | ~125 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| Microsoft MSX | `MSX` | 4 | ~8 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| NEC PC-98 | `PC98` | 4 | ~4 | ~0,004 MB | <1 MB | 0 | 0 | Cover su CDN |
| GCE Vectrex | `VECTREX` | 3 | ~3 | ~0,003 MB | <1 MB | 0 | 0 | Cover su CDN |
| ColecoVision | `COLECO` | 3 | ~3 | ~0,003 MB | <1 MB | 0 | 0 | Cover su CDN |
| Fujitsu FM Towns | `FMTOWNS` | 3 | ~3 | ~0,003 MB | <1 MB | 0 | 0 | Cover su CDN |
| NEC PC-88 | `PC88` | 2 | ~2 | ~0,002 MB | <1 MB | 0 | 0 | Cover su CDN |
| **TOTALE** | | **1.699** | **~15.000+ (full set esteso)** | **~1,7 MB** | **~187 MB** | **0** | **0** | **Immagini locali: solo ~193 file su 1.699** |

**Chiave di lettura:**
- **Tag completi / Tag 1G1R:** attualmente **0** nel catalogo JSON e nel DB. Il campo non esiste. Se si aggiunge una tabella tag con 10 tag per gioco (media dal report architettura), si arrivano a ~17.000 tag per il catalogo attuale.
- **Peso info:** stimato a 1 KB per record indicizzato in SQLite.
- **Peso immagini:** stimato a 110 KB per boxart (dati `STIME_STORAGE_PIATTAFORME_IMMAGINI.md`).

---

## 7. Analisi Immagini: stato attuale, problemi e come comportarsi

### 7.1 Stato attuale delle immagini locali

Eseguito conteggio file nelle cartelle `covers/`:

| Cartella | N° immagini locali |
|----------|-------------------|
| `covers/pinball/` | 82 |
| `covers/gamewatch/` | 28 |
| `covers/laserdisc/` | 36 |
| `covers/amiga/` | 25 |
| `covers/scummvm/` | 12 |
| `covers/atari/` | 6 |
| `covers/arcade/` | 4 |
| `covers/snes/` | 1 |
| `covers/recomp/` | 1 |
| `covers/pc/` | 2 |
| **Altre (c64, mame, nes, genesis, psx, ecc.)** | **0** |

**Totale immagini locali:** circa **193 file** su **1.699 giochi** (11,4%).

### 7.2 Problemi identificati

1. **Dipendenza da CDN esterna:** il 100% delle `cover_url` punta a `raw.githubusercontent.com/libretro-thumbnails/...` o a `/api/cover/file/...`. Senza connessione, la griglia mostra placeholder o fallback tipografici.
2. **Nessuna immagine locale per le piattaforme principali:** C64 (223 giochi), MAME (568), NES (54), SNES (86), Mega Drive (58), PSX (60) hanno **zero** file PNG locali.
3. **Nessuna snap / marquee / wheel / hero:** nel catalogo esiste solo `cover_url`. Non ci sono campi per screenshot in-game, marquee arcade, ruote di selezione o banner hero widescreen.
4. **Nessuna gestione diritti:** le immagini da libretro-thumbnails sono gratuite ma non c'è traccia di licenza o attributione nel DB.
5. **Nessun fallback progressivo:** se l'immagine manca, non c'è una gerarchia (cover → snap → titolo vettoriale → placeholder colorato).

### 7.3 Strategia consigliata (prodotto pronto al mercato)

| Livello | Azione | Costo storage | Beneficio |
|---------|--------|---------------|-----------|
| **L0 – Offline sicuro** | Card vettoriali dinamiche (colori console, font retro) per tutti i giochi senza immagine | 0 KB | Navigazione sempre fruibile offline |
| **L1 – Boxart on-demand** | Scarica boxart solo per titoli visualizzati o preferiti (cache LRU 150 MB) | ~110 KB/titolo | Griglia veloce, zero bloat iniziale |
| **L2 – Pre-download piattaforma** | Pacchetto boxart HD per piattaforma sbloccata (es. SNES ~85 MB, C64 ~25 MB, MAME ~105 MB) | Decine-hundreds MB | 60 FPS nativi, zero latenza rete |
| **L3 – Full offline** | Boxart + Snaps + Hero per tutte le piattaforme selezionate | Centinaia MB – pochi GB | Esperienza tipo Steam/Netflix senza rete |

**Raccomandazione per RetroHub:** adottare il **Profilo 1 Smart On-Demand** (già descritto in `STIME_STORAGE_PIATTAFORME_IMMAGINI.md`) come default, con opzione **Profilo 4 Full Pack Offline** per utenti con SSD capienti.

---

## 8. Musicisti, Programmatori, Sviluppatori: stato e gap

### 8.1 Campi attualmente disponibili

| Campo | Presenza | Tipologia | Note |
|-------|----------|-----------|------|
| `developer` | ✅ Sì | Stringa | Spesso contiene più nomi separati da `/` (es. "Capcom / Micronics / Elite Systems") |
| `publisher` | ✅ Sì | Stringa | Spesso uguale a developer |
| `composer` / musicista | ❌ No | — | Assente |
| `programmer` / programmatore | ❌ No | — | Assente |
| `designer` | ❌ No | — | Assente |
| `credits` / note | ❌ No | — | Assente |

### 8.2 Impatto commerciale

Per un prodotto che vuole vendere o attrarre utenti appassionati (es. schede gioco con "musiche di Koji Kondo", "programmato da Yu Suzuki"), l'assenza di campi dedicati a musicisti e programmatori è un **limite serio**. I competitor (ScreenScraper, GamesDB, IGDB) offrono questi crediti.

### 8.3 Proposta estensione modello dati

Aggiungere al modello `Game` (o tabella separata `game_credits`):

```rust
struct GameCredits {
    game_id: i32,
    role: String,        // "Composer", "Programmer", "Designer", "Producer", "Artist"
    name: String,        // "Koji Kondo", "Yu Suzuki", ecc.
    is_primary: bool,    // true se principale
}
```

Oppure campi denormalizzati per i casi d'uso più comuni:
- `composer: Option<String>`
- `lead_programmer: Option<String>`
- `designer: Option<String>`

---

## 9. Ipotesi organizzazione ROM per piattaforma (quante totali, quante 1G1R)

Di seguito una proposta di piano editoriale per RetroHub, distinguendo **catalogo attuale 1G1R** (dati reali) da **full set obiettivo** (stime di settore da confermare).

### 9.1 Tier 1 – Console principali (priorità monetizzazione)

| Piattaforma | 1G1R attuale | Full set obiettivo (1G1R) | Peso ROM full set (stima) | Note |
|-------------|--------------|---------------------------|---------------------------|------|
| NES | 54 | ~716 | ~140 MB | Core fceumm |
| SNES | 86 | ~780 | ~1,1 GB | Core snes9x |
| Mega Drive | 58 | ~880 | ~1,0 GB | Core genesis_plus_gx |
| N64 | 20 | ~388 | ~4,6 GB | Core mupen64 |
| GameCube | 5 | ~650 | ~780 GB | Core dolphin |
| PSX | 60 | ~1.450 | ~580 GB | Core duckstation |
| PS2 | 0 | ~2.000 | ~3,6 TB | Core pcsx2 (da valutare) |
| Dreamcast | 5 | ~620 | ~370 GB | Core flycast |

### 9.2 Tier 2 – Computer e Arcade

| Piattaforma | 1G1R attuale | Full set obiettivo (1G1R) | Peso ROM full set | Note |
|-------------|--------------|---------------------------|-------------------|------|
| C64 | 223 | ~3.000 | ~180 MB | Core vice |
| Amiga | 25 | ~1.800 | ~2,7 GB | Core puae (WinUAE mancante) |
| DOS | 100 | ~1.200 | ~18 GB | Core dosbox_pure |
| Arcade MAME | 568 | ~950 (parent) | ~28 GB | Core mame |
| Neo Geo | 120 | ~150 | ~3,7 GB | Core fbneo |
| NAOMI | 27 | ~27 | ~28 GB | Core flycast |

### 9.3 Tier 3 – Handheld e di nicchia

| Piattaforma | 1G1R attuale | Full set obiettivo | Peso ROM | Note |
|-------------|--------------|--------------------|----------|------|
| GBA | 51 | ~1.050 | ~6,3 GB | Core mgba |
| GB/GBC | 0 | ~1.000 | ~400 MB | Core gambatte |
| Atari 2600 | 15 | ~500 | ~5 MB | Core stella |
| MSX / X68000 / PC-88 / PC-98 | ~22 | varie | poche centinaia MB | Core di nicchia |

### 9.4 Totale stimato (full set 1G1R)

| Livello | N° piattaforme | N° giochi 1G1R | Peso ROM stimato |
|---------|----------------|----------------|------------------|
| Solo Tier 1 (console top) | 8 | ~7.484 | ~6,5 TB |
| Tutto il catalogo proposto | ~30 | ~15.000+ | ~15-20 TB |

**Conclusione:** un full set completo è fuori portata per un side project. La strada vincente è **1G1R curato su Top 10-15 piattaforme** (circa 1.000-1.200 giochi) + on-demand per il resto.

---

## 10. Aree di rischio e raccomandazioni operative

### 10.1 Rischi critici

| Rischio | Gravità | Descrizione | Mitigazione |
|---------|---------|-------------|-------------|
| `launch_target` mancante (66,9%) | Alta | 1.136 giochi non lanciabili automaticamente | Compilare mapping titolo → percorso `.lnk` o eseguire `rom_scanner` su `emu/` per popolare i link |
| Dipendenza CDN immagini | Alta | Offline la griglia è vuota per 88% dei giochi | Implementare profilo vettoriale L0 + cache on-demand L1 |
| Nessun campo composer/programmer | Media | Schede gioco anonime, meno appeal per collezionisti | Aggiungere tabella `game_credits` |
| Percorsi hardcoded | Media | App non portabile su altri PC | Estrarre in `config.rs` / variabili d'ambiente |
| UI monolitica `ui.html` (~4.000 righe) | Media | Manutenzione difficile | Separare in CSS/JS o template engine |
| Tag assenti | Bassa | Filtri per stile/epoca impossibili | Aggiungere tabella `tags` con relazione many-to-many |

### 10.2 Raccomandazioni priorità alta (prima del lancio MVP)

1. **Completare `launch_target`** per tutti i 1.699 titoli (scansione `emu/` + mappatura automatica).
2. **Scaricare boxart locali** per le Top 10 piattaforme (pacchetto ~450 MB) e attivare navigazione offline.
3. **Aggiungere campi `composer` e `programmer`** al modello dati per le schede gioco dei top titoli.
4. **Creare tabella `tags`** (es. "platformer", "shmup", "co-op", "italiano", "1980s") con collegamento many-to-many a `games`.
5. **Implementare fallback vettoriale** per giochi senza immagine (card colorate per console).
6. **Documentare fonti immagini** (libretro-thumbnails) e aggiungere attributizione nella UI.

### 10.3 Raccomandazioni priorità media (post-MVP)

- Integrazione ScreenScraper per metadati multi-lingua.
- Supporto regioni (JAP/USA/EUR) con filtro toggle.
- Hero artwork widescreen per vetrina principale.
- Sincronizzazione cloud del catalogo (master_cloud_catalog.json su Git/S3).
- Plugin per emulatori mancanti (WinUAE per Amiga, PCSX2 per PS2, ecc.).

---

## 11. Conclusione

Il database RetroHub è **solido come scheletro (1.699 titoli, 36 piattaforme, 1G1R, SQLite)** ma **acerbo come prodotto commerciale**:

- Mancano due terzi dei `launch_target`.
- Le immagini locali sono quasi assenti (solo ~193 file su 1.699).
- Non esistono campi per musicisti e programmatori.
- I tag non esistono come struttura dati.

Il percorso consigliato è concentrarsi su un **MVP 1G1R delle Top 10 piattaforme** (circa 1.000 titoli), con boxart locali pre-scaricate, crediti sviluppatore/publisher/completati, e un sistema di download on-demand per le ROM. Questo permette di lanciare un prodotto leggero (<1 GB di asset iniziali), navigabile offline, e scalabile verso full set successivi.

---

**Fine del report.**

**Percorso file:** `C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game\reports\ANALISI_DB_GIOCHI_LLM_20261120_RETROHUB\REPORT_DB_GIOCHI_PIATTAFORME_RETROHUB_LLM_20261120.md`  
**Data completamento:** 20 Novembre 2026  
**Autore:** LLM — Hermes Agent (profilo `side-code`)
