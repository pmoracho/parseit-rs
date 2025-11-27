//! Módulo de análisis para parseit-rs.
//! Proporciona funciones para deducir el formato de un archivo de longitud fija
//! y para parsear los datos aplicando lookups y formateo numérico.
//! 
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::io::{BufReader, BufRead};
use encoding_rs::WINDOWS_1252; // O usa ISO_8859_1
use crate::config::{ConfigSchema, FieldDefinition, FormatDefinition, calculate_format_length};
use crate::io::get_first_line_length;

/// Formatea una cadena numérica de entrada basada en el tipo de campo y las opciones de salida.
/// ## Argumentos
/// - `raw_value`: Valor crudo extraído del archivo de datos.
/// - `field_type`: Tipo de dato (ej: "zamount", "amount", "numeric").
/// - `format_numeric`: Indica si se debe aplicar formateo numérico con separadores.
/// - `decimal_places`: Cantidad de decimales implícitos/deseados.
/// 
/// ## Retorno
/// String - Retorna la cadena formateada según las reglas especificadas.
/// 
/// ## Errores
/// No retorna errores, pero si la conversión falla, devuelve el valor crudo.
/// 
/// ## Ejemplo
/// ```
/// let formatted = format_field_value("00012345", "zamount", true, 2);
/// assert_eq!(formatted, "123,45");
/// ```
fn format_field_value(
    raw_value: &str,
    field_type: &str, // Ej: "zamount", "amount", "numeric"
    format_numeric: bool, // Reformatear con separadores S/N
    decimal_places: usize, // Cantidad de decimales implícitos/deseados
    ) -> String {
    let raw_trimmed = raw_value.trim();

    if raw_trimmed.is_empty() {
        return if decimal_places > 0 { "0,00".to_string() } else { "0".to_string() };
    }

    let mut number_string_for_decimal: String;
    let mut final_decimal_places = decimal_places;
    let field_type_lower = field_type.to_lowercase();
    
    // --- FASE 1: CONVERSIÓN A CADENA ESTÁNDAR (Punto decimal '.') ---
    match field_type_lower.as_str() {
        
        "zamount" => {
            let num_str = raw_trimmed; 
            let len = num_str.len();

            // 1. Asegurarse de que el número sea lo suficientemente largo para tener una parte entera.
            // Si el número es más corto que los decimales, rellenamos con ceros a la izquierda.
            let integer_part = if len < final_decimal_places {
                // Rellenamos hasta que la parte entera tenga al menos un dígito '0'
                let padding = "0".repeat(final_decimal_places - len + 1);
                format!("{}{}", padding, num_str)
            } else {
                num_str.to_string()
            };

            let len = integer_part.len();
            
            // 2. Calcular el índice del punto decimal.
            // Aquí len siempre será >= final_decimal_places, por lo que checked_sub no será None.
            let index_of_dot = len.checked_sub(final_decimal_places).unwrap_or(0);

            let int_part = &integer_part[0..index_of_dot];
            let dec_part = &integer_part[index_of_dot..];
            
            // 3. Crear la cadena final: eliminamos los ceros a la izquierda de la parte entera
            let final_int_part = int_part.trim_start_matches('0');
            
            // Si la parte entera después de trim está vacía (ej: 000.12), usamos "0"
            let final_int_part = if final_int_part.is_empty() {
                "0"
            } else {
                final_int_part
            };

            number_string_for_decimal = format!("{}.{}", final_int_part, dec_part);
        },
        
        "amount" | "numeric" => {
            // Eliminar separadores de miles y convertir el último separador a punto decimal
            number_string_for_decimal = raw_trimmed
                .replace('.', "")
                .replace(',', ".");
            
            // Ajustar decimales si no hay punto
            if !number_string_for_decimal.contains('.') && final_decimal_places > 0 {
                 number_string_for_decimal.push_str(&format!(".{:0<width$}", "", width = final_decimal_places));
            }
            
            // Si el tipo es solo "numeric" y decimal_places es 0, mantener la escala en 0.
            if field_type_lower == "numeric" && final_decimal_places == 0 {
                // No hay cambio en final_decimal_places
            } else if field_type_lower == "amount" && final_decimal_places == 0 {
                 final_decimal_places = 2; // Estándar de 2 para montos si no se especificó
            }
        }
        _ => return raw_trimmed.to_string(), // Si no es numérico, retornar el valor crudo
    }
    
    // --- FASE 2: CONVERSIÓN, ESCALA Y FORMATEO DE SALIDA ---
    let mut number = match Decimal::from_str(&number_string_for_decimal) {
        Ok(d) => d,
        Err(_) => return raw_value.to_string(),
    };

    // Ajustar la escala
    number.set_scale(final_decimal_places as u32).expect("Fallo al configurar la escala.");

    if !format_numeric {
        // Devolver formato estándar (punto decimal)
        return number.to_string().replace('.', ",");
    }

    // --- REFORMATEO (Punto para miles, Coma para decimal) ---

    let number_string = number.to_string();
    let parts: Vec<&str> = number_string.split('.').collect();
    
    let integer_part = parts[0];
    let decimal_part = parts.get(1).map_or("00", |v| v); 

    // 4. Aplicar Separador de Miles (Punto '.')
    let mut final_integer = String::new();
    let mut count = 0;
    let mut is_negative = false;

    for char in integer_part.chars().rev() {
        if char == '-' {
            is_negative = true;
            continue; // Saltar el signo en el bucle de formateo
        }
        if count > 0 && count % 3 == 0 {
            final_integer.push('.'); 
        }
        final_integer.push(char);
        count += 1;
    }
    
    let mut formatted_integer_part: String = final_integer.chars().rev().collect();
    
    if is_negative {
        // Añadir el signo negativo al principio
        formatted_integer_part.insert(0, '-');
    }
    
    // 5. Ensamblar el resultado final (ej: 1.234.567,89)
    format!("{},{}", formatted_integer_part, decimal_part)
}


/// Procesa el archivo de datos de longitud fija, aplica lookups y formateo,
/// y devuelve un vector de registros listos para imprimir.
/// 
/// ## Argumentos
/// - `file_path`: Ruta al archivo de datos.
/// - `fields`: Definiciones de campos del formato seleccionado.
/// - `schema`: Esquema de configuración cargado.
/// - `format_numeric`: Indica si se debe aplicar formateo numérico con separadores.
/// - `dont_use_tables`: Indica si se deben evitar las tablas de lookup.
/// - `long_format`: Indica si se debe devolver la salida en formato largo.
/// 
/// ## Retorno
/// `Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>>` -
/// Tupla con encabezados y registros procesados, o un error.
/// 
/// ## Errores
/// Retorna un error si no se puede abrir o leer el archivo.
/// 
/// ## Ejemplo
/// ```
/// let (headers, records) = parse_to_records("data.dat", &fields, &schema, true, false, false)?;
/// ``` 
pub fn parse_to_records(file_path: &str, 
                        fields: &[FieldDefinition],
                        schema: &ConfigSchema,
                        format_numeric: bool,
                        dont_use_tables: bool,
                        long_format: bool,
                    ) -> Result<(Vec<String>, Vec<Vec<String>>), Box<dyn Error>> {
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // 1. Obtener encabezados
    let headers: Vec<String> = fields.iter().map(|f| f.nombre.clone()).collect();
    let mut records: Vec<Vec<String>> = Vec::new();

    // 2. Iterar por las lineas del archivo
    for line_result in reader.split(b'\n') {
        
        let buffer = line_result?;
        
        let (cow, _, _) = WINDOWS_1252.decode(&buffer);
        let line = cow.to_string(); // Convertir a String propia

        let mut start_pos = 0;
        let mut record_parts = Vec::new();

        // 3. Procesamos cada columna
        for field in fields.iter() {
            let end_pos = start_pos + field.len;

            // Asegurarse de no exceder la longitud de la línea
            if end_pos > line.len() {
                eprintln!("Advertencia: Línea demasiado corta. Campo '{}' incompleto.", field.nombre);
                record_parts.push("".to_string());
                break;
            }

            let raw_value = line[start_pos..end_pos].trim().to_string();
            let mut final_value = raw_value.clone();

            // ******* Lógica de Lookup (Tablas) *******
            let should_lookup = !dont_use_tables;
            if field.tipo == "table" && should_lookup {
                let table_name = &field.param1; 
                if let Some(table) = schema.tables.get(table_name) {
                    if let Some(lookup_value) = table.get(&raw_value) {
                        // Concatenar valor crudo y descripción
                        final_value = format!("{raw_value} - {lookup_value}");
                    }
                }
            }

            // ***************************************** // Aplicar formateo numérico si es necesario
            if field.tipo == "zamount" || field.tipo == "amount" {
                final_value = format_field_value(&final_value, 
                                                &field.tipo, 
                                                format_numeric, 
                                                field.param1.parse::<usize>().unwrap_or(2) // Decimales
                ); 
            }
            
            // 4. Almacenar el valor final
            record_parts.push(final_value);
            start_pos = end_pos;
        }

        records.push(record_parts);
    }

    // Si se solicita formato largo, aplanamos los registros aquí y devolvemos
    // encabezado y registros ya listos para escribir (cada fila tendrá
    // tres columnas: número de fila, nombre de columna y valor).
    if long_format {
        let flat_headers = vec!["#".to_string(), "Columna".to_string(), "Valor".to_string()];
        let mut flat_records: Vec<Vec<String>> = Vec::new();

        for (row_index, record) in records.iter().enumerate() {
            let row_num = (row_index + 1).to_string();
            for (col_index, value) in record.iter().enumerate() {
                let col_name = headers.get(col_index).cloned().unwrap_or_else(|| format!("col_{}", col_index + 1));
                flat_records.push(vec![row_num.clone(), col_name, value.clone()]);
            }
        }

        return Ok((flat_headers, flat_records));
    }

    Ok((headers, records))
}


/// Intenta identificar el formato de un archivo de datos comparando la longitud 
/// de su primer registro con las longitudes predefinidas en el esquema de configuración.
///
/// Este proceso es crucial para determinar qué conjunto de reglas de análisis (schema)
/// debe aplicarse al archivo de longitud fija.
///
/// ## Argumentos
///
/// * `file_path`: La ruta al archivo de datos de longitud fija que se va a analizar.
/// * `formats`: Un mapa de todas las definiciones de formato disponibles (`FormatDefinition`) 
///              extraídas del archivo de configuración.
///
/// ## Retorno
/// `Result<String, Box<dyn Error>>`.
/// * **`Ok(String)`**: Contiene el nombre del formato cuya longitud de registro coincide.
/// * **`Err(Box<dyn Error>)`**: Si no se encuentra ninguna coincidencia o si hay un error de lectura del archivo.
///
/// ## Errores
///
/// Retorna un error si:
/// * No se puede abrir o leer la primera línea del archivo (`file_path`).
/// * Ninguna `FormatDefinition` en `formats` coincide con la longitud del primer registro.
///
/// ## Ejemplo
///
/// ```ignore
/// // Asumiendo que 'config_schema' ya está cargado y 'file_path' es válido.
/// let formats = &config_schema.formats;
/// match deduce_format("data.dat", formats) {
///     Ok(name) => println!("Formato deducido: {}", name),
///     Err(e) => eprintln!("Fallo al deducir el formato: {}", e),
/// }
/// ```
pub fn deduce_format(
    file_path: &str, 
    formats: &HashMap<String, FormatDefinition>
    ) -> Result<String, Box<dyn Error>> {
    let data_len = get_first_line_length(file_path)?;
    
    for (name, definition) in formats.iter() {
        let format_len = calculate_format_length(&definition.fields);

        if data_len == format_len {
            return Ok(name.clone());
        }
    }

    Err(format!(
        "No se pudo identificar el formato. Ningún formato coincide con la longitud de registro de {} bytes.",
        data_len
    ).into())
}