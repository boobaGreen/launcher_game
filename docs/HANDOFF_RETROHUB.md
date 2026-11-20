# Passaggio di Consegna (Handoff) — Progetto RetroHub

Questo documento formalizza il passaggio di consegna definitivo dalla sessione precedente (`53d7d039-d91a-4937-8d32-83dc0f056807`) al workspace corrente del progetto `launcher_game`.

---

## 1. Stato del Repository e Branch

- **Repository**: `boobaGreen/launcher_game`
- **Branch attivo**: `refactor-modular-rust`
- **Target OS**: Windows (con astrazione cross-platform per futuri port Linux/SteamOS)
- **Database Gioco**: `retro_hub.db` (oltre 1.679 giochi catalogati tra MAME, Laserdisc, Daphne, ScummVM, DOS, C64, Amiga, Console)

---

## 2. Decisioni Architetturali & Modifiche Recenti

1. **Gestione Multi-Monitor (MAME / Arcade)**:
   - Diagnosticato e risolto il problema dello schermo nero e mouse bloccato al lancio di MAME su schermi secondari.
   - Aggiunto rilevamento display con API Windows (`GetMonitorInfoW`, `EnumDisplayMonitors`) in `src/config.rs` esposto su `/api/system/displays`.
   - Tool di validazione: `test_osd.ps1`.

2. **Launcher MS-DOS & ScummVM**:
   - In `src/launcher.rs`, implementato `to_short_path` tramite `GetShortPathNameW` per garantire che i percorsi con spazi vengano passati a DOSBox Staging nel formato 8.3 compatibile.
   - Aggiunta risoluzione automatica delle sottocartelle `games/` e `roms/` per ScummVM e i sistemi retro computer / console.

3. **Copertine & Scraping**:
   - Laserdisc: Aggiornate/generate copertine per *GP World*, *Interstellar*, *Mach 3*, *Super Don Quix-ote*, *Thayer's Quest*, *Us vs Them*.
   - PC Home / ScummVM: Configurato il supporto per la saga *Broken Sword* (*Il ritorno dei templari*, *La profezia dei Maya*, *Il segreto dei templari*).
   - In `src/server.rs`: Gestione unificata del routing `/api/cover` per servire cover dirette e fallback.

4. **UX/UI Ordinamento**:
   - Aggiunto toggle universale Ascendente/Discendente per tutti i filtri di ordinamento (Titolo, Anno, Rating, Ultimo giocato).
   - UI fluida e integrata in `src/ui.html`.

---

## 3. Sicurezza e Backup

La sessione precedente da 123 MB è stata archiviata in sicurezza con backup completo di tutti i database e report in:
`~/.gemini/antigravity/backups/RetroHub_chat_backup_53d7d039`

---

## 4. Prossimi Passi

- [ ] Verifica visiva / funzionale del toggle Ascendente/Discendente in `src/ui.html`.
- [ ] Compilazione `cargo build --release` con bump di versione.
- [ ] Commit e push sul branch `refactor-modular-rust`.
