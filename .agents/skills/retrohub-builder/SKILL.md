---
name: retrohub-builder
description: Procedure operative per compilare, testare, ottimizzare con LTO, incrementare la versione ed effettuare commit e push del launcher RetroHub.
---

# RetroHub Builder Skill

Questa skill guida l'agente nelle procedure di compilazione, packaging e rilascio di **RetroHub**.

## 1. Controllo di Sintassi & Compilazione Veloce
Prima di avviare build pesanti, esegui un controllo rapido:
```powershell
cargo check
```

## 2. Compilazione Release Ottimizzata (LTO)
Per generare il binario di produzione ad alte prestazioni (< 10 MB):
```powershell
cargo build --release
```
Il binario generato si trova in `target/release/retro-hub.exe`. Per aggiornare il file radice usato dal launcher:
```powershell
Copy-Item "target/release/retro-hub.exe" "retro-hub.exe" -Force
```

## 3. Version Bumping
Quando rilasci una nuova versione:
1. Aggiorna `version = "0.X.Y"` in `Cargo.toml`.
2. Aggiorna la dicitura `v0.X.Y` in `src/ui.html` (header e footer dell'interfaccia).
3. Aggiorna l'intestazione di `ROADMAP.md`.

## 4. Checklist Pre-Commit
- [ ] `git status` per verificare file modificati e untracked.
- [ ] Nessun file di log, cache o binario di test temporaneo lasciato untracked.
- [ ] Commit con convenzione semantica (es. `feat(v0.X.Y): descrizione sintetica`).
- [ ] Push sul branch `refactor-modular-rust`:
```powershell
git push origin refactor-modular-rust
```
