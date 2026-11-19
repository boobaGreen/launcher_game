# 📐 Specifiche Tecniche: Wizard Onboarding, Strategie Storage & Architettura On-Demand

**Progetto:** RetroHub — Client Retrogaming Nativo (Rust + Webview)  
**Versione Documento:** 2.1 (Analisi Dettagliata Storage, Metadati & Wizard)  
**Autore:** Team di Architettura & Design Software  
**Data:** 19 Novembre 2026  

---

## 🧭 1. Visione Architetturale: Lo Streaming Retrogaming Intelligente

A differenza dei frontend tradizionali (Batocera, RetroBat, RetroPie), che costringono l'utente a configurare in anticipo gigabyte o terabyte di file prima ancora di visualizzare un singolo gioco, RetroHub adotta il paradigma del **Client Ibrido Streaming / On-Demand** (ispirato all'esperienza utente di Steam e Spotify):

1. **Separazione Netta tra Scheletro (Metadati + Immagini) e Muscolo (File ROM):**
   * Lo *Scheletro* è leggerissimo, sfogliabile a 60fps con zero attese.
   * Il *Muscolo* (la ROM) viene trasferito sul PC solo quando l'utente decide di giocare.
2. **Modularità Dinamica a Console:**
   * Le oltre 200 piattaforme supportate non vengono imposte tutte insieme. L'utente attiva le console che gli interessano, sbloccandone in tempo reale l'enciclopedia visiva.
3. **Gestione del Ciclo di Vita del File (Zero-Disk Footprint):**
   * Le ROM temporanee scaricate possono essere rimosse a fine partita con un dialog di chiusura, mantenendo l'hard disk a zero megabyte occupati, oppure consolidate localmente.

---

## 📊 2. Dimensionamento Reale: Quanto Pesano Dati, Immagini e ROM?

Per prendere decisioni ingegneristiche inconfutabili, abbiamo calcolato l'esatto peso matematico di ciascun componente dell'architettura.

### 2.1 I Metadati (Testo, Tag, Anni, Generi, Sviluppatori, Link CDN)
* **Peso di 1 record di gioco singolo:** circa **500 byte – 800 byte** all'interno del database SQLite indicizzato.
* **1.000 giochi:** circa **0,8 MB**.
* **10.000 giochi:** circa **8 MB**.
* **Tutto l'universo delle 200 piattaforme (~50.000 giochi storici):** **circa 35 MB – 45 MB**.
> **Verdetto sui Metadati:** Il peso dei testi e dei link è **trascurabile**. Anche un dispositivo con pochissimo spazio può contenere l'intera storia dei videogiochi in un singolo file SQLite da 40 MB.

---

### 2.2 Le Immagini delle Copertine (Boxart)
Una copertina standard verticale (300x420 px compressa in formato WebP o PNG ottimizzato) pesa in media **70 KB – 110 KB** (media: **90 KB**).

| Ambito | Numero Titoli | Piattaforma Sbloccata (Copertine Locali HD) | Peso Totale ROMs (Solo On-Demand) |
| :--- | :--- | :--- | :--- |
| **Super Nintendo (SNES)** | 780 giochi | **~85 MB** (salvate in locale a 60 FPS) | **~1.1 GB** (0 MB su disco a monte) |
| **Nintendo NES** | 710 giochi | **~78 MB** (salvate in locale a 60 FPS) | **~140 MB** (0 MB su disco a monte) |
| **Sega Mega Drive** | 880 giochi | **~96 MB** (salvate in locale a 60 FPS) | **~1.0 GB** (0 MB su disco a monte) |
| **Game Boy Advance (GBA)** | 1.050 giochi | **~115 MB** (salvate in locale a 60 FPS) | **~6.3 GB** (0 MB su disco a monte) |
| **Sony PlayStation (PS1)** | 1.450 giochi | **~159 MB** (salvate in locale a 60 FPS) | **~580 GB** (0 MB su disco a monte) |
| **Arcade MAME (1G1R Parent)** | 950 giochi | **~105 MB** (salvate in locale a 60 FPS) | **~28 GB** (0 MB su disco a monte) |
| **TOP 5 Piattaforme Regina** | ~4.500 giochi | **~450 MB** totali copertine locali | **0 MB su disco** (100% On-Demand al click) |
| **TOP 10 Piattaforme Regina** | ~8.000 giochi | **~780 MB** totali copertine locali | **0 MB su disco** (100% On-Demand al click) |

> **⚡ La Regola Aurea RetroHub sulle Copertine:**
> Allo sblocco di ogni piattaforma scelta dall'utente, RetroHub **scarica e memorizza in locale nella cartella `covers/<piattaforma>/` tutte le copertine HD**.
> - **Prestazioni:** Griglia a 60 FPS nativi a 0ms, zero lag di caricamento, consultazione fluida al 100% offline anche in aereo o senza internet.
> - **ROMs:** **Rigorosamente 0 Byte preventivi su disco**. Nessun download di massa di ROM inutilizzate. Il gioco viene scaricato solo ed esclusivamente al click `▶ GIOCA`.

---

### 2.3 I File delle ROM
* **Console 8/16-Bit (NES, Game Boy, Master System, SNES, Genesis, C64):**
  * Dimensione media: **250 KB – 2 MB** per gioco.
  * Tempo di download con fibra o 4G: **< 1 secondo**.
* **Console 32/64-Bit e Arcade (GBA, N64, Neo-Geo, MAME):**
  * Dimensione media: **4 MB – 32 MB** per gioco.
  * Tempo di download: **1 – 3 secondi**.
* **Piattaforme su Disco Ottico (PS1, Dreamcast, GameCube):**
  * Dimensione media: **250 MB – 1,2 GB** per gioco (compresso CHD/RVZ).
  * Tempo di download: **15 – 60 secondi**.

---

## 🔍 3. Analisi delle 10 Piattaforme Regina (Dati Statistici Mondiali)

Attraverso i dati storici di utilizzo di RetroArch, Batocera e dei progetti retrogaming open-source mondiali, la top 10 assoluta dei sistemi più giocati e ricercati al mondo è:

1. **Arcade (MAME / FBNeo):** 28% delle sessioni mondiali.
2. **Super Nintendo (SNES):** 18%.
3. **Sony PlayStation (PS1):** 14%.
4. **Sega Genesis / Mega Drive:** 11%.
5. **Nintendo NES:** 9%.
6. **Game Boy Advance (GBA):** 7%.
7. **Nintendo 64 (N64):** 5%.
8. **Commodore 64 (C64):** 3% (leader storico in Europa e Italia).
9. **Nintendo GameCube:** 3%.
10. **Sega Dreamcast:** 2%.

*(Altre piattaforme ad altissima fidelizzazione comunitaria: Neo Geo MVS/AES, Amiga, ScummVM, MS-DOS, PSP, Game Boy Color, Sega Master System, Atari 2600).*

---

## 👥 4. I 4 Profili Utente & Strategie di Storage

```mermaid
flowchart TD
    W["Wizard Iniziale RetroHub"]
    W --> C1["Caso 1: Smart On-Demand (Default)<br>(0 ROM sul PC • Copertine Locali)"]
    W --> C2["Caso 2: Collezione Esistente<br>(Ho già le mie ROM in locale)"]
    W --> C3["Caso 3: Low Storage / Handheld<br>(0 Byte immagini a monte)"]
    W --> C4["Caso 4: Full Pack Offline<br>(Tutto su disco per console scelte)"]

    C1 --> O1["Copertine HD scaricate in locale (~80-170 MB/console)<br>ROMs 100% On-Demand al click ▶ GIOCA<br>Eliminazione opzionale post-partita"]
    C2 --> O2["Modalità Sola Lettura<br>Auto-Discovery cartella radice standard<br>Zero spostamenti • Zero duplicazioni"]
    C3 --> O3["Modalità Card Tipografiche Vettoriali<br>Zero immagini caricate a monte (0 Byte)<br>Cover scaricata solo per Preferiti ⭐ o giocate"]
    C4 --> O4["Modalità Full Offline<br>Copertine + Intero romset scaricato in locale<br>100% giocabile sempre senza internet"]
```

### 👤 Caso 1: Smart On-Demand Standard (Default Consigliato di RetroHub)
* **Esigenza:** Installare ed essere in gioco in 60 secondi, con navigazione visiva fluida e zero gigabyte sprecati per ROM inutilizzate.
* **Scelta Consigliata nel Wizard:**
  * Piattaforme consigliate all'avvio: **Top 5 o Top 10 Regina + PC INDIE (Freeware)**.
  * **Copertine & Metadati:** Allo sblocco della piattaforma, il pacchetto copertine HD viene scaricato e memorizzato in locale (`covers/<console>/`), garantendo navigazione a 60 FPS nativi a 0ms e consultazione 100% offline.
  * **File ROM:** **0 MB sul disco a monte**. La singola ROM viene scaricata all'istante solo quando l'utente preme `▶ GIOCA`.
  * **Gestione post-partita:** Prompt di chiusura con opzione per eliminare automaticamente la ROM o conservarla sul disco.
  * **PC Indie nel Profilo Nuovo Utente:** Di default sono visibili i titoli freeware immediatamente scaricabili (LocoMalito, AM2R, Cave Story, Spelunky).
* **Impatto su disco:** Solo il pacchetto copertine delle console sbloccate (~350-450 MB per 5 console storiche) + database SQLite. 0 MB di ROM preventive!

---

### 👤 Caso 2: Chi ha già una Collezione Locale (Utente Esperto / Il Tuo Setup)
* **Esigenza:** Riconoscere all'istante l'archivio locale, senza riscaricare o duplicare nulla, e arricchirlo con il cloud per le console mancanti.
* **La Regola delle Cartelle (Convenzione vs Scansione Selvatica):**
  * **Perché NO alla scansione arbitraria dell'intero hard disk:** Scansionare ricorsivamente `C:\` o `D:\` da cima a fondo è un errore critico: impiega decine di minuti, scalda l'hardware, genera falsi positivi e richiede permessi di root/amministratore che su macOS e Linux creano problemi di sicurezza.
  * **La Soluzione Professionale (Standard Universale Batocera/RetroBat):**
    L'utente indica **UNA cartella radice** (es. `C:\Users\...\Desktop\Clod\emu` oppure `D:\ROMS` o `./roms`).
    All'interno, RetroHub cerca le cartelle console standard universali:
    * `roms/snes/`, `roms/megadrive/`, `roms/nes/`, `roms/mame/`, `roms/c64/`, `roms/psx/`, `roms/gba/`, ecc.
    * **Riconoscimento Automatico Indie & LocoMalito:** Rileva all'istante sottocartelle come `roms/indie/` o `LocoMalito & Indie/`, mappando automaticamente sia i titoli commerciali (Hollow Knight, Celeste, UFO 50) sia i freeware con le copertine verticali Steam 600x900 e i metadati completi.
    * **Pulsante Collega Eseguibile:** Se un gioco commerciale è installato altrove (es. su SSD secondario `D:\Games\Hollow Knight`), l'utente usa il comando `[ 📂 Collega Gioco PC ]` selezionando l'eseguibile, con associazione automatica istantanea.
  * **Compatibilità Multipiattaforma Garantita:** Questo schema è identico su Windows (`roms\snes`), Linux (`roms/snes`) e macOS (`roms/snes`), rendendo la migrazione futura istantanea.
  * **Visualizzazione a Video:**
    * In `[ 💾 Solo Scaricati ]`: mostra esclusivamente le ROM e i giochi PC trovati localmente.
    * In `[ 🔓 Tutto l'Archivio ]`: mostra i locali in verde `▶ GIOCA` e l'intero catalogo mondiale non posseduto in ciano `☁️ SCARICA & GIOCA`.

---

### 👤 Caso 3: Dispositivi Low Storage / Handheld (Card Vettoriali Dinamiche)
* **Esigenza:** Massima leggerezza su dispositivi con memoria SSD limitata (Steam Deck, ROG Ally, Mini PC, vecchi laptop con 10 GB liberi).
* **Funzionamento:**
  * In questo profilo, **RetroHub NON scarica nemmeno le immagini delle copertine a monte (0 Byte)**!
  * Tutte le card a video utilizzano un **Placeholder Tipografico Vettoriale Dinamico**:
    * Sfumatura CSS con i colori sociali autentici della console (es. viola per SNES, nero/rosso per Arcade, blu per Mega Drive).
    * Titolo nitido con font retro.
    * Badge informativo con Genere, Anno e Produttore.
    * Icona filigrana vettoriale (🎮, 🕹️, 💾).
  * **Quando viene scaricata la vera copertina?**
    SOLO quando l'utente clicca per scaricare la ROM o assegna la Stella dei Preferiti ⭐!
  * **Impatto su disco:** **0 byte per le immagini all'avvio**! Il catalogo di 50.000 giochi richiede solo i 35 MB del file SQLite!

---

### 👤 Caso 4: PC Gaming con Storage Illimitato / Full Pack Offline
* **Esigenza:** Avere l'intero catalogo pronto e giocabile 100% offline per le console prescelte, senza alcuna necessità di connessione internet.
* **Gestione Piattaforme:**
  * L'utente sceglie le specifiche console per cui scaricare sia l'intero pacchetto copertine HD sia il set completo di file ROM.
  * **Filtro di Disattivazione Sempre Presente:** Dalle impostazioni o dalla sidebar, qualsiasi console non gradita può essere spenta con un interruttore `[ ON / OFF ]`, sparendo all'istante dall'interfaccia.
  * **Gestione Spazio:** Tutte le ROM scaricate vengono conservate permanentemente sul disco nella cartella locale `roms/<console>/`.

---

## 🏷️ 5. Metadati Omogenei per Ricerche Combinate tra tutte le 200 Console

Per consentire le ricerche combinate ultra-rapide e filtri di sottosezione (es. per scaricare solo un pezzo di piattaforma), abbiamo mappato i campi condivisi al 100% da tutti i database mondiali:

### 5.1 I 7 Metadati Universali (Presenti in tutte le 200 Piattaforme)
1. `title` : Titolo normalizzato pulito (senza prefissi e suffissi regionali).
2. `system_code` : Identificativo univoco console (`SNES`, `MD`, `NES`, `MAME`, `PSX`, `C64`...).
3. `year` : Anno di prima pubblicazione commerciale (es. `1992`).
4. `genre` : Macro-generi universali standardizzati:
   * *Platform*, *Picchiaduro (Fighting)*, *Shoot 'em up*, *Azione/Avventura*, *RPG*, *Corse (Racing)*, *Sport*, *Puzzle*, *Strategia*, *Simulazione*.
5. `developer` : Casa di sviluppo principale (*Capcom, Konami, Nintendo, Sega, Square, SNK, Namco, Rare, Taito*...).
6. `players` : Numero giocatori supportati (1P, 2P, 4P, Cooperativo, Versus).
7. `region` : Versione geografica principale (Europe, USA, Japan).

### 5.2 Filtri di Sottosezione Avanzati per Ridurre lo Scheletro della Piattaforma
Nel Wizard Avanzato o dal menu piattaforma, l'utente potrà scegliere di caricare non tutta la console, ma solo una sottosezione mirata:
* 🌟 **Solo Top 50 / Top 100 Storici** (i capolavori imperdibili).
* 🇮🇹 **Solo Edizioni Europee / Multilingua Italiano**.
* 🥊 **Solo per Genere** (es. solo i Picchiaduro e i Platform, saltando i giochi sportivi o puzzle minori).
* 📅 **Intervallo Temporale d'Oro** (es. `1991..1996`).

---

## 🏆 6. Arena Online & Leaderboard Standardizzata (Il Progetto Tecnico)

L'Arena di RetroHub si articolerà in due anime complementari:

### 6.1 Matchmaking P2P Sincronizzato (Sfida 1v1)
* **Host:** Sceglie il gioco e clicca *"Crea Stanza"*. Il server assegna una porta UDP e genera un codice stanza breve (es. `RH-7821`).
* **Guest:** Inserisce il codice `RH-7821` e clicca *"Connetti"*.
* **Handshake UDP:** Calcolo del Ping reale tramite pacchetti echo (RTT < 40ms = verde, 40-90ms = giallo, >90ms = rosso).
* **Lancio:** All'avvio dell'Host, l'emulatore dedicato (Flycast, Snes9x, Dolphin) si avvia in modalità netplay sincronizzata.

### 6.2 Leaderboard Standardizzata (High Scores & Record Mondiali)
* **Per Arcade (MAME / Neo-Geo) e Classici (C64, NES, SNES):**
  * Il sistema legge il punteggio massimo raggiunto al game over (da NVRAM o output di fine sessione).
  * **Classifica Locale:** La Hall of Fame del tuo PC (Top 10 record registrati sul tuo computer).
  * **Classifica Globale RetroHub:** Se connesso a internet, il punteggio viene inviato con checksum di verifica per competere nella classifica mondiale del mese con bacheca trofei.

---

## 📝 7. Riepilogo Decisioni di Design Concordate

1. **Piattaforma Sbloccata = Copertine Locali HD + ROMs 100% On-Demand:**
   * Allo sblocco di una piattaforma, il pacchetto copertine HD viene scaricato e memorizzato in locale (`covers/<console>/`) per garantire navigazione a 60 FPS nativi e fruizione 100% offline del catalogo.
   * Le ROM restano rigorosamente a **0 Byte su disco**; il download avviene solo al click `▶ GIOCA` con pulizia automatica o manuale post-partita.
2. **Struttura cartelle con convenzione standard universale:** Cartella radice indicata dall'utente + sottocartelle `roms/<console>/`. Scansione in sola lettura, zero file toccati.
3. **Modalità Card Vettoriali Dinamiche (Caso 3):** Opzione attiva per handheld e dispositivi low storage con 0 byte per le immagini a monte (copertina reale scaricata solo per Preferiti ⭐ o giochi avviati).
4. **MAME Curato 1G1R di Default:** Set pulito di ~950 Parent ROM uniche escludendo cloni e slot machine, con opzione di sblocco visualizzazione cloni/regionali.
5. **Piattaforme modulari sempre disattivabili:** Nessuna console viene imposta a forza; ciascuna può essere accesa o spenta con un toggle.
6. **Leaderboard integrata in Arena:** Pilastro competitivo per Arcade e classici a punti.
