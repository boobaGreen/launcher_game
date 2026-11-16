use std::path::Path;

fn main() {
    println!("==================================================");
    println!("🕹️  RetroHub Core Engine (Rust) v0.1.0");
    println!("    Collezione Unificata Retrogaming & Recomp");
    println!("==================================================");

    let emu_path = r"C:\Users\ClaudioDall'Ara\Desktop\Clod\emu";
    if Path::new(emu_path).exists() {
        println!("✔ Cartella emulazione rilevata: {}", emu_path);
    } else {
        println!("⚠ Cartella emulazione non trovata: {}", emu_path);
    }

    println!("\nPronto per l'indicizzazione dei metadati e il server UI.");
}
