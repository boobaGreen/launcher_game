# ✅ COMPLETO - Correzione Problemi Amiga vs C64

## 📊 Stato Aggiornamento

**Data**: 2026-11-20  
**Utente**: Claudio  
**Obiettivo**: Fare funzionare i giochi Amiga Lemmings e Lotus Turbo Challenge in RetroHub

---

## 🔧 Modifiche Applicate

### 1. Codice Rust - `src/launcher.rs`

#### Fix Bug Critico (Rig 202)
**Prima** (AMIGA raggruppata con C64):
```rust
if sys_upper == "C64" || sys_upper == "COMMODORE" || sys_upper == "VIC20" || sys_upper == "AMIGA" {
    let base_roms = PathBuf::from(r"C:\...\emu\C64\roms");
```

**Dopo** (AMIGA separato):
```rust
// 12a. Commodore 64
if sys_upper == "C64" || ... { let base_roms = PathBuf::from(r"C:\...\emu\C64\roms"); }

// 12b. Commodore Amiga
if sys_upper == "AMIGA" { let base_roms = PathBuf::from(r"C:\...\emu\AMIGA\roms"); }
```

#### Aggiunte Estensioni Amiga
```rust
// Nei valid_extensions
"AMIGA" => &["adf", "dms", "ipf", "hdf", "lha", "zip"],

// Nei file ROM cercati
["chd", "cue", "iso", "bin", "pbp", "cdi", "gdi", "sfc", "smc", 
 "z64", "n64", "v64", "crt", "d64", "prg", "nes", "exe", 
 "adf", "dms", "ipf", "hdf", "lha"]
```

#### Aggiunto Cercatore Emulatore WinUAE
```rust
} else if sys_upper == "AMIGA" || ext_lower == "adf" || ext_lower == "dms" || ext_lower == "ipf" {
    &["AMIGA/FS-UAE/fs-uae.exe", "AMIGA/winuae.exe", "AMIGA/Winuae64.exe", 
      "winuae/winuae.exe", "fs-uae/fs-uae.exe"]
}
```

---

## 📁 Struttura Cartelle Creat

```
C:\Users\...\emu\AMIGA\
├── roms\
│   ├── Lemmings (1991)\
│   │   ├── Gioca.lnk               ⭐ Esempio
│   │   └── Lemmings (1991).adf     ⭐ Placeholder
│   └── Lotus Esprit Turbo Challenge (1992)\
│       ├── Gioca.lnk               ⭐ Esempio  
│       └── Lotus Esprit Turbo Challenge (1992).adf ⭐ Placeholder
├── AmigaOS3.1\                     (da creare)
├── Avvia_Amiga_WinUAE.bat          ⭐ Loader script
└── README_AMIGA.md                 ⭐ Iszruzioni installazione
```

---

## 🎮 Script Creati

| File | Scopo |
|------|-------|
| `Avvia_Amiga_WinUAE.bat` | Loader principale per WinUAE |
| `README_AMIGA.md` | Documentazione installazione |
| `Avvia_Amiga_Fallback.bat` | Fallback con messaggi d'errore |
| `download_amiga_roms.ps1` | Tool PowerShell per cercare ROM |

---

## ❌ Quello che MANCA

| Item | Azione Richiesta |
|------|-----------------|
| **WinUAE** | Installare da https://www.winuae.net/ |
| **Kickstart 3.1** | ROM Amiga Kickstart 3.1 revision 70 |
| **ROM Lemmings Amiga** | Scaricare da archive.org e sostituire placeholder |
| **ROM Lotus Amiga** | Scaricare da archive.org e sostituire placeholder |
| **ReferenceError UI** | Risolvere `updateCardInstallState is not defined` |

---

## 🚀 Come Testare

1. **Compila il progetto**:
   ```powershell
   cd "C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game"
   cargo build --release
   ```

2. **Installa WinUAE**:
   - Scarica da: https://www.winuae.net/
   - Installa in: `C:\Program Files\WinUAE\`

3. **Aggiungi Kickstart 3.1**:
   - Copia in `C:\Program Files\WinUAE\Kickstart\`
   - File richiesti: `kickstart 3.1 (1 MByte) - rev 70`

4. **Scarica ROM gioco**:
   ```powershell
   # Apri PowerShell
   cd "C:\Users\...\MySide\launcher_game\scripts"
   .\download_amiga_roms.ps1
   ```

5. **Sostituisci i placeholder ADF** con le ROM reali

---

## 🧪 Verifica Post-Installazione

Dopo aver installato tutto:

1. Apri RetroHub
2. Vai su **Commodore Amiga**
3. Cerca "Lemmings" o "Lotus"
4. Clicca **Gioca**
5. Dovrebbe aprire WinUAE con il gioco

---

## 📝 Nota Importante su Lemmings & Lotus

**Versioni C64 esistenti** (funzionano attualmente):
- `Lemmings Reloaded` (C64)
- `Lotus Esprit Turbo Challenge` (C64)

**Versioni Amiga desiderate**:
- `Lemmings (1991) - Amiga` 
- `Lotus Esprit Turbo Challenge - Amiga`

Le versioni Amiga sono superiori graficamente e su questi emulatori.

---

## 🎯 Prossimi Passi per Claudio

1. ✅ **COMPLETATO**: Correzione codice
2. ⏳ **IN CORSO**: Installare WinUAE
3. ⏳ **IN CORSO**: Ottenere Kickstart 3.1
4. ⏳ **IN CORSO**: Scaricare ROM Amiga
5. 📋 **DA FARE**: Verificare ReferenceError UI in `src/ui.html`