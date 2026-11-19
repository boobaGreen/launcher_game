# 🎨 RetroHub — Specifiche di Design System UX/UI & Hero Spotlight Widescreen

Questo documento formalizza l'intera identità visiva, la palette cromatica, i componenti di interfaccia, l'anatomia della **Hero Spotlight**, le card e i filtri di **RetroHub**, progettati secondo i più moderni standard dell'industria videoludica (*Steam Big Picture, PlayStation 5 OS, Apple TV 4K, tvOS e Playnite*).

---

## 💎 1. I 5 Principi Guida del Design System

1. **Cinematic Dark Luxury (OLED Ready):**  
   Fondo nero ossidiana (`#080b10`) per contrasto assoluto, neri infiniti e zero affaticamento visivo nelle sessioni notturne o su schermi TV da salotto.
2. **Accenti Neon Dinamici a Bassa Saturazione:**  
   Ogni console e ogni stato ha una luce perimetrale coerente col brand (viola Nintendo, blu reale Sega, indaco PlayStation, oro Recomp, fucsia/viola Indie).
3. **Gerarchia Tipografica ad Altissima Leggibilità (WCAG AA):**  
   Titoli netti, spaziature generose, metadati in grigio ardesia ad alto contrasto. Nessun testo minuscolo o illeggibile da 3 metri di distanza.
4. **Micro-Interazioni Elastiche e Feedback Tattile a 60 FPS:**  
   Hover con elevazione isometrica morbida (`scale(1.03)`), rotazione 3D della boxart in parallasse, badge fluttuanti e ombre multi-livello.
5. **Gamepad-First & Tastiera/Mouse Unificati:**  
   Ogni elemento possiede un anello di focus (`focus-ring`) nitido e luminoso. HUD a schermo permanente con legenda tasti controller Xbox (`(A) Gioca`, `(B) Indietro`, `(X) Preferito`, `(Y) 🎲 Casuale`).

---

## 🎨 2. Palette Cromatica & Token Visivi Ufficiali

```css
:root {
    /* Superfici e Fondi */
    --bg-void: #080b10;             /* Sfondo generale profondo (OLED) */
    --bg-surface: #0f1520;          /* Fondo base card e pannelli */
    --bg-surface-elevated: #162030; /* Card al passaggio del mouse o modali */
    --bg-glass: rgba(15, 21, 32, 0.75); /* Effetto vetro con blur(24px) */
    
    /* Bordi e Riflessi */
    --border-subtle: rgba(255, 255, 255, 0.08);
    --border-glow: rgba(0, 229, 180, 0.4);
    --border-card-hover: rgba(255, 255, 255, 0.22);

    /* Testo e Tipografia */
    --text-primary: #f8fafc;        /* 98% bianco brillante */
    --text-secondary: #94a3b8;      /* Slate 400 - genere, anno, sviluppatore */
    --text-muted: #64748b;          /* Slate 500 - didascalie, scorciatoie */

    /* Colori di Accento Funzionali */
    --accent-cyan: #00e5b4;         /* Azione primaria, Gioca, Scarica, Online */
    --accent-blue: #38bdf8;         /* Cloud, Filtri attivi, Info */
    --accent-gold: #f59e0b;         /* Preferiti ⭐, Recomp ⚡, Leaderboard 🏆 */
    --accent-purple: #a855f7;       /* PC Indie, Fan Games, Comunità */
    --accent-magenta: #ec4899;      /* Funzione 🎲 Retro-Roulette / Sorprendimi */
    --accent-danger: #f43f5e;       /* Arena Live, Scontro, Elimina */

    /* Colori Sociali Console (Glow Dinamico) */
    --console-snes: #8b5cf6;        /* Viola Super Nintendo */
    --console-megadrive: #0284c7;   /* Blu Elettrico Sega */
    --console-psx: #3b82f6;         /* Indaco Sony PlayStation */
    --console-arcade: #f97316;      /* Arancio Neo-Geo / Capcom */
    --console-n64: #10b981;         /* Verde Smeraldo Nintendo 64 */
    --console-c64: #6366f1;         /* Blu Nostalgia Commodore */
    --console-recomp: #eab308;      /* Oro Nativo PC 60fps */
    --console-indie: #ec4899;       /* Fucsia Neon Neo-Retro */
}
```

---

## 🌟 3. La HERO SPOTLIGHT Widescreen (Il Cuore della UI)

La Hero Banner è la vetrina cinematografica che accoglie l'utente appena apre l'hub.

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│  HERO SPOTLIGHT CINEMATICA (Altezza 420px, Raggio 24px, Vetro & Gradiente Dinamico)   │
├───────────────────────────────────────────────────────┬────────────────────────────────┤
│  [ 🎮 SNES ]  [ ⭐ I TUOI PREFERITI • 2/6 ]           │                                │
│                                                       │       ┌────────────────┐       │
│  Super Mario World                                    │       │                │       │
│  1990 • Nintendo • Platformer • 👥 2 Giocatori        │       │  COPERTINA 3D  │       │
│                                                       │       │   FLUTTUANTE   │       │
│  Il capolavoro assoluto che ha definito l'era a       │       │  IN PARALLASSE │       │
│  16-bit. Esplora 96 uscite segrete e cavalca Yoshi!   │       │ (rotateY -12°) │       │
│                                                       │       │                │       │
│  🎮 Controlli Xbox: A Salto, X Corsa/Sputa, B Giro   │       └────────────────┘       │
│                                                       │         Shadow & Glow          │
│  [ ▶ GIOCA ORA ]   [ ⭐ Salva ]   [ 🎲 Sorprendimi ]  │                                │
├───────────────────────────────────────────────────────┴────────────────────────────────┤
│  [‹] [ ──●──── ] [ ────── ] [ ────── ] [ ────── ] [›]       [ ⚙️ Personalizza Hero ]    │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

### 3.1 Anatomia e Funzionalità della Hero
1. **Sfondo Ambientale Dinamico a 3 Livelli:**
   * **Livello 1 (Cover Blur Backdrop):** La copertina del gioco proiettata a tutto schermo con sfocatura gaussiana a 45px e luminosità attenuata (`brightness 0.25`).
   * **Livello 2 (Glow della Console):** Un alone radiale sfumato con il colore ufficiale della piattaforma (es. se la Hero mostra *Super Mario World*, l'ambiente si tinge di una soffusa luce viola SNES).
   * **Livello 3 (Vignettatura seamless):** Gradiente orizzontale scuro da sinistra verso destra per garantire che il testo sia sempre nitido e leggibile al 100%.
2. **Copertina 3D Fluttuante con Parallax Tilt:**
   * Renderizzata a destra con inclinazione prospettica (`rotateY(-12deg) rotateX(4deg)`).
   * Animazione morbida di levitazione a 60 FPS (`heroCoverFloat 6s ease-in-out infinite`).
   * Bordo in cristallo speculare con riflesso di luce sul lato superiore.
3. **Miniature & Avanzamento Temporale:**
   * In basso a sinistra, una serie di pillole o mini-punti.
   * La pillola del gioco corrente contiene una **barra di riempimento luminoso** (`linear-progress`) che scorre durante i secondi di visualizzazione prima di passare al prossimo gioco!
   * Cliccabile con mouse o sfogliabile con le frecce della tastiera e con `LB` / `RB` del gamepad.

---

### 3.2 Opzioni di Rotazione Altamente Configurabili (`[ ⚙️ Personalizza Hero ]`)

L'utente ha pieno controllo su cosa vedere nella Hero. Cliccando sull'ingranaggio in basso a destra nella Hero (o dalle Impostazioni), si apre il pannello:

#### 1. Sorgente dei Titoli in Vetrina (Da dove prendere i giochi):
* **👑 Proposti del Giorno / Curati (Default):** Selezione bilanciata dei più grandi capolavori storici (1 da ogni console regina: SNES, Arcade, PS1, Recomp, Indie, Megadrive, N64).
* **⭐ Solo i Miei Preferiti:** La Hero diventa la tua bacheca personale: scorre solo tra i titoli che hai contrassegnato con la stella.
* **⏱️ Ultimi Giocati / Iniziati ("Continua la Partita"):** Mostra gli ultimi 5 giochi avviati per riprendere subito da dove avevi interrotto.
* **🎮 Sintonizzata sulla Piattaforma Attuale:** Se nella barra selezioni "Mega Drive", la Hero mostra i capolavori Mega Drive. Se selezioni "PC Indie", mostra i titoli Indie!
* **🎲 Shuffle Totale (Retro-Discovery):** Rimescola continuamente giochi casuali dall'intero catalogo per farti scoprire perle dimenticate.

#### 2. Timer e Comportamento di Rotazione:
* **Intervalli Selezionabili:**
  * `Disattivata (Rotazione solo manuale con frecce)`
  * `Veloce (5 secondi)`
  * `Standard (8 secondi - Consigliato)`
  * `Rilassata (15 secondi)`
* **Pausa Intelligente su Hover / Focus:**  
  Se il mouse si sofferma sulla Hero o se l'utente la seleziona con il controller per leggere la trama o i comandi, **il timer si mette automaticamente in pausa** e non cambia gioco sotto il naso dell'utente!

---

## 🎲 4. La Funzione "Retro-Roulette" (Sorprendimi!)

Per abbattere l'indecisione da "paralisi della scelta" quando hai migliaia di titoli:

1. **Accessibilità Immediata:**
   * Un pulsante distintivo con gradiente fucsia/viola `[ 🎲 Sorprendimi ]` collocato:
     - All'interno della Hero Banner.
     - Nella barra dei filtri superiore.
     - Mappato nativamente sul tasto **`(Y)` del controller Xbox** (o triangolo PlayStation).
2. **Animazione a Slot-Machine / Rullo Retrogaming:**
   * Quando cliccato, la card o la Hero mostra per 0.8 secondi uno scorrimento frenetico e divertente di 10 titoli casuali con tick sonoro retro opzionale.
   * Si arresta con effetto rimbalzo sul gioco prescelto, aprendo la scheda e permettendo l'avvio immediato con `(A)`.
3. **Filtro Intelligente della Roulette:**
   * Se sei su una console (es. Neo-Geo), estrae solo tra i giochi Neo-Geo.
   * Se hai attivo il filtro `[ 💾 Solo Scaricati ]`, estrae solo tra i giochi già pronti sul tuo PC.

---

## 🃏 5. Anatomia delle CARD di Gioco (Grid Cards)

Le card della griglia principale sono il punto di contatto più frequente: devono essere leggibili, scattanti e ricche di dettagli senza risultare caotiche.

```
┌──────────────────────────────────────────────┐
│ [ 🎮 SNES ]                     [ 💾 ] [ ⭐ ] │  <-- Badge Piattaforma + Stato + Fav
│                                              │
│                                              │
│                                              │
│              BOXART VERTICALE                │
│                 (2:3 o 3:4)                  │
│                                              │
│                                              │
├──────────────────────────────────────────────┤  <-- Sfumatura scura per il testo
│ Chrono Trigger                               │  <-- Titolo Grassetto Bianco
│ 1995 • Squaresoft • RPG                      │  <-- Anno, Produttore, Genere
│ ──────────────────────────────────────────── │
│ [ ▶ GIOCA SUBITO ]             [ 👥 1P / 2P ]│  <-- Quick-Action al passaggio mouse
└──────────────────────────────────────────────┘
```

### 5.1 Stati e Micro-Interazioni delle Card
* **Stato Riposo:**
  * Dimensioni responsive (min-width 180px, max 240px).
  * Raggio angoli: `16px`.
  * Bordo sottile: `1px solid rgba(255, 255, 255, 0.08)`.
  * Copertina WebP con caricamento asincrono progressivo.
* **Stato Hover (Mouse) & Focus (Gamepad):**
  * Elevazione elastica: `transform: translateY(-8px) scale(1.03)`.
  * Bordo illuminato con il colore di accento della piattaforma.
  * Ombra cinetica profonda: `box-shadow: 0 16px 36px rgba(0, 0, 0, 0.8), 0 0 25px rgba(accent, 0.35)`.
  * Comparsa dolce in dissolvenza delle azioni rapide in basso (`[ ▶ GIOCA ]` e `[ ⭐ ]`).
* **Stato Typographic Placeholder (Caso 3 Low-Storage a 0 byte immagini):**
  * Sfondo con gradiente vettoriale dinamico nei colori ufficiali della console.
  * Micro-griglia geometrica o filigrana SVG della console sullo sfondo.
  * Titolo in rilievo, anno e genere perfettamente leggibili al centro.
  * Nessun pixel di immagine scaricato: eleganza pura da rivista di design!

---

## 🔘 6. Design System dei PULSANTI (Button Tokens)

Ogni pulsante risponde a regole precise di colore, forma e gerarchia:

| Tipologia | Stile / Gradiente | Colore Testo | Utilizzo | Effetto Hover |
| :--- | :--- | :--- | :--- | :--- |
| **Primario (Hero / Play)** | `linear-gradient(135deg, #00e5b4 0%, #00b4d8 100%)` | `#080b10` (Nero Profondo) | Avvio Gioco, Download, Conferma | Glow ciano + leggera espansione (`scale 1.04`) |
| **Secondario (Glass)** | `rgba(255, 255, 255, 0.07)` + Blur | `#f8fafc` (Bianco) | Preferiti, Sfoglia, Indietro, Filtri | Schiarimento fondo + bordo bianco |
| **Roulette (Random 🎲)** | `linear-gradient(135deg, #a855f7 0%, #ec4899 100%)` | `#ffffff` | Sorprendimi / Casuale | Rotazione di 360° dell'icona dado |
| **Pericolo / Elimina** | `linear-gradient(135deg, #f43f5e 0%, #be123c 100%)` | `#ffffff` | Elimina ROM temporanea, Reset | Bordo cremisi brillante |
| **Configurazione (Setup)** | `rgba(255, 255, 255, 0.05)` | `#94a3b8` (Grigio Slate) | Utility `DoConfig.exe`, Opzioni Hero | Icona ingranaggio ruota di 90° |

---

## 🔍 7. Selezione di Liste con Filtri Intelligenti (Smart Filter Bar)

Per consentire ricerche immediate anche tra 50.000 titoli:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│  [ 🔍 Cerca titolo, sviluppatore o anno...               ]   [ 💾 Scaricati ] [ 🔓 Cloud ]
├────────────────────────────────────────────────────────────────────────────────────────┤
│  PIATTAFORME:  [ TUTTI ]  [ 🕹️ ARCADE ]  [ 🎮 SNES ]  [ ⚡ RECOMP ]  [ 👾 PC INDIE ] ...│
├────────────────────────────────────────────────────────────────────────────────────────┤
│  FILTRI:  [ ⭐ Preferiti ]  [ 👥 Multi-2P ]  [ Platform ]  [ Azione ]  [ RPG ]  [ Shmup ]  │
└────────────────────────────────────────────────────────────────────────────────────────┘
```

1. **Barra Console a Scorrimento Fluido:**
   * Icone vettoriali nitide con contatore dinamico dei giochi installati / totali.
   * Scorribile con mouse wheel o con i bumper `LB` / `RB` del gamepad.
2. **Switch Stato a Levetta Magnetica:**
   * `[ 💾 Solo Scaricati ]` (mostra solo i giochi già pronti e avviabili offline).
   * `[ 🔓 Tutto l'Archivio Cloud ]` (mostra l'universo intero con download al volo).
3. **Pillole di Genere & Feature:**
   * Tag interattivi ad attivazione immediata (senza ricaricare la pagina).
   * Possibilità di combinare più filtri: es. `SNES` + `Platform` + `Multi-2P`.
4. **Fuzzy Search con Evidenziazione in Tempo Reale:**
   * Ricerca istantanea con match tollerante agli errori di digitazione (es. "zelda oot" trova subito *The Legend of Zelda: Ocarina of Time*).

---

## 🎮 8. Navigazione Gamepad-First (HUD Always-On)

In basso a destra dello schermo compare una barra di stato permanente con le guide grafiche dei tasti del controller:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                                (A) Seleziona/Gioca  •  (B) Indietro  •  (X) Preferito ⭐│
│                                (Y) 🎲 Sorprendimi  •  (LB/RB) Piattaforme              │
└────────────────────────────────────────────────────────────────────────────────────────┘
```
* **Focus Trap Elegante:** Quando si apre una modale (es. il Wizard o l'Arena), il focus del gamepad resta vincolato all'interno della finestra per non perdersi nello sfondo.
* **Auto-Scroll al Focus:** Quando navighi nella griglia col D-Pad o con la levetta analogica, la pagina scorre fluidamente per mantenere sempre la card selezionata al centro della visuale.

---

## ✅ Riepilogo delle Decisioni di Design

1. **Hero Widescreen:** Presente di default nella Home generale, personalizzabile con 5 sorgenti (Proposti, Preferiti, Recenti, Piattaforma, Shuffle) e timer configurabile con pausa su hover.
2. **Card 3D:** Rapporto 2:3 moderno, boxart con lazy-load, velo gradiente per il testo e modalità vettoriale a 0 byte per profili a basso storage.
3. **Pulsanti e Micro-Interazioni:** Bottoni ad alto contrasto con forme squircle (`12px`), effetti glow su hover ed elevazione isometrica.
4. **Retro-Roulette:** Integrata nativamente nella Hero e via controller `(Y)`.
5. **Leggibilità Totale:** Contrasti certificati per schermi TV e monitor desktop moderni.
