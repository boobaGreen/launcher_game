**╔══════════════════════════════════════════════════╗**
**║     RetroHub Refactor - Script di Test Auto    ║**
**╚══════════════════════════════════════════════════╝**

# 1. Verifica Branch
Write-Host "[1/6] Verifica Branch Refactoring..." -ForegroundColor Yellow
$branch = git.exe rev-parse --abbrev-ref HEAD
if ($branch -ne "refactor-modular-rust") {
    Write-Host "  ❌ Non sei sul branch corretto!" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ Branch: $branch" -ForegroundColor Green

# 2. Verifica Dipendenze
Write-Host "[2/6] Verifica Dipendenze Rust..." -ForegroundColor Yellow
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "  ❌ Cargo non trovato!" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ Cargo disponibile" -ForegroundColor Green

# 3. Pulizia Build Precedente
Write-Host "[3/6] Pulizia build precedente..." -ForegroundColor Yellow
git.exe clean -fdx 2>$null
Write-Host "  ✅ Cartelle di build pulite" -ForegroundColor Green

# 4. Build in Release
Write-Host "[4/6] Compilazione Release..." -ForegroundColor Yellow
cargo.exe build --release 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ❌ Errore compilazione!" -ForegroundColor Red
    exit 1
}
Write-Host "  ✅ Compilazione riuscita" -ForegroundColor Green

# 5. Creazione Collegamento Desktop
Write-Host "[5/6] Creazione collegamento su Desktop..." -ForegroundColor Yellow
$target = "C:\Users\ClaudioDall'Ara\Desktop\Clod\MySide\launcher_game\target\release\retro-hub.exe"
$shortcutPath = "$env:USERPROFILE\Desktop\RetroHub-Refactor-Test.lnk"

if (Test-Path "$env:USERPROFILE\AppData\Local\Temp\CreateShortcut.vbs") {
    Remove-Item "$env:USERPROFILE\AppData\Local\Temp\CreateShortcut.vbs" -Force
}

$vbscript = @"
Set oWS = WScript.CreateObject("WScript.Shell")
Set oLink = oWS.CreateShortcut("$shortcutPath")
oLink.TargetPath = "$target"
oLink.IconLocation = "$target, 0"
oLink.Save
"@
$vbscript | Out-File "$env:USERPROFILE\AppData\Local\Temp\CreateShortcut.vbs" -Encoding Unicode
&cscript.exe //nologo "$env:USERPROFILE\AppData\Local\Temp\CreateShortcut.vbs"

if (Test-Path $shortcutPath) {
    Write-Host "  ✅ Collegamento creato: $shortcutPath" -ForegroundColor Green
} else {
    Write-Host "  ⚠️ Collegamento non creato (potrebbe non essere supportato)" -ForegroundColor Yellow
}

# 6. Backup Database Esistente
Write-Host "[6/6] Backup Database..." -ForegroundColor Yellow
$dbPath = "retro_hub.db"
if (Test-Path $dbPath) {
    $backupName = "retro_hub_backup_$(Get-Date -Format 'yyyyMMdd_HHmmss').db"
    Copy-Item $dbPath $backupName
    Write-Host "  ✅ Backup creato: $backupName" -ForegroundColor Green
} else {
    Write-Host "  ⚠️ Nessun database da backup" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🎉 TEST COMPLETATO!" -ForegroundColor Cyan
Write-Host "" -ForegroundColor Cyan
Write-Host "📁 Eseguibile: $target" -ForegroundColor White
Write-Host "📍 Collegamento: $shortcutPath" -ForegroundColor White
Write-Host "" -ForegroundColor Cyan
Write-Host "✅ Puoi ora procedere con il merge se tutti i test sono passati!" -ForegroundColor Green