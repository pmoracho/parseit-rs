// src/main.rs
mod config;
mod parse;

use clap::Parser;
use std::error::Error;
use config::ConfigSchema; 
use parse::process_file;

use crate::parse::deduce_format;

// --- 1. Definición de Argumentos de Línea de Comandos (usando clap) ---

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// Ruta al archivo de datos de longitud fija a procesar.
    #[arg(short, long)]
    data_file: String,

    /// Nombre del formato a usar de 'parseit.toon' (ej: "sample").
    #[arg(short, long)]
    format_name: Option<String>,
    
    /// Delimitador para la salida CSV (por defecto es ',').
    #[arg(long, default_value = ",")]
    delim_character: String,
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

    let actual_format_name = if let Some(name) = args.format_name {
        name
    } else {
        eprintln!("Formato no especificado. Intentando deducir por tamaño de registro...");
        deduce_format(&args.data_file, &schema.formats)?
    };

    // Obtener el formato específico
    let format_def = schema.formats.get(&actual_format_name)
        .ok_or_else(|| format!("El formato '{}' no se encontró en {}", actual_format_name, CONFIG_FILE))?;

    process_file(&args.data_file, &format_def.fields, &schema, &args.delim_character)?;
    
    Ok(())
}