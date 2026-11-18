# 🕹️ RetroHub — Manuale Utente & Documentazione di Progetto

**Versione:** 1.2.0 (Rust Edition)    
**Autore:** Claudio Dall'Ara  
**Repository GitHub:** [https://github.com/boobaGreen/launcher_game](https://github.com/boobaGreen/launcher_game)  
**Piattaforma:** Windows 10 / 11 (x86_64)

---

## 📖 Indice dei Contenuti
1. [Introduzione & Panoramica](#1-introduzione--panoramica)
2. [Requisiti & Installazione](#2-requisiti--installazione)
3. [Guida all'Uso dell'Interfaccia Grafica](#3-guida-alluso-dellinterfaccia-grafica)
4. [Motore di Ricerca Fuzzy & Filtri Multi-Tag](#4-motore-di-ricerca-fuzzy--filtri-multi-tag)
5. [Configurazione Controller (Xbox & Adattatore USB PS1)](#5-configurazione-controller-xbox--adattatore-usb-ps1)
6. [Guida al Gioco Online Multiplayer (P2P Netplay)](#6-guida-al-gioco-online-multiplayer-p2p-netplay)
7. [Personalizzazione Libreria & Download On-Demand](#7-personalizzazione-libreria--download-on-demand)
8. [Architettura Tecnica del Codice Rust](#8-architettura-tecnica-del-codice-rust)
9. [Risoluzione Problemi & Domande Frequenti (FAQ)](#9-risoluzione-problemi--domande-frequenti-faq)

---

## 1. Introduzione & Panoramica

**RetroHub** è una centrale di comando desktop ad altissime prestazioni scritta interamente in **Rust**, ideata per gestire, sfogliare, catalogare e lanciare all'istante l'intera collezione retrogaming unificata (**1.149 titoli** distribuiti su 16 sistemi differenti).

### I Punti di Forza:
- **Binario Unico Ultra-Compatto**: Tutto il sistema (database SQLite, motore di ricerca fuzzy, micro-server HTTP locale, interfaccia grafica embedded e netcode multiplayer) è racchiuso in un unico file `.exe` di soli **3,5 MB**.
- **Avvio Istantaneo (< 50 millisecondi)**: Nessun tempo di caricamento, consumo di CPU pari allo 0% in idle e memoria RAM inferiore a 25 MB.
- **Esperienza Console da Salotto**: Finestra nativa senza barre del browser (frameless), completamente navigabile tramite controller da divano.
- **Copertine & Boxart Automatiche**: Integrazione diretta con il CDN ufficiale Libretro per visualizzare poster 2D/3D in alta definizione con fallback grafico stilizzato.

---

## 7. Personalizzazione Libreria & Download On-Demand

### 7.1 Selezione Piattaforme all'Inizio
All'avvio iniziale, RetroHub mostra una schermata di benvenuto con l'elenco completo delle piattaforme supportate (16+). Puoi selezionare solo le piattaforme che ti interessano.

### 7.2 Visualizza Tutti i Giochi
- Il catalogo centrale contiene le informazioni di tutti i 1.149 titoli (copertine, tags, metadata).
- Puoi navigare l'intera libreria o filtrarla per le piattaforme selezionate.
- Usa il pulsante "Mostra Tutte" per visualizzare tutti i titoli.

### 7.3 QWizard Auto-Rilevamento ROM
RetroHub può individuare automaticamente le ROM esistenti sul tuo PC:
- Scansione delle cartelle comuni (C:\EMULATORS, D:\ROMS, E:\Giochi)
- Mappatura intelligente per estensione e nome file
- Spostamento automatico nella struttura corretta

### 7.4 Download On-Demand per Immagini
- **Copertine automatiche**: CDN Libretro per boxart 2D/3D
- **Fallback grafico**: Poster stilizzati quando le copertine non sono disponibili
- **Cache locale**: Immagini salvate per accesso offline

### 7.5 Download On-Demand per ROM
Quando clicchi "Gioca" su un gioco senza ROM:
1. Viene mostrato un link sicuro per il download
2. OPPURE scarica automaticamente da fonti legittime
3. Verifica SHA1 per integrità file

### 7.6 Prompt Post-Gioco
Dopo aver giocato, appare una finestra:
```
Vuoi mantenere la ROM?
[TENERE LA ROM]    [CANCELLARE]
```

### 7.7 Gestione Libreria Utente
- **Pagina "La Mia Libreria"**: Filtra per ROM presenti e preferiti
- **Pulsante "Aggiorna"**: Ricerca nuove ROM nelle cartelle
- **Statistiche**: Numero giochi giocabili, spazio totale occupato

---

## 2. Requisiti & Installazione

### Requisiti di Sistema:
- **Sistema Operativo**: Windows 10 o Windows 11 (64-bit).
- **Controller (Opzionale ma consigliato)**: Pad Xbox (One, Series X/S, 360) e/o Adattatore USB Dual Joypad PS1/PS2.
- **Rete**: Connessione Internet necessaria solo per il download delle copertine e per le sfide multiplayer online (il client offline funziona sempre al 100%).

### Installazione & Primo Avvio:
Il progetto è già configurato e compilato nella cartella:  
`C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game\`

Per avviarlo hai due opzioni immediate:
1. **Dal Desktop / Cartella Giochi**:  
   Fai doppio clic sul collegamento [Avvia RetroHub (Client Rust).lnk](file:///C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/Avvia%20RetroHub%20(Client%20Rust).lnk).
2. **Dalla Cartella del Progetto**:  
   Fai doppio clic su `Avvia_RetroHub.bat` oppure direttamente su `retro-hub.exe`.

> [!NOTE]
> Al primissimo avvio, il client sincronizza i 1.149 giochi nel database locale `retro_hub.db` in meno di un secondo. Negli avvii successivi l'apertura è letteralmente istantanea!

---

## 3. Guida all'Uso dell'Interfaccia Grafica

L'interfaccia è progettata per offrire il massimo comfort visivo in ambiente scuro (*Dark Mode Neon*):

```
+---------------------------------------------------------------------------------------------------+
|  🕹️ RetroHub [1.149 Giochi]      [🔍 Cerca titolo, anno, autore...] [✖]    [⭐ Preferiti] [🌐 Arena]  |
+---------------------------------------------------------------------------------------------------+
| BARRA LATERALE          |  HERO BANNER (Gioco Attualmente Selezionato)                            |
| ----------------------- |  [COVER] Street Fighter II (MAME Arcade) | 1991 | Picchiaduro 1v1      |
| [X] Tutte (1.149)       |  🎮 Mappatura: Back: Coin | Start: 1P | A/B/X/Y/LB/RB: Pugni/Calci       |
| [ ] MAME Arcade (596)   |  [ ▶ GIOCA ORA ]   [ 🌐 Sfida Online ]                                   |
| [ ] Commodore 64 (224)  |-------------------------------------------------------------------------|
| [ ] MS-DOS (100)        | GRIGLIA A COPERTINE (Poster Boxart 3D con effetto Hover & Profondità)   |
| [ ] ScummVM (64)        | +----------+  +----------+  +----------+  +----------+  +----------+      |
| [ ] Recomp Native (45)  | | [MAME]   |  | [C64]    |  | [RECOMP] |  | [DOS]    |  | [SNES]   |      |
| [ ] LocoMalito (33)     | | Metal    |  | Turrican |  | Zelda    |  | Doom II  |  | Super    |      |
| [ ] Sega NAOMI (27)     | | Slug     |  | II       |  | OoT      |  |          |  | Mario W. |      |
| [ ] Super Nintendo (21) | +----------+  +----------+  +----------+  +----------+  +----------+      |
+---------------------------------------------------------------------------------------------------+
```

### Componenti Principali:
1. **Top Bar**:
   - Badge con il conteggio in tempo reale dei titoli visualizzati.
   - Casella di ricerca istantanea con pulsante rapido `[✖]` di cancellazione.
   - Pulsante **`⭐ Preferiti`**: visualizza esclusivamente i giochi che hai contrassegnato con la stella.
   - Pulsante **`🌐 Arena Online`**: apre la finestra delle sfide multiplayer P2P.
   - Spia di stato del controller: mostra i joypad attualmente connessi.
2. **Barra Laterale (Filtri Rapidi)**:
   - **Piattaforme**: Filtra all'istante per uno dei 16 sistemi supportati.
   - **Decadi Temporali**: Isola i giochi usciti negli anni '80, '90, 2000, 2010 o Moderni (2020-2026).
3. **Hero Banner (In Alto)**:
   - Mostra in grande la copertina del gioco selezionato, l'anno, il genere, lo sviluppatore e la configurazione comandi specifica per controller.
   - Pulsante verde **`▶ GIOCA ORA`** per avviare la partita.
   - Pulsante azzurro **`🌐 Sfida Online`** per avviare la modalità stanza multigiocatore.
4. **Griglia Copertine**:
   - Schede con proporzioni boxart (3:4) e badge console colorato in alto a sinistra.
   - Clic su una scheda: seleziona il gioco e aggiorna il banner.
   - Doppio clic su una scheda: lancia subito il gioco!
   - Clic sulla stella `⭐`: aggiunge o rimuove il titolo dai tuoi Preferiti.

---

## 4. Motore di Ricerca Fuzzy & Filtri Multi-Tag

A differenza dei semplici filtri statici, RetroHub integra un motore di ricerca a punteggio pesato (*Weighted Fuzzy Engine*) basato sull'algoritmo di **Jaro-Winkler** e sulla distanza di **Levenshtein**.

### Caratteristiche della Ricerca:
- **Tolleranza agli Errori di Battitura**:  
  Se scrivi `"stret fiter"`, il sistema riconosce al 95% di confidenza *"Street Fighter II"*.  
  Se scrivi `"zellda"`, individua immediatamente *"The Legend of Zelda"*.  
  Se scrivi `"pacman"`, trova *"Pac-Man"*.
- **Ricerca Parziale per Parola**:  
  Cercando `"mario"`, troverai tutti i titoli Mario per SNES, N64, Recomp e Arcade.  
  Cercando `"shmup"` o `"sparatutto"`, isolerai l'intera categoria degli sparatutto a scorrimento.
- **Intersezione Booleana dei Filtri**:  
  Tutti i filtri si combinano tra loro. Ad esempio:
  - Seleziona **MAME Arcade** nella barra laterale.
  - Clicca sulla decade **Anni 90**.
  - Digita `"capcom"` nella ricerca.  
  -> Il client mostrerà unicamente i capolavori arcade Capcom degli anni '90!

---

## 5. Configurazione Controller (Xbox & Adattatore USB PS1)

Il client supporta in modo nativo qualsiasi controller per PC, con calibrazione specifica per due categorie principali:

### 🎮 Controller Xbox (XInput):
| Tasto Controller | Azione nel Client RetroHub |
| :--- | :--- |
| **D-Pad / Levetta Sinistra** | Naviga fluidamente tra le copertine della griglia |
| **Tasto A** | Avvia immediatamente il gioco selezionato |
| **Tasto B** | Chiude le finestre modali o torna indietro |
| **Tasto X** | Apre la scheda dettagliata e i comandi del gioco |
| **Tasto Y** | Aggiunge / Rimuove il gioco dai Preferiti ⭐ |
| **Dorsali LB / RB** | Salta da una console all'altra nella barra laterale |

---

### 🕹️ Adattatore USB Dual Joypad PS1/PS2 (DirectInput):
Se colleghi l'adattatore USB a due porte per controller PlayStation 1 o 2:
- **Funzionamento Automatico**: Riconosciuto al volo senza installare driver.
- **Tasto Croce ($\times$)**: Conferma / Avvia Gioco (`A`).
- **Tasto Cerchio ($\bigcirc$)**: Annulla / Chiudi Dettagli (`B`).
- **Tasto Quadrato ($\square$)**: Scheda Informativa & Comandi (`X`).
- **Tasto Triangolo ($\triangle$)**: Aggiunge ai Preferiti ⭐ (`Y`).
- **Dorsali L1 / R1**: Cambio rapido console (`LB/RB`).
- **Frecce Direzionali (D-Pad)**: Supportate sia in modalità analogica che digitale.

### 👥 Uso Simultaneo Misto (Xbox + PS1 Insieme):
Puoi collegare contemporaneamente sia il pad Xbox che l'adattatore PS1:
- Nel client, **qualunque pad tocchi risponde immediatamente**.
- Nei giochi a due giocatori (come *Street Fighter*, *Tekken*, *Metal Slug*, *Windjammers*), un giocatore può usare il pad PS1 e l'altro il controller Xbox!

---

## 6. Guida al Gioco Online Multiplayer (P2P Netplay)

La modalità **Online Arena** consente di giocare online con amici su qualsiasi emulatore senza server intermedi a pagamento e senza configurare porte sul router.

```
       [ Tuo PC (Host) ]                             [ PC del tuo Amico (Guest) ]
              │                                                   │
    1. Clicca "Sfida Online"                                      │
    2. Copia Token Invito ─────────────── Invia Token ───────────>│
              │                                           3. Incolla Token e
              │<────────── Handshake UDP Diretto P2P ──── 4. Clicca "Collegati"
              │                                                   │
  5. Clicca "AVVIA PARTITA"                                       │
              │                                                   │
    [ ▶ Gioco Aperto (P1) ]                        [ ▶ Gioco Aperto Sincronizzato (P2) ]
```

### Procedura per Ospitare (Host):
1. Seleziona il gioco con cui vuoi sfidare il tuo amico.
2. Clicca sul pulsante **`🌐 Sfida Online`** in alto.
3. Inserisci il tuo nome e clicca **`⚡ Genera Stanza Online`**.
4. Clicca su **`📋 Copia Token Invito`** e invia la stringa al tuo amico (tramite chat, WhatsApp, Telegram, Discord, ecc.).
5. Quando l'amico si connette, la spia diventa verde con il Ping stimato (es. *16ms*).
6. Clicca **`▶ AVVIA PARTITA ONLINE`**: il gioco si aprirà in sincrono su entrambi i computer!

### Procedura per Partecipare (Guest):
1. Apri RetroHub sul tuo computer.
2. Clicca su **`🌐 Arena Online`** in alto a destra.
3. Vai nella scheda **`Partecipa a Stanza (Guest)`**.
4. Inserisci il tuo nickname e incolla il Token ricevuto.
5. Clicca **`🔗 Collegati alla Stanza`**: non appena l'Host dà il via, la partita si avvierà automaticamente sul tuo schermo!

---

## 7. Architettura Tecnica del Codice Rust

Il codice sorgente si trova in:  
`C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game\src\`

```
src/
├── main.rs         # Inizializzazione, gestione flag di avvio e apertura finestra desktop
├── models.rs       # Modelli dati strongly-typed (Game, Platform, SearchFilter, Stats)
├── db.rs           # Layer database SQLite locale (CRUD giochi, play count, preferiti)
├── catalog.rs      # Ingestion del Master Catalog con indicizzazione directory e cache
├── search.rs       # Algoritmo di ricerca Fuzzy (Jaro-Winkler) e ordinamento pesato
├── launcher.rs     # Gestore esecuzione nativa Windows (.lnk, .bat, mame.exe, .exe)
├── server.rs       # Micro-server REST HTTP ad alte prestazioni su tiny_http
├── netplay.rs      # Modulo P2P Netplay: gestione stanze, token esadecimali e handshake UDP
└── ui.html         # Interfaccia grafica completa embedded a compile-time (Zero dipendenze)
```

### Come Ricompilare il Progetto:
Se modifichi il codice sorgente o l'interfaccia HTML:
```powershell
cd "C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game"

# Per eseguire i test unitari automatici:
cargo test

# Per compilare la versione Release ultra-ottimizzata:
cargo build --release
```
L'eseguibile ottimizzato viene generato in `target\release\retro-hub.exe`.

---

## 8. Risoluzione Problemi & Domande Frequenti (FAQ)

### D: Il gioco non si avvia quando clicco su "Gioca"?
**R:** Verifica che la cartella `C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\` non sia stata spostata o rinominata. RetroHub cerca i collegamenti ufficiali `.lnk` e i file batch all'interno delle rispettive cartelle di sistema.

### D: Sul controller PS1 non funzionano le frecce direzionali?
**R:** Premi il pulsante centrale **`ANALOG`** sul joypad per accendere la spia rossa. Molti adattatori USB richiedono la modalità analogica attiva per mappare correttamente gli assi della croce direzionale.

### D: Come forzare l'aggiornamento dei giochi se ne aggiungo di nuovi?
**R:** Puoi avviare l'eseguibile da terminale passando il comando `--refresh`:
```powershell
.\retro-hub.exe --refresh
```
Il database locale rileggerà all'istante l'intero catalogo HTML aggiornato.

### D: La finestra si apre ma non vedo le copertine?
**R:** Le copertine vengono scaricate on-demand dal CDN aperto di Libretro. Assicurati che il PC sia connesso a Internet durante la prima consultazione. In assenza di connessione, RetroHub mostrerà un'elegante cartuccia arcade stilizzata con il nome del gioco e della console.
