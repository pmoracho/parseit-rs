//! parseit-rs: Herramienta para interpretar archivos de longitud fija del ARCA.
//! (entre otros formatos).
//! Proporciona funcionalidades para cargar configuraciones desde archivos TOML,
//! deducir formatos automáticamente, parsear archivos de datos y generar salidas en varios formatos
//! (CSV, terminal interactivo).
//!  
mod config;
mod parse;
mod io;

use clap::Parser;
use std::error::Error;
use prettytable::{Table, format, row};
use crate::parse::{deduce_format, parse_to_records};
use crate::io::{write_output};
use crate::config::{CONFIG_FILE, ConfigSchema, FormatDefinition, calculate_format_length};

// Estructura de ayuda para almacenar y ordenar los datos
struct FormatData<'a> {
    category: String,
    name: &'a String,
    count: usize,
    total_len: usize,
}

const PROGRAM_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const BANNER: &str = const_format::formatcp!(r#"

╔═╗╔═╗╦═╗╔═╗╔═╗╦╔╦╗
╠═╝╠═╣╠╦╝╚═╗║╣ ║ ║ 
╩  ╩ ╩╩╚═╚═╝╚═╝╩ ╩
by {}"#, PROGRAM_AUTHORS);

// --- 1. Definición de Argumentos de Línea de Comandos (usando clap) ---
#[derive(Parser, Debug)]
#[command(
    author,
    version, 
    about = "Herramienta para inerpretar archivos de longitud fija del ARCA.", 
    long_about = r#"Esta utilidad fue diseñada para procesar registros de longitud fija 
basados en esquemas definidos en archivos TOON, permitiendo formatos de 
salida variados."#,
    before_help = BANNER,
)]
struct Args {
    /// Ruta al archivo de datos de longitud fija a procesar.
    #[arg(short, long, default_value = "")]
    data_file: String,

    /// Nombre del formato a usar de 'parseit.toon' (ej: "sample").
    #[arg(short, long)]
    format_name: Option<String>,
    
    /// Delimitador para la salida CSV (por defecto es ',').
    #[arg(long, short='c', default_value = ",")]
    delim_character: String,

    /// Output type: Ejemplo: csv
    #[arg(long, short='o', default_value = "csv")]
    output_type: String,

    /// Genera la salida en formato largo (transpuesto): NumeroFila, NombreColumna, Valor
    #[arg(long, short='l', default_value_t = false)]
    long_format: bool, 

    /// Formato numérico para montos (ej: "1,234.56" o "1.234,56").
    #[arg(long, short='n', default_value_t = false)]
    format_numeric: bool,

    /// Evita el uso de tablas de lookup (como sifere-jurisdicciones) y devuelve el valor crudo.
    #[arg(long, short='t', default_value_t = false)]
    dont_use_tables: bool,

    #[arg(short = 's', long, default_value_t = false)] 
    show_formats: bool,
}

/// Función auxiliar para mostrar los formatos usando prettytable y ordenando por categoría/nombre
/// 
/// ## Argumentos
/// - `formats`: Mapa de nombres de formatos a sus definiciones.
/// 
/// ## Retorno
/// Nada. Imprime la tabla directamente en la salida estándar.
/// 
/// ## Errores
/// No retorna errores.
///
/// ## Ejemplo
/// ```
/// display_available_formats(&schema.formats);
/// ```
fn display_available_formats(formats: &std::collections::HashMap<String, FormatDefinition>) {
    let mut table = Table::new();
    
    // 1. Definir los encabezados
    table.add_row(row![bFg->"CATEGORÍA", bFg->"NOMBRE DEL FORMATO", bFg->"Nº DE CAMPOS", bFg->"LONGITUD TOTAL"]); 
    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    // 2. Pre-procesar los datos y construir el vector de FormatData
    let mut processed_data: Vec<FormatData> = formats.iter()
        .map(|(name, definition)| {
            let category = definition.category.clone();
            FormatData {
                category,
                name,
                count: definition.fields.len(),
                total_len: calculate_format_length(&definition.fields), 
            }
        })
        .collect();

    // 3. ORDENACIÓN DOBLE: Primero por categoría, luego por nombre
    processed_data.sort_by(|a, b| {
        // Ordenar por categoría (String)
        a.category.cmp(&b.category)
            // Si las categorías son iguales, ordenar por nombre de formato (String)
            .then_with(|| a.name.cmp(&b.name))
    });

    // 4. Llenar la tabla
    for data in processed_data {
        table.add_row(row![
            data.category, 
            data.name, 
            data.count, 
            data.total_len
        ]);
    }

    println!("\n▶️ Formatos disponibles en 'parseit.toon':\n");
    table.printstd();
}


// --------------------------------------------------------------------------------------------------------
// --- Función Principal ---
// --------------------------------------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();

    // Cargar la configuración
    let schema: ConfigSchema = match config::load_config_from_paths() {
        Ok(s) => s,
        Err(e) => {
            return Err(e);
        }
    };

    // --- LÓGICA DE MOSTRAR FORMATOS Y SALIR ---
    if args.show_formats {
        display_available_formats(&schema.formats);
        return Ok(()); // Salir del programa inmediatamente
    }
    // ----------------------------------------

    if args.data_file.is_empty() {
        return Err("Error: Debe proporcionar la ruta al archivo de datos usando --data-file o -d.".into());
    }    

    let actual_format_name = if let Some(name) = args.format_name {
        name
    } else {
        deduce_format(&args.data_file, &schema.formats)?
    };

    // Obtener el formato específico
    let format_def = schema.formats.get(&actual_format_name)
        .ok_or_else(|| format!("El formato '{}' no se encontró en {}", actual_format_name, CONFIG_FILE))?;


    let (headers, records) = parse_to_records(
        &args.data_file,
        &format_def.fields, // campos del formato
        &schema,            // tablas de lookup
        args.format_numeric,
        args.dont_use_tables,
        args.long_format,
    )?;    

    write_output(
        &args.output_type,
        headers,
        records,
        &args.delim_character
    )?;    
    
    Ok(())
}
