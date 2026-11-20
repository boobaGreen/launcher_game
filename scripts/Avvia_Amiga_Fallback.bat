@echo off
REM ============================================
REM RetroHub - Amiga Loader (Fallback)
REM ============================================
REM Questo script gestisce il lancio giochi Amiga
REM quando WinUAE non e' installato o non configurato

setlocal EnableDelayedExpansion

set GAME_NAME=%1
set EMU_DIR="C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\AMIGA"
set ROMS_DIR="%EMU_DIR%\roms\%GAME_NAME%"
set ADF_FILE="%ROMS_DIR%\%GAME_NAME%.adf"

REM Verifica cartella gioco
if not exist %ROMS_DIR% (
    echo.
    echo ============================================
    echo ERRORE: Cartella gioco non trovata!
    echo ============================================
    echo.
    echo Per giocare a %GAME_NAME%:
    echo 1. Installa WinUAE da https://www.winuae.net/
    echo 2. Aggiungi Kickstart 3.1 (ROM Amiga)
    echo 3. Scarica la ROM da archive.org
    echo    -> https://archive.org/details/Lemmings_(USA)
    echo.
    echo Cartella attesa:
    echo   %ROMS_DIR%
    echo.
    echo Press any key to open folder...
    pause >nul
    explorer "%EMU_DIR%\roms"
    exit /b 1
)

REM Verifica se c'e' l'ADF
if not exist %ADF_FILE% (
    echo.
    echo ============================================
    echo ATTENZIONE: File ADF non trovato!
    echo ============================================
    echo.
    echo Il file %ADF_FILE% e' necessario per giocare.
    echo.
    echo Scarica da:
    echo   https://archive.org/details/Lemmings_(USA)
    echo   (o ricerca su Google: "%GAME_NAME% Amiga ROM adf")
    echo.
    echo Press any key to open archive.org...
    pause >nul
    start "" "https://archive.org/search.php?query=%GAME_NAME%+amiga+adf"
    exit /b 1
)

REM Verifica WinUAE
set WINUAE_EXE="C:\Program Files\WinUAE\winUAE.exe"
if not exist %WINUAE_EXE% (
    echo.
    echo ============================================
    echo ERRORE: WinUAE non installato!
    echo ============================================
    echo.
    echo WinUAE e' il emulatore Amiga richiesto.
    echo Installalo da: https://www.winuae.net/
    echo.
    echo Press any key to open download page...
    pause >nul
    start "" "https://www.winuae.net/"
    exit /b 1
)

echo.
echo ============================================
echo Avvio %GAME_NAME% su Amiga 500...
echo ============================================
echo.

REM Avvia WinUAE con il gioco
%WINUAE_EXE% -model=500 -floppy0=%ADF_FILE% -kickstart=3.1 -fastRAM=1MB -joystick=1 -sound=1

echo.
echo Gioco terminato o interrotto.
pause
endlocal