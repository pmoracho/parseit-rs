// src/main.rs
mod config;
mod parse;

use clap::Parser;
use std::error::Error;
use config::ConfigSchema; 
use parse::process_file;

// --- 1. Definición de Argumentos de Línea de Comandos (usando clap) ---

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ruta al archivo de datos de longitud fija a procesar.
    #[arg(short, long)]
    data_file: String,

    /// Nombre del formato a usar de 'parseit.toon' (ej: "sample").
    #[arg(short, long, default_value = "sample")]
    format_name: String,
}

// --------------------------------------------------------------------------------------------------------
// --- Función Principal ---
// --------------------------------------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();

    // Cargar la configuración
    const CONFIG_FILE: &str = "parseit.toon";
    let schema: ConfigSchema = match config::load_config(CONFIG_FILE) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error al cargar el esquema de configuración ({}): {}", CONFIG_FILE, e);
            return Err(e);
        }
    };

    // Obtener el formato específico
    let format_def = schema.formats.get(&args.format_name)
        .ok_or_else(|| format!("El formato '{}' no se encontró en {}", args.format_name, CONFIG_FILE))?;
    
    // Procesar el archivo usando la definición de campos
    println!("Procesando archivo de datos: {}...", args.data_file);
    println!("Usando el formato: '{}'", args.format_name);

    // Llama a la función de procesamiento
    process_file(&args.data_file, &format_def.fields, &schema)?;
    
    eprintln!("\nProceso finalizado. Salida CSV escrita a la salida estándar.");

    Ok(())
}