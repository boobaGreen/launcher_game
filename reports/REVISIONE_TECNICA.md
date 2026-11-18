# 🔍 RetroHub — Report Tecnico di Revisione

**Data:** 17 Novembre 2026  
**Autore:** Cline (AI Coding Agent)  
**Versione:** 1.0.0  

---

## 📊 Executive Summary

RetroHub è un launcher per retrogaming scritto in Rust che gestisce 1.149 giochi su 16 piattaforme. L'architettura è basata su un server HTTP locale che serve un'interfaccia web embedded, con database SQLite per la persistenza.

**Punti di forza:**
- Architettura a singolo binario (Rust + server HTTP + UI HTML)
- Ricerca fuzzy con algoritmo Jaro-Winkler
- Filtri multi-tag avanzati
- Supporto controller Xbox
- Database SQLite per statistiche e preferiti

**Punti critici:**
- Hardcoding estremo di percorsi (non portabile)
- Configurazione assente
- Gestione errori povera
- UI eccessivamente complessa in un unico file
- Documentazione multiplayer non allineata alla realtà

---

## 1. Problemi Critici Architetturali

### 1.1 Hardcoding Estremo di Percorsi Assoluti

Il progetto soffre di una dipendenza massiccia da percorsi hardcoded. Questo rende il progetto **completamente non portabile**.

**File e posizioni critiche:**

| File | Righe | Codice | Problema |
|------|-------|--------|----------|
| `src/main.rs` | 41-43 | `r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"` | Hardcoded percorso Edge |
| `src/main.rs` | 81 | `Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu")` | Percorso catalogo hardcoded |
| `src/launcher.rs` | 16 | `let mame_dir = r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\mame";` | Percorso MAME hardcoded |
| `src/catalog.rs` | 663 | `Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu\covers\c64")` | Percorso copertine C64 hardcoded |
| `src/server.rs` | 81 | `Path::new(r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu")` | Percorso server hardcoded |

**Impatto:** Il progetto funziona solo su quella specifica macchina con quella specifica struttura di cartelle.

### 1.2 Configurazione della Porta Hardcoded

**`src/main.rs`** (linea 68):
```rust
let port = 39281;
```

**Problema:** Nessuna possibilità di configurazione della porta tramite argomenti da riga di comando o variabile d'ambiente.

### 1.3 Dipendenza da Edge Hardcoded

**`src/main.rs`** (linee 40-60):
```rust
let edge_paths = [
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
];
```

**Problema:** Il codice controlla solo Edge. Se l'utente usa Chrome, Firefox o un altro browser, il fallback potrebbe non comportarsi come previsto.

---

## 2. Problemi di Codice

### 2.1 Logica di Copertine Eccessivamente Hardcoded in `catalog.rs`

Il file `catalog.rs` contiene **centinaia di righe** di istruzioni `if/else` per matchare titoli di giochi specifici con i loro percorsi copertina.

**Esempio problematico:**
```rust
else if t_lower.contains("secret of monkey island") { clean = "Secret of Monkey Island, The"; }
else if t_lower.contains("curse of monkey island") { clean = "Curse of Monkey Island, The"; }
else if t_lower.contains("myst") { clean = "Myst"; }
// ... molti altri
```

**Soluzione consigliata:** Estrarre la logica in un file JSON esterno (`covers_mapping.json`).

### 2.2 Assenza di Gestione Errori Robusta

Molti blocchi di codice usano `unwrap()`, `expect()` e `unwrap_or_default()`.

**Esempi critici:**
```rust
// src/catalog.rs
let mame_meta_map: HashMap<String, MameMeta> =
    serde_json::from_str(json_clean).unwrap_or_default();

// src/main.rs
let db = Database::open(db_path).expect("Inizializzazione database fallita");
```

### 2.3 Sync dell'Avvio con Timeout di 6 Secondi

**src/main.rs** (linee 102-111):
```rust
std::thread::spawn(move || {
    for _ in 0..60 {
        if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // ...
});
```

**Problema:** Il thread che apre la finestra aspetta fino a 6 secondi. Meglio usare un canale di notifica.

### 2.4 Heartbeat Server Aggressivo

**src/server.rs** (linee 61-64):
```rust
let last = self.last_ping.load(Ordering::SeqCst);
if last > 0 && now_secs().saturating_sub(last) > 15 {
    break;
}
```

**Problema:** Il server si spegne dopo 15 secondi di inattività.

### 2.5 `launcher.rs` — Lancio con PowerShell

**Problema:** Usa PowerShell con stringhe complesse e escaping dei caratteri:
```rust
let script = format!(
    r#"$p = Start-Process -FilePath '{}' -PassThru; if ($p) {{ Start-Sleep -Milliseconds 400; (New-Object -ComObject WScript.Shell).AppActivate($p.Id) }}"#,
    escaped
);
```

**Rischio:** L'escaping è fragile e l'avvio di PowerShell è lento.

---

## 3. Problemi UX/UI

### 3.1 Interfaccia Eccessivamente Complessa in `ui.html`

Il file `ui.html` è lungo **3995 righe** con CSS e JavaScript inline.

**Problemi:**
- CSS inline eccessivo: tutto in un unico blocco `<style>`
- Manca separazione file CSS/JS
- Scarsa organizzazione del codice
- Nessuna ottimizzazione per il rilascio

### 3.2 Manca Sistema di Configurazione Utente

Non esiste:
- Impostazioni salvate persistentemente
- Opzione per cambiare la porta del server
- Profili utente
- Storico ricerche

### 3.3 Gestione Errori Utente Povera

Molti errori mostrano solo un messaggio in console. L'interfaccia web non notifica l'utente quando:
- Un gioco non si avvia
- Una copertina non è disponibile
- Il server non risponde

---

## 4. Analisi Commenti in Linea

### Commenti Utili e Descrittivi:
- `src/main.rs` (linee 18-36, 63-76): Spiegano bene il Single Instance Check
- `src/launcher.rs` (linee 7-8, 14-16, 32-33): Descrivono il funzionamento dei casi di lancio
- `src/search.rs` (linee 25, 55): Spiegano la tokenizzazione e il parsing

### Commenti Ridondanti:
- `src/db.rs` (linee 92-94): Commento ovvio dal nome della funzione

### Commenti Fuorvianti:
- `src/main.rs` (linea 71): Il commento "Single instance check" è corretto, ma la logica è fragile

### Commenti Assenti:
- `src/models.rs`: Nessun commento sui campi dei struct
- `src/netplay.rs`: Mancano commenti sulla logica di handshake UDP

**Giudizio complessivo sui commenti:** Utili e ben scritti, ma alcuni sono ridondanti.

---

## 5. Documentazione

### Documentazione Completa:
- **README.md**: Ben strutturato, elenca piattaforme e caratteristiche
- **docs/MANUAL.md**: Documentazione completa con installazione, uso, ricerca, controller, multiplayer
- **ROADMAP.md**: Chiara e ben organizzata in 3 fasi

### Punti Deboli:
1. **Hardcoding nei percorsi del manuale:** Fa riferimento a `C:\Users\ClaudioDall'Ara\Desktop\Clod\emu`
2. **Mancanza sezione "Contribuire":** Nessuna guida su how to contribuire
3. **Mancanza guida debug:** Nessun How to run tests, come abilitare i log
4. **Incoerenza versioni:** Cargo.toml dice 0.1.0, manuale dice 1.0.0

---

## 6. Sezione Online/Netplay

### Incongruenze Trovate

#### 6.1 Inconsistenza Manuale vs Roadmap

| Aspetto | Manuale | Roadmap | Allineamento |
|---------|---------|---------|--------------|
| Descrizione Host | Dettagliata | Non completato `[ ]` | ❌ Inesatto |
| Descrizione Guest | Dettagliata | Non completato `[ ]` | ❌ Inesatto |

#### 6.2 Implementazione vs Documentazione

| Funzionalità | Documentazione | Codice Reale | Allineamento |
|-------------|---------------|--------------|--------------|
| Creazione stanza | ✅ | ✅ | ✅ |
| Unione tramite token | ✅ | ✅ | ✅ |
| Avvio sincronizzato | ✅ | ✅ | ✅ |
| Ping reale | ❌ "stimato" | ❌ Hardcoded 20ms | ❌ |
| IP pubblico | ❌ Implicito | ❌ Solo locale | ❌ |
| NAT Traversal | ❌ Non menzionato | ❌ Non implementato | ✅ |
| Sicurezza token | ❌ Non menzionato | ❌ In chiaro | ⚠️ |

#### 6.3 Codice Critico

**`src/netplay.rs`** (linee 51, 222-225):
```rust
let public_ip = resolve_public_ip().unwrap_or_else(|| get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string()));
fn resolve_public_ip() -> Option<String> {
    get_local_ip()  // ⚠️ Restituisce solo IP locale!
}
```

**`src/netplay.rs`** (linea 166):
```rust
ping_ms: Some(20),  // ⚠️ Hardcoded, non è un ping reale
```

---

## 7. Raccomandazioni

### Implementazione Immediate:
1. **Sostituire i path hardcoded con configurazione:**
   - Argomenti da riga di comando: `--emu-path`, `--port`, `--browser`
   - File di configurazione JSON/YAML
   - Variabili d'ambiente come fallback

2. **Estrarre la logica di copertine in un file JSON esterno**

3. **Migliorare la gestione errori:**
   - Aggiungere `Result` propagazione
   - Aggiungere log strutturati

### UX/UI:
4. **Separare CSS e JavaScript in file esterni**

5. **Aggiungere sistema di impostazioni salvate**

6. **Migliorare notifica errori utente (toast/banner)**

### Documentazione:
7. **Rimuovere path hardcoded dalla documentazione**

8. **Aggiungere guida alla contribuzione**

9. **Allineare le versioni (Cargo.toml ↔ Manuale)**

### Netplay:
10. **Implementare risoluzione IP pubblica reale (STUN)**

11. **Implementare ping reale (misurazione RTT)**

12. **Aggiungere NAT traversal (STUN/TURN)**

---

## 📊 Metriche

| Categoria | Punteggio | Note |
|-----------|-----------|------|
| Architettura | 5/10 | Hardcoding estremo |
| Codice | 7/10 | Funzionale ma fragili |
| UX/UI | 6/10 | Buona idea, male implementata |
| Documentazione | 7/10 | Buona ma inaccurata |
| Configurabilità | 3/10 | Nessuna |
| Manutenibilità | 5/10 | Difficile da manutenere |

---

## 📝 Conclusione

RetroHub è un progetto **funzionale ma estremamente fragile e non portabile**. La mancanza di configurazione e l'eccesso di hardcoded paths lo rende difficile da usare fuori dall'ambiente di sviluppo.

**Punteggio complessivo: 5.4/10**

---

*Report generato da Cline AI Coding Agent - 17 Novembre 2026*