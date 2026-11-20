# 🔧 Diagnostica Amiga vs C64 - RetroHub

## 📋 Problema Identificato

### 1. Bug di Raggruppamento (FIXED)
**File**: `src/launcher.rs` (riga 202)

**PRIMA** (errore):
```rust
// 12. Commodore 64 & Amiga
if sys_upper == "C64" || sys_upper == "COMMODORE" || sys_upper == "VIC20" || sys_upper == "AMIGA" {
    let base_roms = PathBuf::from(r"C:\Users\...\emu\C64\roms");
```

**DESPRÌ** (corretto):
```rust
// 12a. Commodore 64 (C64, VIC20, PLUS4)
if sys_upper == "C64" || sys_upper == "COMMODORE" || sys_upper == "VIC20" || sys_upper == "PLUS4" {
    let base_roms = PathBuf::from(r"C:\Users\...\emu\C64\roms");
}

// 12b. Commodore Amiga
if sys_upper == "AMIGA" {
    let base_roms = PathBuf::from(r"C:\Users\...\emu\AMIGA\roms");
}
```

## 📂 Struttura Cartelle Amiga

```
✅ CREATA: C:\Users\...\emu\AMIGA\
├── roms\                    (per le ROM .adf)
├── AmigaOS3.1\              (per il sistema operativo)
├── Avvia_Amiga_WinUAE.bat   (loader script)
└── README_AMIGA.md          (istruzioni)
```

## 🎮 Giochi Lemmings & Lotus

| Gioco | Versione | Estensione | Percorso Atteso |
|-------|----------|------------|-----------------|
| Lemmings | Amiga | `.adf` | `emu\AMIGA\roms\Lemmings (1991)\Gioca.lnk` |
| Lotus Turbo | Amiga | `.adf` | `emu\AMIGA\roms\Lotus Esprit Turbo Challenge (1992)\Gioca.lnk` |

**ATTENZIONE**: Il C64 ha già Lemmings Reloaded e Lotus Esprit Turbo Challenge in:
`emu\C64\roms\[ nome ]\`

## ❌ Problemi Rimanenti

### 1. Mancano le ROM Amiga
- **Lemmings (Amiga)**: `Lemmings (1991).adf`
- **Lotus Turbo Challenge (Amiga)**: `Lotus Esprit Turbo Challenge (1992).adf`

Le ROM Amiga vanno posizionate in:
```
C:\Users\...\emu\AMIGA\roms\Lemmings (1991)\
  └── Gioca.lnk -> avvia Lemmings (1991).adf

C:\Users\...\emu\AMIGA\roms\Lotus Esprit Turbo Challenge (1992)\
  └── Gioca.lnk -> avvia Lotus Esprit Turbo Challenge (1992).adf
```

### 2. Manco l'Emulatore WinUAE
- **Richiesto**: WinUAE 4.10+ (64-bit)
- **Percorso**: `C:\Program Files\WinUAE\winUAE.exe`
- **Oppure**: `C:\Users\...\emu\AMIGA\winUAE.exe`

### 3. ReferenceError UI (NON RISOLTO)
**File**: `src/ui.html` (riga 5491)

```javascript
updateCardInstallState is not defined
```

**Causa**: Funzione mancante nel codice JavaScript del frontend Webview.
**Soluzione**: Aggiornare il codice UI con la funzione corretta o rimuovere la chiamata.

## ✅ Modifiche Apply

1. ✅ **launcher.rs**: Separiamo Amiga da C64
2. ✅ **launcher.rs**: Aggiunta estensione `.adf` per ricerca ROM
3. ✅ **launcher.rs**: Aggiunto supporto WinUAE nel find_matching_emulator
4. ✅ **Struttura cartelle**: Creata `emu\AMIGA\`
5. ✅ **Script loader**: Creato `Avvia_Amiga_WinUAE.bat`

## 🚀 Prossimi Passi

1. **Installa WinUAE**: https://www.winuae.net/
2. **Aggiungi Kickstart 3.1** (ROM Kickstart Amiga)
3. **Scarica ROM gioco Amiga** (Lemmings, Lotus)
4. **Riprendi l'applicazione RetroHub**

## 📊 Verifica Post-Installazione

Dopo aver installato WinUAE e le ROM, da terminale:
```powershell
cd "C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game"
cargo build --release
.\target\release\retro-hub.exe
```