//! Módulo de entrada/salida para parseit-rs.
//! Proporciona funciones para escribir la salida en diferentes formatos (CSV, terminal interactivo).
//! También incluye utilidades para leer archivos de datos, como obtener la longitud de la primera línea
//! de un archivo de longitud fija.
//! 
use std::{error::Error, fs::File, io::{BufRead, BufReader}};

use encoding_rs::WINDOWS_1252;
use tempfile::NamedTempFile;
use csvlens::{run_csvlens_with_options, CsvlensOptions};

use std::io::{self, Write};

/// Escribe los registros procesados a la salida estándar en el formato especificado.
/// 
/// ## Argumentos
/// - `output_typr`: Tipo de salida ("csv" o "term").
/// - `headers`: Encabezados de las columnas.
/// - `records`: Registros de datos.
/// - `delim_character`: Carácter delimitador para CSV.
/// 
/// ## Retorno
/// `Result<(), Box<dyn Error>>` - Ok si la operación es exitosa, o un error en caso contrario.
/// 
/// ## Errores
/// Retorna un error si falla la escritura en la salida estándar o si el tipo de salida
/// no es reconocido.
/// 
/// ## Ejemplo
/// ```
/// write_output("csv", headers, records, ",")?;
/// ```
pub fn write_output(
    output_typr: &str,
    headers: Vec<String>,
    records: Vec<Vec<String>>,
    delim_character: &str,
    ) -> Result<(), Box<dyn Error>> {
    match output_typr {
        "csv" => write_csv_output(headers, records, delim_character),
        // 💡 Llama a la nueva write_interactive, que ya no necesita el argumento 'fields'
        "term" => write_interactive(headers, records),
        _ => Err(format!("Tipo de salida desconocido: {}", output_typr).into()),
    }
}

/// Escribe los registros procesados a la salida estándar en formato CSV o Long Format.
/// 
/// ## Argumentos
/// - `headers`: Encabezados de las columnas.
/// - `records`: Registros de datos.
/// - `delim_character`: Carácter delimitador para CSV.
/// 
/// ## Retorno
/// `Result<(), Box<dyn Error>>` - Ok si la operación es exitosa, o un error en caso contrario.
/// 
/// ## Errores
/// Retorna un error si falla la escritura en la salida estándar.
/// 
/// ## Ejemplo
/// ```
/// write_csv_output(headers, records, ",")?;
/// ```
pub fn write_csv_output(
    headers: Vec<String>,
    records: Vec<Vec<String>>,
    delim_character: &str,  
    ) -> Result<(), Box<dyn Error>> {
    
    let mut output = io::stdout().lock();
    
    writeln!(output, "{}", headers.join(delim_character))?;
    
    for record in records.iter() {

        let escaped_record: Vec<String> = record.iter()
            .map(|v| format!("\"{}\"", v.replace('"', "\"\"")))
            .collect();

        writeln!(output, "{}", escaped_record.join(delim_character))?;
    }

    Ok(())
}

/// Escribe los registros procesados en un archivo temporal y abre csvlens para selección interactiva.
/// 
/// ## Argumentos
/// - `headers`: Encabezados de las columnas.
/// - `records`: Registros de datos.
/// 
/// ## Retorno
/// `Result<(), Box<dyn Error>>` - Ok si la operación es exitosa, o un error en caso contrario.
/// 
/// ## Errores
/// Retorna un error si falla la creación del archivo temporal, la escritura de datos,
/// o la ejecución de csvlens.
/// 
/// ## Ejemplo
/// ```
/// write_interactive(headers, records)?;
/// ```
pub fn write_interactive(
    headers: Vec<String>,
    records: Vec<Vec<String>>,
    ) -> Result<(), Box<dyn Error>> {
    
    // 1. Crear un archivo temporal. Se borra automáticamente cuando 'temp_file' sale del scope.
    let temp_file = NamedTempFile::new()?;
    let file_path = temp_file.path().to_string_lossy().to_string();
    let mut file = temp_file.reopen()?; 
    
    // Usamos '|' como delimitador para la compatibilidad con csvlens
    const DELIMITER: &str = "|"; 
    
    // 2. Escribir Encabezado y Registros en el archivo temporal
    writeln!(file, "{}", headers.join(DELIMITER))?;
    
    for record in records.iter() {
        // Escapamos las comillas internas (doble comilla) y envolvemos el valor con comillas
        let escaped_record: Vec<String> = record.iter()
            .map(|v| format!("\"{}\"", v.replace('"', "\"\"")))
            .collect();
            
        writeln!(file, "{}", escaped_record.join(DELIMITER))?;
    }
    
    file.flush()?; 
    
    let options = CsvlensOptions {
        filename: Some(file_path), 
        delimiter: Some(DELIMITER.to_string()),
        ignore_case: true,
        debug: false, 
        ..Default::default()
    };
    
    let result = run_csvlens_with_options(options);

    // 4. Manejar la salida (selección o error)
    match result {
        Ok(Some(selected_cell)) => {
            println!("Celda seleccionada por el usuario: {}", selected_cell);
        }
        Ok(None) => {
            // Usuario salió sin seleccionar
        }
        Err(e) => {
            eprintln!("Error al abrir el archivo {}", e);
        }
    }
    
    Ok(())
}


/// Lee la primera línea del archivo de datos y devuelve su longitud.
/// 
/// ## Argumentos
/// - `file_path`: Ruta al archivo de datos.
/// 
/// ## Retorno
/// `Result<usize, Box<dyn Error>>` - Longitud de la primera línea o error.
/// 
/// ## Errores
/// Retorna un error si no se puede abrir o leer el archivo.
///  
/// ## Ejemplo
/// ```
/// let length = get_first_line_length("data.txt")?;
/// println!("La longitud de la primera línea es: {}", length);
/// ```
pub fn get_first_line_length(file_path: &str) -> Result<usize, Box<dyn Error>> {

    let file = File::open(file_path)?;
    
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_until(b'\n', &mut buffer)?;
    
    let (cow, _, _) = WINDOWS_1252.decode(&buffer);
    let line = cow.to_string(); 
    Ok(line.trim_end().len()) 
}