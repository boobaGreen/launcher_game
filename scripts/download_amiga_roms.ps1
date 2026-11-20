# PowerShell Script - Download ROM Amiga
# RetroHub - Commodore Amiga Collection

param(
    [string]$GameTitle = $null,
    [switch]$ListAvailable = $false
)

$ErrorActionPreference = "Stop"
$EMU_PATH = "C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\AMIGA"
$ROMS_PATH = "$EMU_PATH\roms"

# Funzione per verificare se una ROM esiste già
function Test-ROMExists {
    param([string]$name)
    $patterns = @("$ROMS_PATH", "$ROMS_PATH\$name", "$ROMS_PATH\$name*")
    foreach ($p in $patterns) {
        if (Test-Path $p) { return $true }
    }
    return $false
}

# Funzione download da Archive.org
function Invoke-ArchiveDotOrgDownload {
    param(
        [string]$ArchiveId,
        [string]$Destination
    )
    
    $url = "https://archive.org/download/$ArchiveId"
    Write-Host "Downloading from: $url" -ForegroundColor Yellow
    
    try {
        # Usa Invoke-WebRequest per scaricare
        $response = Invoke-WebRequest -Uri "$url?format=txt" -TimeoutSec 30 -ErrorAction SilentlyContinue
        # Nota: per file binari usa -OutFile con URL diretto
        return $true
    } catch {
        Write-Host "Download fallito: $_" -ForegroundColor Red
        return $false
    }
}

# Lista giochi Amiga disponibili
$AmigaGames = @(
    @{ Name = "Lemmings"; ArchiveId = "Lemmings_(USA)"; Year = 1991; Genre = "Puzzle" },
    @{ Name = "Lotus Esprit Turbo Challenge"; ArchiveId = "lotus-esprit-turbo-challenge-1990"; Year = 1990; Genre = "Racing" },
    @{ Name = "Turrican"; ArchiveId = "turrican-1990"; Year = 1990; Genre = "Platform" },
    @{ Name = "Shadow of the Beast"; ArchiveId = "shadow-of-the-beast-1989"; Year = 1989; Genre = "Platform" },
    @{ Name = "The Chaos Engine"; ArchiveId = "chaos-engine-1993"; Year = 1993; Genre = "Shooter" }
)

if ($ListAvailable) {
    Write-Host "`n=== Giochi Amiga Amiga Disponibili ===" -ForegroundColor Cyan
    Write-Host "Percorso ROM: $ROMS_PATH`n"
    
    foreach ($game in $AmigaGames) {
        $exists = Test-ROMExists $game.Name
        $status = if ($exists) { "✓ INSTALLATO" } else { "✗ DA INSTALLARE" }
        Write-Host ("[{0}] {1} ({2}) - {3}" -f $status, $game.Name, $game.Year, $game.Genre)
    }
    exit 0
}

# Ricerca gioco specifico
if ($GameTitle) {
    $game = $AmigaGames | Where-Object { $_.Name -like "*$GameTitle*" }
    if ($game) {
        Write-Host "Trovato: $($game.Name)" -ForegroundColor Green
        if (-not (Test-ROMExists $game.Name)) {
            Write-Host " Scarica su: https://archive.org/details/$($game.ArchiveId)" -ForegroundColor Cyan
        }
    } else {
        Write-Host "Gioco non trovato nell'elenco." -ForegroundColor Red
    }
    exit 0
}

# Modalità interattiva
Write-Host @"
===========================================
  RetroHub - Download ROM Amiga
===========================================

Scegli un'opzione:
1. Lista giochi disponibili
2. Cerca gioco specifico
3. Apri cartella ROM
4. Esci

Inserisci numero: " -ForegroundColor Cyan -NoNewline

$choice = Read-Host

switch ($choice) {
    "1" { 
        $ListAvailable = $true
        $MyInvocation.MyCommand.Path | ForEach-Object { & $_ -ListAvailable }
    }
    "2" {
        Write-Host "`nNome gioco: " -ForegroundColor Yellow -NoNewline
        $title = Read-Host
        & $MyInvocation.MyCommand.Path -GameTitle $title
    }
    "3" { 
        if (Test-Path $ROMS_PATH) {
            explorer $ROMS_PATH
        } else {
            Write-Host "Cartella ROM non trovata: $ROMS_PATH" -ForegroundColor Red
        }
    }
    "4" { exit 0 }
    default { Write-Host "Scelta non valida" -ForegroundColor Red }
}