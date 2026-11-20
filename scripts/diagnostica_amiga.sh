#!/bin/bash
# Script diagnostic Amiga - RetroHub
# Verifica lo stato dell'installazione Amiga

echo "🧪 Diagnostica Amiga - ReteHub"
echo "================================"
echo ""

# Verifica struttura cartelle
echo "📁 Struttura cartelle:"
if [ -d "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA" ]; then
    echo "  ✅ Cartella AMIGA trovata"
    ls -la "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA" 2>/dev/null || echo "    (vuota)"
else
    echo "  ❌ Cartella AMIGA non trovata"
fi

echo ""
echo "🎮 ROM Giochi Amiga:"

# Verifica Lemmings
if [ -d "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA/roms/Lemmings (1991)" ]; then
    echo "  ✅ Lemmings cartella trovata"
    ls -la "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA/roms/Lemmings (1991)" 2>/dev/null
else
    echo "  ❌ Lemmings cartella non trovata"
fi

# Verifica Lotus
if [ -d "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA/roms/Lotus Esprit Turbo Challenge (1992)" ]; then
    echo "  ✅ Lotus cartella trovata"
    ls -la "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA/roms/Lotus Esprit Turbo Challenge (1992)" 2>/dev/null
else
    echo "  ❌ Lotus cartella non trovata"
fi

echo ""
echo "⚙️  Emulatori disponibili:"

# Verifica WinUAE
if [ -f "C:/Program Files/WinUAE/winUAE.exe" ] || [ -f "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA/winuae.exe" ]; then
    echo "  ✅ WinUAE trovato"
else
    echo "  ❌ WinUAE non installato"
fi

# Verifica FS-UAE
if [ -f "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/AMIGA/fs-uae.exe" ]; then
    echo "  ✅ FS-UAE trovato"
fi

echo ""
echo "📝 Configurazione C64 (per confronto):"
ls -la "C:/Users/ClaudioDall'Ara/Desktop/Clod/emu/C64/roms" 2>/dev/null | head -5

echo ""
echo "✅ Diagnostica completata"