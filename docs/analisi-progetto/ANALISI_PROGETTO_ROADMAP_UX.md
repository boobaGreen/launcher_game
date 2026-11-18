# 🎮 RetroHub — Report di Analisi Strategica di Progetto
**Ruolo:** Project Manager & Lead Architect  
**Data Analisi:** 18 Novembre 2026  
**Target Progetto:** `C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game`  
**Versione Software Riferimento:** 1.2.0 (Rust Edition)

---

## 📌 1. Executive Summary & Macro Valutazione

**RetroHub** si presenta concettualmente come un launcher desktop ad altissime prestazioni scritto in **Rust**, orientato all'esperienza "da salotto" (10-foot UI / Gamepad-first), con un catalogo unificato di oltre **1.149 titoli** su **16 piattaforme** emulative e native PC.

### Sintesi dello Stato dell'Arte:
* 🟢 **Core Engine & Performance Locale:** L'architettura a singolo binario (`retro-hub.exe`), l'incorporamento del micro-server HTTP locale (`tiny_http`), l'uso di SQLite (`rusqlite`) con indici e l'algoritmo di ricerca fuzzy combinata (**Jaro-Winkler + Levenshtein + token booleani NOT/OR**) rappresentano un'eccellenza ingegneristica per velocità (<50ms di avvio, footprint RAM ridotto).
* 🟡 **Design & UX/UI:** L'interfaccia embedded ha un forte impatto visivo (dark mode stile Steam Deck/RecompLauncher, Hero Carousel, glow ciano/teal, supporto controller), ma è strutturata come un monolite non scalabile (~4.000 righe di HTML/CSS/JS in un unico file) con carenze critiche di rendering (mancanza di virtualizzazione per oltre 1.100 card) e scarsa gestione degli errori visibili all'utente.
* 🔴 **Debito Tecnico & Portabilità:** Il progetto soffre di **hardcoding estremo** di percorsi assoluti specifici della macchina di sviluppo (`C:\Users\ClaudioDall'Ara\...`), rendendolo attualmente non distribuibile o portabile senza interventi manuali nel codice sorgente.
* 🔴 **Discrepanza tra Roadmap/Documentazione e Codice:** C'è un disallineamento sostanziale tra quanto dichiarato nella documentazione (MANUAL.md e ROADMAP.md) e ciò che è realmente implementato a livello funzionale: intere feature (es. Netplay cloud, lobby pubblica, auto-download ROM e CDN mirror) sono parzialmente simulate o rispondono con dati mockati.

---

## 📋 2. Analisi della Documentazione & Coerenza

### 2.1 Punti di Forza della Documentazione (🟢)
* **Manuale Utente Esaustivo (`docs/MANUAL.md`):** Il manuale è redatto con chiarezza espositiva, diagrammi logici in ASCII art, tabelle di mappatura pulsanti (Xbox e PS1/PS2 via adattatore USB) e guide passo-passo ben congegnate per l'utente finale.
* **README Coinvolgente (`README.md`):** Panoramica sintetica ed efficace con elenco esaustivo delle 16 piattaforme supportate e metriche di build chiare.

### 2.2 Punti Deboli & Incongruenze Critiche (🔴)
1. **"Paperware" vs Stato Reale (Discrepanze Funzionali):**
   * *Multiplayer Netplay / Arena:* Il manuale descrive un'infrastruttura di handshake UDP e ping RTT stimato con server STUN. Nel codice sorgente (`src/netplay.rs` e `src/server.rs`), il ping è un valore statico o fittizio (`ping = r.player_name.len() as u64 * 10` o hardcoded `Some(20)`), e l'IP pubblico non viene risolto tramite server STUN ma ricade sempre su interfacce locali (`127.0.0.1` o IP LAN).
   * *Download On-Demand e Gestione ROM (Fase 4):* Nel manuale e nella roadmap la Fase 4 risulta ampiamente spuntata o descritta come operativa. In realtà, gli endpoint di `server.rs` (`/api/library/config`, `/api/library/scan`, `/api/library/download`, `/api/library/delete`) restituiscono stringhe JSON cablate staticamente (`{"success": true, "found": 0}`), senza interazione reale con il filesystem o la rete.
2. **Hardcoding di Ambienti Personali nella Documentazione:**
   * La documentazione fa continuo riferimento a percorsi assoluti personali (`C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\...`) anziché a percorsi relativi o variabili d'ambiente (`%APPDATA%`, `./roms`, `./emulators`), trasmettendo la sensazione di un progetto privato non ancora standardizzato per release esterne.
3. **Disallineamento delle Versioni:**
   * `Cargo.toml` indica versione `1.2.0`, `reports/REVISIONE_TECNICA.md` indica `1.0.0`, mentre in varie sezioni storiche si fa riferimento a parametri di versionamento non sincronizzati.

---

## 🗺️ 3. Analisi della Roadmap & Stato Reale di Avanzamento

| Fase di Roadmap | Stato Dichiarato | Stato Reale nel Codice | Valutazione del Project Manager |
| :--- | :---: | :---: | :--- |
| **FASE 1: Client Locale & Esperienza Utente** | **100% Completato** | **90% Funzionante** | 🟢 **Core solido.** Backend Rust veloce, ricerca fuzzy eccellente, persistenza SQLite solida. Manca la separazione pulita delle configurazioni. |
| **FASE 2: Multiplayer P2P Diretto** | **100% Completato** | **45% Parziale** | 🟡 **Funzionante solo in LAN.** La generazione del codice stanza esiste, ma la comunicazione attraversa solo localhost/LAN. Manca vero NAT Traversal / STUN. |
| **FASE 3: Cloud Lobby & Matchmaking** | **100% Completato** | **25% Simulato** | 🔴 **Mock locale.** Non esiste un server di segnalazione cloud o broker pubblico; le stanze sono gestite solo in una struttura dati in memoria dell'istanza locale. |
| **FASE 4: Download On-Demand & Library Management** | **In Corso (molti task [x])** | **10% Embrionale** | 🔴 **API Stub.** `downloader.rs` e `rom_scanner.rs` sono moduli slegati dal flusso principale; le chiamate HTTP relative rispondono con mock statici. |

### Verdetto Strategico sulla Roadmap:
La roadmap ha subito un tipico fenomeno di **feature creep** (introduzione precoce di multiplayer P2P, cloud lobby e CDN downloader) prima di aver consolidato i fondamenti essenziali del software (portabilità dei percorsi, file di configurazione utente, separazione del frontend, gestione errori).

---

## 🎨 4. Deep-Dive: Design, UX/UI & Usabilità

L'interfaccia utente è il punto di contatto fondamentale tra il giocatore e la collezione. RetroHub punta a un'esperienza da console (*10-Foot User Interface*), ma presenta luci e ombre significative.

### 4.1 Design System & Impatto Visivo (🟢)
* **Atmosfera Visiva Moderna:** La palette colori scura (`--bg-base: #0b0f17`, `--bg-surface: #131926`) con accenti neon teal/ciano (`--accent: #00d2b4`) e oro per i comandi conferisce un look premium e cinematografico.
* **Hero Banner Dinamico:** La sezione superiore con copertina in evidenza, tag genere/anno, dettagli di controllo gamepad e pulsante *"GIOCA ORA"* è ben strutturata visivamente e richiama i layout di PlayStation 5 e Steam Big Picture.
* **Card Cover & Fallback:** Rapporto proporzionale poster standardizzato (~175x270px) con fallback tipografico elegante ("GTI Club 2 style") quando la boxart esterna non è reperibile.

### 4.2 Criticità Architetturali della UI (🔴)
1. **Monolite `ui.html` da 152 KB (~4.000 righe):**
   * CSS, logica di templating, gestione eventi Gamepad, chiamate fetch e manipolazione DOM risiedono interamente in un solo file.
   * Modificare o correggere un bug visivo comporta rischi elevatissimi di regressione.
   * Assenza di un build system frontend (es. Vite/Tailwind o semplice modularizzazione ES Modules).
2. **Performance di Rendering & Assenza di Virtualizzazione:**
   * La UI crea ed inserisce nel DOM fino a **1.149 card contemporaneamente**.
   * Su monitor ad alto refresh rate o GPU integrate, lo scrolling della libreria genera cali di frame e rallentamenti percepibili (*jank* da DOM reflow).
   * **Soluzione necessaria:** Implementare *virtual scrolling* (renderizzare solo le 20-30 card visibili a schermo).

### 4.3 Punti Deboli UX & Interazione Utente (🟡/🔴)
1. **Feedback Silente sugli Errori (Zero Error Dialogs):**
   * Se un emulatore non è installato, o se un collegamento batch fallisce, l'utente clicca *"Gioca"* e non riceve alcun feedback visivo a schermo (la notifica fallisce silenziosamente o viene scritta solo nei log della console di backend).
   * **Raccomandazione UX:** Introdurre un sistema globale di **Toast Notifications** o banner modale che informi chiaramente l'utente sul motivo del mancato avvio (es. *"ROM non trovata in percorso X"*, *"Emulatore Y mancante"*).
2. **Assenza di una Schermata "Impostazioni / Opzioni" in-App:**
   * L'utente non ha modo dall'interfaccia di configurare le cartelle delle ROM, rimappare i comandi del pad, selezionare la risoluzione o abilitare/disabilitare emulatori.
3. **Esperienza Gamepad Parziale nei Modali:**
   * Mentre la navigazione della griglia principale tramite D-pad / stick analogico funziona adeguatamente, diverse schermate modali (Arena Online, filtri avanzati, finestre di dialogo) richiedono l'uso del mouse, spezzando l'immersione "da divano".
4. **Heartbeat Server e Rischio Disconnessione (15 Secondi):**
   * Il backend chiude il server se non riceve ping per 15 secondi. Se l'utente minimizza il browser, riceve una chiamata o l'interfaccia web entra in sospensione energetica su laptop, il launcher si arresta improvvisamente in background.

---

## ⚠️ 5. Macro Punti Deboli & Debiti Tecnici Critici

### 🔴 1. Hardcoding Strutturale dei Percorsi
* **Gravità:** Bloccante per qualsiasi distribuzione.
* **Descrizione:** File come `main.rs`, `server.rs`, `launcher.rs`, `catalog.rs` e i plugin puntano direttamente a `C:\Users\ClaudioDall'Ara\...`. Se la cartella viene spostata, o eseguita su un altro PC, il launcher fallisce o si apre a catalogo vuoto.
* **Azione correttiva:** Centralizzare tutti i percorsi in `retrohub.json` con risoluzione dinamica dei percorsi relativi (`./emu`, `./covers`, `./data`).

### 🔴 2. Fragilità del Catalogo Dati (`catalog.rs`)
* **Gravità:** Alto debito di manutenibilità.
* **Descrizione:** Il file `catalog.rs` supera i 75 KB e contiene centinaia di righe di logica condizionale cablata (`if t_lower.contains("secret of monkey island") ...`).
* **Azione correttiva:** Estrarre tutte le associazioni e i metadati in file dati esterni (JSON o tabella SQLite dedicata `game_metadata`). Il codice Rust deve contenere solo la logica di parsing, non i dati.

### 🔴 3. Lancio Giochi tramite PowerShell Escaping
* **Gravità:** Instabilità di lancio.
* **Descrizione:** In `launcher.rs`, l'avvio sfrutta uno script PowerShell iniettato con concatenazione di stringhe:
  ```rust
  let script = format!(r#"$p = Start-Process -FilePath '{}' -PassThru; ... (New-Object -ComObject WScript.Shell).AppActivate($p.Id)"#, escaped);
  ```
  Se il titolo del gioco o il percorso contiene virgolette particolari o caratteri non standard, il lancio fallisce o genera errori di parsing su PowerShell.
* **Azione correttiva:** Utilizzare chiamate dirette tramite `std::process::Command` nativo di Windows con argomenti isolati o API Win32 (`ShellExecuteExW`).

### 🟡 4. Maturità del Plugin System
* **Gravità:** Media.
* **Descrizione:** È stato introdotto il trait `EmulatorPlugin` con implementazioni per Dolphin, DuckStation, Flycast, ecc., ma il sistema è solo parzialmente integrato con il ciclo di lancio principale di `launcher.rs`.

---

## 🎯 6. Piano d'Azione Strategico & Raccomandazioni (Roadmap Correttiva)

Per portare RetroHub da eccellente prototipo locale a prodotto maturo, stabile e distribuibile, si consiglia di strutturare i prossimi interventi in tre macro-fasi prioritarie:

```
[ FASE 1: STABILITÀ & PORTABILITÀ ] ──> [ FASE 2: ECCELLENZA UX/UI ] ──> [ FASE 3: ESPANSIONE FUNZIONALE ]
- Disaccoppiamento percorsi           - Virtual Scrolling (1100+ card)     - Completamento reale ROM Scanner
- Configurazione JSON dinamica         - Separazione CSS/JS da ui.html      - Sincronizzazione CDN verificata
- Gestione Errori & ShellExecute      - Sistema Notifiche Toast            - De-scope o consolidamento Netplay
```

### 🥇 Fase 1: Consolidamento Architetturale & Portabilità (Priorità Massima)
1. **Configurazione Relativa & Auto-rilevamento:**
   * Far caricare a `RetroHubConfig` i percorsi da `./retrohub.json`. In assenza di configurazione, scansionare la directory corrente e richiedere all'utente la directory della libreria al primo avvio.
2. **Estrazione Dati da `catalog.rs`:**
   * Spostare tutti i dizionari di cover, titoli e steam IDs da codice sorgente a file `covers_db.json`.
3. **Rafforzamento Lancio Applicazioni:**
   * Sostituire l'invocazione tramite script PowerShell con chiamate Windows native prive di problemi di string-escaping.

### 🥈 Fase 2: Raffinamento Design & User Experience (Priorità Alta)
1. **Virtualizzazione della Griglia Giochi:**
   * Integrare un algoritmo di virtual rendering in JavaScript per mantenere nel DOM solo le card visibili a viewport, garantendo 60 FPS costanti nello scorrimento.
2. **Modularizzazione del Frontend:**
   * Suddividere `ui.html` in file logici: `styles/`, `scripts/gamepad.js`, `scripts/api.js`, `scripts/ui.js` e compilarli/incorporarli in fase di build.
3. **Sistema di Feedback Errori & Loading State:**
   * Visualizzare banner/toast non invasivi in basso a destra quando un emulatore non risponde o un file manca.
4. **Schermata Impostazioni Grafica:**
   * Aggiungere una modale accessibile via controller con icona a ingranaggio per personalizzare percorsi, audio, temi e comportamento post-partita.

### 🥉 Fase 3: Trasparenza Funzionale & Pulizia Roadmap (Priorità Media)
1. **Allineamento Documentale della Roadmap:**
   * Riportare la Fase 3 (Lobby Cloud) e la Fase 4 (Download On-Demand) allo stato reale (`[ ]` da implementare) per evitare discrepanze tra promessa documentata e software installato.
2. **Decisione Strategica sul Netplay:**
   * Valutare se il Netplay sia un vero obiettivo di valore per l'utente singolo/da salotto; in caso affermativo, implementare un server di signaling STUN esterno reale, altrimenti focalizzare il 100% delle risorse sull'eccellenza dell'esperienza launcher locale offline.

---
*Report redatto e archiviato in `docs/analisi-progetto/ANALISI_PROGETTO_ROADMAP_UX.md`.*
