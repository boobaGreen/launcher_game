# 🌐 Roadmap & Architettura i18n Multilingua

Questo documento definisce la strategia di internazionalizzazione (i18n) e localizzazione (l10n) di **RetroHub**, pianificando l'estensione multilingua del launcher a partire dall'Inglese.

---

## 1. Stato Attuale del Progetto

- **Lingua Attiva:** Attualmente l'interfaccia utente, i toast, i testi della Hero, i modali e la documentazione sono al **100% in lingua Italiana (`it-IT`)**.
- **Strategia Operativa per le Release v3.x:**
  - Come concordato, per il momento **manteniamo l'interfaccia focalizzata su una singola lingua (Italiano)**.
  - Questa scelta ingegneristica consente di mantenere il codice snello, velocizzare i cicli di compilazione ed evitare overhead di traduzione durante le fasi di rifinitura UX/UI, logica di ricerca e gestione ROM.

---

## 2. Roadmap di Rilascio delle Lingue

L'introduzione del multilingua avverrà per scaglioni progressivi:

| Fase | Codice Lingua | Nome Lingua | Stato / Priorità | Note di Rilascio |
| :---: | :---: | :---: | :---: | :--- |
| **0** | `it-IT` | 🇮🇹 Italiano | **Attiva (Default)** | Lingua madre di riferimento di tutte le funzioni. |
| **1** | `en-US` | 🇬🇧 English | **Prossima Release (P1)** | Prima lingua secondaria; apre a tutta la community retrogaming internazionale. |
| **2** | `es-ES` | 🇪🇸 Español | **Successiva (P2)** | Mercato retrogaming europeo e latinoamericano (compatibilità archivi spagnoli). |
| **3** | `fr-FR` | 🇫🇷 Français | **Pianificata (P3)** | Mercato europeo e standard documentale (ScreenScraper / Recalbox). |
| **4** | `de-DE` | 🇩🇪 Deutsch | **Pianificata (P4)** | Ecosistema Commodore / Amiga europeo. |
| **5** | `ja-JP` | 🇯🇵 日本語 | **Pianificata (P5)** | Console native giapponesi (PC Engine, X68000, Super Famicom). |

---

## 3. Architettura Tecnica i18n Prevista

Per abilitare il cambio lingua a caldo senza ricaricare l'applicazione o riavviare il server Rust, utilizzeremo una struttura a dizionari JSON leggeri:

### A. Struttura dei File di Traduzione
```text
launcher_game/
  └── assets/
      └── locales/
          ├── it.json   (Dizionario base Italiano)
          ├── en.json   (Dizionario Inglese - Fase 1)
          ├── es.json   (Dizionario Spagnolo - Fase 2)
          └── ...
```

### B. Esempio Dizionario JSON (`en.json`)
```json
{
  "nav": {
    "search_placeholder": "Search game, developer, year or genre...",
    "filters": "Filters",
    "surprise": "Surprise Me",
    "arena": "Arena",
    "setup_wizard": "Setup Wizard"
  },
  "hero": {
    "random_discovery": "RANDOM DISCOVERY",
    "play_now": "PLAY NOW",
    "download_and_play": "DOWNLOAD & PLAY",
    "favorite": "Favorite",
    "random": "Random"
  },
  "filters_modal": {
    "title": "Advanced Multi-Filter Search",
    "decade_label": "Historic Era & Year Range",
    "genres_label": "Game Genres (Combinable)",
    "publisher_label": "Publisher / Developer",
    "all_epochs": "All Eras",
    "reset": "Reset All",
    "save_collection": "Save View to Collection",
    "apply": "Show Matching Games"
  }
}
```

### C. Selettore UX nelle Opzioni (Settings Modal)
Nella finestra delle Opzioni/Impostazioni verrà aggiunto un dropdown elegante con bandiere:
```html
<div class="settings-row">
    <label>🌐 Lingua Interfaccia / Language</label>
    <select id="languageSelect" onchange="changeLanguage(this.value)">
        <option value="it" selected>🇮🇹 Italiano</option>
        <option value="en">🇬🇧 English</option>
        <option value="es">🇪🇸 Español</option>
    </select>
</div>
```
La preferenza scelta verrà memorizzata in `localStorage.setItem('retrohub_lang', lang)` e caricata all'avvio.
