//! Módulo de entrada/salida para parseit-rs.
//! Proporciona funciones para escribir la salida en diferentes formatos (CSV, terminal interactivo).
//! También incluye utilidades para leer archivos de datos, como obtener la longitud de la primera línea
//! de un archivo de longitud fija.
//! 
use std::{error::Error, fs::File, io::{BufRead, BufReader}};

use encoding_rs::WINDOWS_1252;
use tempfile::NamedTempFile;
use csvlens::{run_csvlens_with_options, CsvlensOptions};
use prettytable::{Cell, Row, Table, format};

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
        "term" => write_interactive(headers, records),
        "sql" => write_sql_output(headers, records),
        "txt" => write_txt_output(headers, records),
        "html" => write_html_output(headers, records),
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
/// Escribe un script SQL a la salida estándar, incluyendo la sentencia CREATE TABLE
/// y las sentencias INSERT correspondientes a los registros.
/// 
/// ## Argumentos
/// - `headers`: Encabezados de las columnas (usados como nombres de columna SQL).
/// - `records`: Registros de datos (usados como valores a insertar).
/// 
/// ## Retorno
/// `Result<(), Box<dyn Error>>` - Ok si la operación es exitosa, o un error en caso contrario.
/// 
/// ## Errores
/// Retorna un error si falla la escritura en la salida estándar.
/// 
/// ## Ejemplo
/// ```ignore
/// // La tabla se llamará 'processed_data' por defecto.
/// write_sql_output(headers, records)?;
/// ```
pub fn write_sql_output(
    headers: Vec<String>,
    records: Vec<Vec<String>>,
    ) -> Result<(), Box<dyn Error>> {
    
    let mut output = io::stdout().lock();
    const TABLE_NAME: &str = "processed_data";
    
    // Función auxiliar para limpiar nombres de columna (reemplazar caracteres especiales)
    let clean_headers: Vec<String> = headers.iter()
        .map(|h| h.replace(' ', "_").to_uppercase())
        .collect();

    // 1. Sentencia CREATE TABLE
    writeln!(output, "--------------------------------------------------------")?;
    writeln!(output, "-- DDL: Creación de tabla '{}'", TABLE_NAME)?;
    writeln!(output, "--------------------------------------------------------")?;
    writeln!(output, "DROP TABLE IF EXISTS {};", TABLE_NAME)?;
    write!(output, "CREATE TABLE {} (\n", TABLE_NAME)?;
    
    let mut column_definitions = Vec::new();
    // Asumimos que todos los campos serán VARCHAR o TEXT para simplificar y asegurar la compatibilidad.
    for (i, header) in clean_headers.iter().enumerate() {
        let definition = if i < clean_headers.len() - 1 {
            format!("    {} VARCHAR(255) NULL,", header)
        } else {
            format!("    {} VARCHAR(255) NULL", header) // El último no lleva coma
        };
        column_definitions.push(definition);
    }
    
    writeln!(output, "{}", column_definitions.join("\n"))?;
    writeln!(output, ");\n")?;

    // 2. Sentencias INSERT
    writeln!(output, "--------------------------------------------------------")?;
    writeln!(output, "-- DML: Inserción de {} registros", records.len())?;
    writeln!(output, "--------------------------------------------------------")?;

    for record in records.iter() {
        // Escapamos las comillas internas (doble comilla) y envolvemos el valor con comillas simples para SQL
        let escaped_values: Vec<String> = record.iter()
            .map(|v| {
                // Reemplazamos ' con '' (escape estándar SQL) y envolvemos en comillas simples
                format!("'{}'", v.replace('\'', "''"))
            })
            .collect();

        writeln!(output, "INSERT INTO {} ({}) VALUES ({});", 
            TABLE_NAME, 
            clean_headers.join(", "),
            escaped_values.join(", ")
        )?;
    }

    Ok(())
}

/// Genera y escribe en stdout una tabla de texto formateada usando prettytable-rs.
///
/// Convierte los encabezados y registros proporcionados en un formato de tabla
/// que es legible en la consola.
///
/// # Argumentos
/// * `headers`: Un vector de Strings para los encabezados de las columnas.
/// * `records`: Un vector de vectores de Strings, donde cada vector interno es una fila de datos.
///
/// # Retorno
/// `Result<(), Box<dyn Error>>`: Retorna Ok(()) en caso de éxito o un Error.
pub fn write_txt_output(
    headers: Vec<String>,
    records: Vec<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    
    let mut table = Table::new();
    
    table.set_format(*format::consts::FORMAT_DEFAULT);
    let header_cells: Vec<Cell> = headers.into_iter()
        .map(|h| Cell::new(&h).style_spec("b")) // 'b' para negrilla (bold)
        .collect();
        
    table.add_row(Row::new(header_cells));

    for record in records {
        let data_cells: Vec<Cell> = record.into_iter()
            .map(|v| Cell::new(&v))
            .collect();
            
        table.add_row(Row::new(data_cells));
    }
    table.printstd();
    
    Ok(())
}

/// Genera y escribe en stdout un documento HTML con una tabla de resultados.
///
/// Convierte los encabezados y registros proporcionados en la estructura
/// <table>, <tr>, <th>, y <td> de HTML.
///
/// # Argumentos
/// * `headers`: Un vector de Strings para los encabezados de las columnas.
/// * `records`: Un vector de vectores de Strings, donde cada vector interno es una fila de datos.
///
/// # Retorno
/// `Result<(), Box<dyn Error>>`: Retorna Ok(()) en caso de éxito o un Error.
pub fn write_html_output(
    headers: Vec<String>,
    records: Vec<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    
    let mut output = io::stdout().lock();

    // 1. Escribir el encabezado del documento HTML
    writeln!(output, "<!DOCTYPE html>")?;
    writeln!(output, "<html>")?;
    writeln!(output, "<head>")?;
    writeln!(output, "  <meta charset=\"UTF-8\">")?;
    writeln!(output, "  <title>Resultados de la Tabla</title>")?;
    
    // ⭐ Modificación del Estilo ⭐
    writeln!(output, "  <style>")?;
    // Base de la tabla

    let css = "

    ";

    writeln!(output, "    table {{ border-collapse: break-word; margin: 20px; table-layout: auto; width: auto; }}")?; // Ajusta el ancho de la tabla y celdas
    writeln!(output, "    th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}")?;
    
    // Encabezados (th)
    writeln!(output, "    th {{ background-color: #f2f2f2; font-weight: bold; }}")?;
    
    // Estilo de rayas (Striping) para filas alternas (<tbody> tr:nth-child(even))
    // nth-child(even) selecciona las filas pares (2da, 4ta, etc.)
    writeln!(output, "    tbody tr:nth-child(even) {{ background-color: #e8f5e9; }}")?; // Color para filas pares
    // nth-child(odd) selecciona las filas impares (1ra, 3ra, etc.)
    writeln!(output, "    tbody tr:nth-child(odd) {{ background-color: #ffffff; }}")?; // Color para filas impares (blanco, para contraste)
    
    writeln!(output, "  </style>")?;
    // Fin de estilos
    
    writeln!(output, "</head>")?;
    writeln!(output, "<body>")?;
    writeln!(output, "  <h1>Resultados Procesados</h1>")?;
    writeln!(output, "  <table>")?;

    // 2. Generar los encabezados (<thead> / <tr> / <th>)
    writeln!(output, "    <thead>")?;
    write!(output, "      <tr>")?;
    for header in headers {
        write!(output, "<th>{}</th>", header)?;
    }
    writeln!(output, "</tr>")?;
    writeln!(output, "    </thead>")?;

    // 3. Generar el cuerpo de la tabla (<tbody> / <tr> / <td>)
    writeln!(output, "    <tbody>")?;
    for record in records {
        write!(output, "      <tr>")?;
        for value in record {
            write!(output, "<td>{}</td>", value)?;
        }
        writeln!(output, "</tr>")?;
    }
    writeln!(output, "    </tbody>")?;

    // 4. Cerrar las etiquetas
    writeln!(output, "  </table>")?;
    writeln!(output, "</body>")?;
    writeln!(output, "</html>")?;

    Ok(())
}