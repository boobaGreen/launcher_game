# 🕹️ RetroHub — Manuale Utente & Documentazione Ufficiale

**Versione:** 2.0.0 (Architettura Modulare & On-Demand)  
**Autore:** Claudio Dall'Ara & Team di Sviluppo  
**Repository GitHub:** [https://github.com/boobaGreen/launcher_game](https://github.com/boobaGreen/launcher_game)  
**Piattaforma:** Windows 10 / 11 (x86_64) — *Progettato per Linux e macOS*

---

## 📖 Indice dei Contenuti
1. [Introduzione & Filosofia di RetroHub](#1-introduzione--filosofia-di-retrohub)
2. [Requisiti di Sistema & Installazione Portabile](#2-requisiti-di-sistema--installazione-portabile)
3. [Interfaccia Grafica da Salotto (10-Foot UI)](#3-interfaccia-grafica-da-salotto-10-foot-ui)
4. [Enciclopedia Visiva & Sblocco Immagini (Zero-Disk)](#4-enciclopedia-visiva--sblocco-immagini-zero-disk)
5. [Download ROM "On-The-Fly" & Prompt Salva-Spazio](#5-download-rom-on-the-fly--prompt-salva-spazio)
6. [Motore di Ricerca Fuzzy Avanzato & Filtri Booleani](#6-motore-di-ricerca-fuzzy-avanzato--filtri-booleani)
7. [Configurazione Controller (Xbox, PS1/PS2 & DirectInput)](#7-configurazione-controller-xbox-ps1ps2--directinput)
8. [Architettura Emulatori & Core Libretro](#8-architettura-emulatori--core-libretro)
9. [Guida alle Sfide Multiplayer (Netplay P2P)](#9-guida-alle-sfide-multiplayer-netplay-p2p)
10. [Risoluzione Problemi & Domande Frequenti (FAQ)](#10-risoluzione-problemi--domande-frequenti-faq)

---

## 1. Introduzione & Filosofia di RetroHub

**RetroHub** è una centrale di comando desktop ad altissime prestazioni scritta interamente in **Rust**, ideata per gestire, sfogliare, catalogare e lanciare all'istante l'intera storia dei videogiochi (oltre **1.149 capolavori** su oltre **16 piattaforme** iniziali, espandibili a oltre 200 sistemi).

### I Punti di Forza Unici:
* ⚡ **Avvio Istantaneo (< 50 millisecondi):** Nessuna attesa di caricamento, consumo di CPU pari allo 0% in idle e memoria RAM inferiore a 30 MB.
* 📦 **Singolo Binario Portabile (`retro-hub.exe`):** Nessun installer invasivo, zero chiavi di registro di Windows, cartella 100% portatile trasferibile su chiavetta USB o hard disk esterno.
* 🎨 **Enciclopedia Visiva Sbloccata al 100%:** Puoi esplorare tutte le copertine boxart in alta definizione, gli anni, i generi e le descrizioni storiche anche senza avere le ROM scaricate in anticipo sul PC.
* ☁️ **Download ROM al Volo (On-The-Fly):** Clicchi su un gioco, RetroHub lo scarica in background in pochi secondi, verifica l'integrità del file e avvia subito la partita.
* 🎮 **Esperienza Console da Divano:** Finestra frameless senza bordi del browser, navigabile al 100% tramite gamepad.

---

## 2. Requisiti di Sistema & Installazione Portabile

### Requisiti Minimi:
* **Sistema Operativo:** Windows 10 o Windows 11 (64-bit).
* **Processore:** Qualsiasi CPU Intel o AMD a 64 bit (anche processori a basso consumo).
* **RAM:** 2 GB di memoria (RetroHub ne occupa solo ~25 MB).
* **Controller (Raccomandato):** Gamepad Xbox (Series X/S, One, 360), PlayStation (DualShock/DualSense) o adattatori USB per pad PS1/PS2.
* **Rete:** Connessione Internet necessaria solo per il download delle copertine e per i giochi on-demand (tutto ciò che è già scaricato funziona al 100% offline).

### Come Avviare RetroHub:
1. Scarica o scompatta la cartella `RetroHub/` in qualsiasi punto del tuo computer (es. `C:\Giochi\RetroHub` o su disco esterno `D:\RetroHub`).
2. Fai doppio clic su `retro-hub.exe` (oppure sul file batch di comodo `Avvia_RetroHub.bat`).
3. L'interfaccia comparirà immediatamente sullo schermo in modalità finestra dedicata a schermo intero.

> [!NOTE]
> Al primissimo avvio, RetroHub crea automaticamente il file di configurazione `retrohub.json` e il database locale indicizzato `retro_hub.db` in meno di un secondo.

---

## 3. Interfaccia Grafica da Salotto (10-Foot UI)

L'interfaccia grafica è progettata con tema scuro cyberpunk/neon ad alto contrasto visivo:

```
+---------------------------------------------------------------------------------------------------+
|  🕹️ RetroHub [1.149 Giochi]      [🔍 Cerca titolo, anno, autore...] [✖]    [⭐ Preferiti] [⚙️ Opzioni]|
+---------------------------------------------------------------------------------------------------+
| BARRA LATERALE          |  HERO BANNER (Spotlight Gioco in Evidenza)                              |
| ----------------------- |  [BOXART] The Legend of Zelda: Ocarina of Time | 1998 | Avventura RPG   |
| [X] Tutte le Console    |  🎮 Mappatura: Levetta: Movimento | A: Azione | B: Spada | Start: Menu    |
| [ ] MAME Arcade (596)   |  [ ▶ GIOCA SUBITO ]   [ 🌐 Sfida Online ]   [ ⭐ Aggiungi ai Preferiti ] |
| [ ] Super Nintendo (21) |-------------------------------------------------------------------------|
| [ ] Commodore 64 (224)  | GRIGLIA A COPERTINE (Poster Boxart 3D con effetto Hover & Glow Ciano)   |
| [ ] Sega Dreamcast (14) | +----------+  +----------+  +----------+  +----------+  +----------+      |
| [ ] MS-DOS PC (100)     | | [SNES]   |  | [MAME]   |  | [N64]    |  | [C64]    |  | [PS1]    |      |
| [ ] Sony PS1 (18)       | | Super    |  | Metal    |  | Super    |  | Turrican |  | Tekken   |      |
| [ ] GameCube (12)       | | Mario W. |  | Slug X   |  | Mario 64 |  | II       |  | 3        |      |
| [ ] PC Recomp (45)      | +----------+  +----------+  +----------+  +----------+  +----------+      |
+---------------------------------------------------------------------------------------------------+
```

### Elementi Chiave:
1. **Top Bar di Controllo:**
   * Badge contatore con il numero di titoli visibili in tempo reale.
   * Barra di ricerca fuzzy con tasto rapido di cancellazione `[✖]`.
   * Filtro immediato **`⭐ Preferiti`**.
   * Icona delle **Impostazioni `⚙️`** per percorsi e opzioni salva-spazio.
2. **Barra Laterale Console:**
   * Elenco rapido con contatore giochi per ciascuna console.
   * Cliccando su una piattaforma, la griglia si aggiorna istantaneamente.
3. **Hero Banner Superiore:**
   * Mostra in formato maxi la copertina, i dettagli di gioco (anno, genere, sviluppatore) e la mappatura precisa dei pulsanti per controller.
4. **Griglia Copertine Poster (3:4):**
   * Schede verticali eleganti con glow ciano al passaggio del mouse o cursore del joypad.
   * Se la boxart esterna non è disponibile, viene mostrato un fallback tipografico scuro con il badge colorato della console.

---

## 4. Enciclopedia Visiva & Sblocco Immagini (Zero-Disk)

Una delle più grandi innovazioni di RetroHub è l'**approccio Zero-Disk Footprint**:
* Non sei obbligato a riempire il tuo disco rigido con 500 GB o 1 Terabyte di ROM prima di poter usare l'applicazione.
* Quando selezioni una console (es. *Super Nintendo* o *MAME*), puoi impostare la visualizzazione su:
  * **🔓 Sblocca Tutto l'Archivio (100% Immagini):** Mostra l'intera collezione mondiale di titoli con copertine in alta definizione, schede storiche e controlli.
  * **💾 Solo Scaricati in Locale:** Mostra unicamente i giochi i cui file ROM sono già fisicamente presenti sul tuo PC.

Le immagini visualizzate vengono salvate automaticamente nella cartella `cache/covers/` per essere disponibili anche quando il PC non è connesso a Internet.

---

## 5. Download ROM "On-The-Fly" & Prompt Salva-Spazio

Come funziona l'avvio di un gioco che non hai ancora sul disco:

```
[ Clic su "☁️ SCARICA & GIOCA" ]
              │
              ▼
[ Download al Volo in Background (da CDN/Mirror) con stima tempo ]
              │
              ▼
[ Verifica Integrità File (Checksum SHA1 / CRC32) ]
              │
              ▼
[ Avvio Automatico Immediato del Gioco a Schermo Intero ]
              │
              ▼ (Sessione di gioco terminata)
[ Chiusura Emulatore ]
              │
              ▼
[ Finestra di Dialogo Salva-Spazio ]
  ┌────────────────────────────────────────────────────────────┐
  │ Vuoi conservare la ROM sul PC o eliminarla per fare spazio?│
  │ [ 💾 TIENI SUL PC ]        [ 🗑️ ELIMINA SUBITO ]           │
  └────────────────────────────────────────────────────────────┘
```

Se scegli **ELIMINA**, il file temporaneo viene cancellato dal disco, mantenendo il computer sempre pulito e leggero. Se scegli **TIENI**, il gioco rimane pronto all'uso istantaneo per le volte successive.

---

## 6. Motore di Ricerca Fuzzy Avanzato & Filtri Booleani

RetroHub non usa semplici filtri a corrispondenza esatta, ma integra un motore a punteggio pesato (*Weighted Fuzzy Engine*) basato sugli algoritmi di **Jaro-Winkler** e **Levenshtein**.

### Esempi di Sintassi Avanzata:
* **Tolleranza agli errori:** Se digiti `"stret fiter"` o `"zellda"`, RetroHub riconosce con oltre il 90% di confidenza i titoli corretti.
* **Frasi Esatte con Virgolette (`"..."`):** Cercando `"lupin"`, il sistema isolerà esclusivamente i titoli su Lupin III, escludendo titoli simili come *Lupo Alberto*.
* **Esclusione (`-` o `NOT`):** `mario -kart` trova tutti i giochi di Mario escludendo i giochi di kart.
* **Filtri di Campo:**
  * `title:sonic` (cerca solo nei titoli)
  * `genre:rpg` o `genre:picchiaduro`
  * `dev:capcom` o `dev:nintendo`
  * `sys:snes` o `sys:mame`
* **Intervalli di Anni:** `1990..1995` mostra solo i giochi pubblicati in quel quinquennio.
* **Operatore OR (`|`):** `dev:sega | dev:namco` trova i titoli di entrambe le case.

---

## 7. Configurazione Controller (Xbox, PS1/PS2 & DirectInput)

RetroHub riconosce istantaneamente qualsiasi gamepad compatibile con Windows (XInput o DirectInput) senza dover configurare driver o software di terze parti:

### 🎮 Controller Xbox (XInput):
| Tasto Controller | Funzione in RetroHub |
| :--- | :--- |
| **D-Pad / Levetta Sinistra** | Scorre fluidamente tra le copertine della griglia |
| **Pulsante A** | Avvia il gioco selezionato (o avvia il download on-the-fly) |
| **Pulsante B** | Chiude le finestre di dialogo o torna indietro |
| **Pulsante X** | Apre il cassetto laterale con i dettagli storici e comandi |
| **Pulsante Y** | Aggiunge o rimuove il gioco dai Preferiti ⭐ |
| **Dorsali LB / RB** | Salta rapidamente da una console all'altra nella sidebar |

### 🕹️ Controller PlayStation (con o senza adattatore USB):
* **Tasto Croce ($\times$):** Conferma / Avvia (`A`).
* **Tasto Cerchio ($\bigcirc$):** Torna indietro (`B`).
* **Tasto Quadrato ($\square$):** Scheda Dettagli (`X`).
* **Tasto Triangolo ($\triangle$):** Preferiti ⭐ (`Y`).
* **Dorsali L1 / R1:** Cambio rapido piattaforma (`LB/RB`).

---

## 8. Architettura Emulatori & Core Libretro

RetroHub adotta l'architettura a **Doppio Binario**:

1. **Console Classiche (NES, SNES, Mega Drive, GB, GBA, C64, Atari, PC Engine...):**
   * Vengono eseguite tramite **RetroArch Libretro Core** in modalità headless (completamente invisibile).
   * L'utente non vede mai RetroArch: il gioco si apre a schermo intero e al termine si ritorna all'istante su RetroHub.
   * I singoli file core (`.dll`) vengono scaricati in modo trasparente dal buildbot ufficiale Libretro solo al primo avvio della rispettiva console.
2. **Sistemi Pesanti e Arcade Speciali (Dolphin, DuckStation, Flycast, MAME, PCSX2, Hypseus):**
   * Vengono eseguiti tramite i rispettivi emulatori standalone dedicati, garantendo il 100% della compatibilità hardware e delle prestazioni grafiche avanzate.

---

## 9. Guida alle Sfide Multiplayer (Netplay P2P)

La modalità **Arena Online** consente di giocare online 1 contro 1 con un amico a distanza:

1. **Host (Chi crea la stanza):**
   * Seleziona il gioco desiderato (es. *Street Fighter II*, *Metal Slug*, *Tekken 3*).
   * Clicca su **`🌐 Sfida Online`** nel banner superiore.
   * Clicca su **`⚡ Genera Stanza Online`** e copia il codice univoco generato (es. `RH-7821`).
   * Invia il codice al tuo amico.
2. **Guest (Chi partecipa):**
   * Apre RetroHub sul proprio PC.
   * Clicca su **`🌐 Arena Online`** in alto.
   * Incolla il codice stanza ricevuto e clicca **`🔗 Connettiti`**.
3. Quando entrambi i giocatori sono sincronizzati, l'host clicca **`▶ AVVIA PARTITA ONLINE`**: il gioco si avvia in sincrono perfetto su entrambi i monitor!

---

## 10. Risoluzione Problemi & Domande Frequenti (FAQ)

### D: Perché un gioco non parte quando clicco su "Gioca"?
**R:** Se il gioco richiede un file locale, verifica che la ROM sia presente nella cartella `roms/<piattaforma>/`. Se è un gioco on-demand, assicurati che il computer sia connesso a Internet durante il primo download.

### D: Il controller non risponde nella schermata principale?
**R:** Premi un qualsiasi tasto sul joypad per "svegliare" l'interfaccia Gamepad di Windows. Se usi un adattatore per controller PS1/PS2, assicurati che la spia rossa **ANALOG** sul pad sia accesa.

### D: Posso aggiungere le mie ROM personali che ho già scaricato altrove?
**R:** Certamente! Ti basta copiare i tuoi file `.zip`, `.sfc`, `.chd` o `.iso` all'interno della cartella `roms/<nome_piattaforma>/` e cliccare sul tasto **Aggiorna Libreria** nelle impostazioni. RetroHub le indicizzerà all'istante nel database.

### D: RetroHub funzionerà anche su Linux o su Mac in futuro?
**R:** Sì! Il motore centrale è scritto in Rust ed è progettato fin dall'inizio per essere multipiattaforma. Una volta perfezionata la versione Windows, verranno rilasciati i binari nativi per Linux e macOS (Apple Silicon M1/M2/M3 e Intel).
