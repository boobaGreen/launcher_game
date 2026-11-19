# 🔍 Architettura Ricerca & Filtri UX: Differenze, Campi e Combinabilità

Questo documento formalizza l'analisi tecnica e l'architettura UX/UI che governa l'interazione tra la **Barra di Ricerca Globale (Header)** e la **Finestra dei Filtri Avanzati ([ Filtri ])** in RetroHub.

---

## 1. Visione d'Insieme: Due Scopi Complementari

In una libreria retrogaming che spazia tra decine di migliaia di titoli su oltre 24 piattaforme, gli utenti hanno due modalità mentali di navigazione completamente diverse:

| Caratteristica | 🔍 Barra di Ricerca Istantanea (Header) | 🎛️ Modale Filtri Avanzati ([ Filtri ]) |
| :--- | :--- | :--- |
| **Modalità Utente** | **"So già cosa voglio"** (Ricerca mirata, veloce) | **"Voglio scoprire / esplorare"** (Faceted browsing, tassonomia) |
| **Punto di Ingresso** | Campo testo sempre visibile in alto a sinistra | Finestra di dialogo dedicata con controlli visivi ricchi |
| **Velocità & Feedback** | Istantanea lettera per lettera (`oninput`, debounced) | Interattiva con contatore live preview prima dell'applicazione |
| **Interfaccia** | Input singolo con icona lente e pulsante cancella `✕` | Chip visivi per epoca, genere, publisher + selettori numerici |
| **Persistenza** | Volatile (si cancella con un clic o con ESC) | Strutturata (salvabile come Raccolta Custom permanente) |

---

## 2. Dettaglio dei Campi e Dati Elaborati

### A. Barra di Ricerca Istantanea (Full-Text Fuzzy Search)
Lavora come un motore di ricerca a tutto campo. Quando l'utente digita una stringa (es. `"capcom"`, `"1994"`, `"doom"`, `"platform"`), la barra esegue una ricerca a testo libero su **4 attributi contemporaneamente**:
1. **Titolo del gioco** (`g.title`): es. *"Super Mario World"*, *"The Legend of Zelda"*.
2. **Produttore / Software House** (`g.developer`): es. *"Capcom"*, *"Konami"*, *"Nintendo"*.
3. **Genere di gioco** (`g.genre`): es. *"Platform"*, *"Action RPG"*, *"Shoot 'em up"*.
4. **Anno di pubblicazione** (`g.year`): es. *"1992"*, *"1998"*.

> **Peculiarità UX:** È adatta per stringhe brevi o frammenti di nome. Se l'utente scrive `"kart"`, trova al volo sia *"Super Mario Kart"* (SNES) che *"Crash Team Racing"* (PS1).

---

### B. Finestra Filtri Avanzati (Faceted Categorical Filtering)
Lavora con vincoli analitici specifici, permettendo combinazioni che un singolo campo di testo non potrebbe mai esprimere in modo intuitivo:

1. **Epoca Storica & Intervallo Anni (Dual Range 1972–2026):**
   - Non si limita a un tetto massimo: permette di scegliere sia **Anno Inizio (Da)** che **Anno Fine (A)**.
   - Chip di decennio con 1 clic:
     - `Anni '70 (1972–1979)`: Atari 2600, Magnavox Odyssey.
     - `Anni '80 (1980–1989)`: NES, C64, Master System, Arcade d'oro.
     - `Anni '90 Golden Era (1990–1999)`: SNES, Mega Drive, Neo-Geo, PS1, N64.
     - `Anni 2000 (2000–2009)`: PS2, Dreamcast, GBA, GameCube.
     - `Moderni / Indie (2010–2026)`: Recomp, Indie freeware, PC nativo.

2. **Generi Multipli Combinabili (Multi-Select Chips):**
   - Chip interattivi che si accendono con bagliore ciano (`Platform`, `Azione / Avventura`, `Picchiaduro`, `Shoot 'em up / FPS`, `RPG`, `Corse`, `Sport`, `Avventura Grafica`, `Arcade`).
   - Se l'utente seleziona sia *Picchiaduro* che *Azione*, il motore mostra giochi appartenenti a uno qualsiasi dei generi scelti (OR interno alla categoria genere).

3. **Produttore / Sviluppatore Strutturato (Anti-Password Autofill & Iconic Chips):**
   - Risolto alla radice il problema del gestore password (aggiunti attributi `type="search"`, `data-lpignore="true"`, `data-form-type="other"` e `autocomplete="off"`).
   - Chip rapidi per le software house leggendarie: *Capcom, Konami, Nintendo, Sega, Square Enix, SNK, Namco, id Software, LucasArts, Commodore, Sony, Atari*.

4. **Piattaforma Specifica & Stato Storage:**
   - Possibilità di isolare una singola console (es. solo *Mega Drive*) e lo stato fisico del file (*Tutto il catalogo cloud on-demand* vs *Solo già scaricati / installati*).
   - Filtri rapidi per *Solo Preferiti ⭐* e *Solo 2+ Giocatori 👥*.

---

## 3. Come si Combinano (Architettura a Intersezione Logica AND)

I due strumenti non si escludono a vicenda, ma **collaborano per restringimento progressivo (Boolean AND)**:

$$\text{Giochi Visibili} = \text{MacroFamiglia} \cap \text{FiltriAvanzati} \cap \text{BarraRicerca} \cap \text{Storage}$$

### Esempio Pratico di Flusso UX:
1. L'utente apre la modale **[ Filtri ]**:
   - Clicca il chip **Anni '90 (1990–1999)**.
   - Clicca il chip **Capcom**.
   - Clicca il chip **Picchiaduro**.
   - Il contatore live mostra istantaneamente: `🎯 8 giochi trovati`.
   - Clicca su `[ 🔍 Mostra 8 Giochi Trovati ]`.
2. Nella griglia compaiono gli 8 picchiaduro Capcom degli anni '90 (*Street Fighter II, Super SFII, Darkstalkers, Marvel vs Capcom, ecc.*).
3. Il pulsante della navbar si trasforma in **`[ Filtri (3) ]`** con un badge ciano ad alta visibilità, segnalando che ci sono 3 filtri attivi.
4. L'utente digita ora `"turbo"` nella **Barra di Ricerca** in alto:
   - La griglia si restringe istantaneamente a 1 solo titolo: *Street Fighter II Turbo*.
5. Se l'utente cancella `"turbo"` con il pulsante `✕`, torna agli 8 giochi Capcom anni '90 senza dover reimpostare nulla.
6. Con il pulsante **`[ 💾 Salva Selezione in Raccolta ]`**, l'utente salva questa lista creata su misura come playlist permanente *"Picchiaduro Capcom 90s"* per richiamarla o proiettarla sulla Hero in qualunque momento!

---

## 4. Linee Guida UX/UI Implementate

1. **Visibilità dello Stato (Zero Ambiguità):** L'utente sa sempre quanti e quali filtri sono attivi grazie al badge numerico sul pulsante `[ Filtri (N) ]` e al badge contatore `X giochi visibili`.
2. **Anteprima Live a Zero Clic:** Dentro la modale non serve applicare alla cieca: il badge `🎯 N giochi trovati` calcola in tempo reale le corrispondenze a ogni clic o pressione di tasto.
3. **Resetta Senza Attrito:** Pulsante `✕ Resetta` presente sia nella modale che nella barra di ricerca che nel banner di nessun risultato trovato.
4. **Protezione Privacy / Password:** Nessuna interferenza da parte dei password manager del browser sui campi di ricerca sviluppatore.
