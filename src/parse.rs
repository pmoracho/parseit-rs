use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::error::Error;
use crate::config::{ConfigSchema, FieldDefinition, FormatDefinition};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Formatea una cadena numérica de entrada basada en el tipo de campo y las opciones de salida.
/// 
/// La salida siempre utiliza el formato Latino/Continental (ej: 1.234.567,89).
fn format_field_value(
    raw_value: &str,
    field_type: &str, // Ej: "zamount", "amount", "numeric"
    numeric_format_flag: bool, // Reformatear con separadores S/N
    decimal_places: usize, // Cantidad de decimales implícitos/deseados
) -> String {
    let raw_trimmed = raw_value.trim();

    if raw_trimmed.is_empty() {
        // Valor por defecto para campos vacíos
        return if decimal_places > 0 { "0,00".to_string() } else { "0".to_string() };
    }

    let mut number_string_for_decimal: String;
    let mut final_decimal_places = decimal_places;

    // --- FASE 1: CONVERSIÓN A CADENA ESTÁNDAR (Punto decimal '.') ---
    match field_type.to_lowercase().as_str() {
        
        // 1. ZAMOUNT (Decimal implícito): Los últimos N dígitos son decimales.
        "zamount" => {
            // Eliminar ceros a la izquierda, excepto si el único dígito es cero.
            let trimmed_no_leading_zeros = raw_trimmed.trim_start_matches('0');
            let integer_part = if trimmed_no_leading_zeros.is_empty() {
                "0"
            } else if trimmed_no_leading_zeros.len() <= final_decimal_places {
                let padding = "0".repeat(final_decimal_places - trimmed_no_leading_zeros.len());
                &format!("0{}{}", padding, trimmed_no_leading_zeros)
            } else {
                &trimmed_no_leading_zeros.to_string()
            };

            let len = integer_part.len();
            let index_of_dot = len.checked_sub(final_decimal_places).unwrap_or(0);

            let int_part = &integer_part[0..index_of_dot];
            let dec_part = &integer_part[index_of_dot..];

            number_string_for_decimal = format!("{}.{}", int_part, dec_part);
        },
        
        // 2. AMOUNT (Decimal explícito) y NUMERIC
        _ => {
            // Asumimos que los separadores de miles son puntos o comas que deben eliminarse
            // y que el separador decimal es el último punto o coma.
            
            // Reemplazar todos los separadores de miles (punto o coma) y dejar solo el decimal
            let cleaned = raw_trimmed
                .replace('.', "")
                .replace(',', ".");
                
            number_string_for_decimal = cleaned;

            // Si es 'amount' o 'numeric', el número de decimales deseados es el que se requiere para la salida
            // Si el número de decimales en el string es 0, no forzamos decimales.
            if !number_string_for_decimal.contains('.') && final_decimal_places > 0 {
                 // Si no hay punto, agregamos los decimales al final para que Decimal pueda leerlo.
                 number_string_for_decimal.push_str(&format!(".{:0<width$}", "", width = final_decimal_places));
            }

            // Si es solo "numeric" y decimal_places es 0, ajustamos la escala a 0.
            if field_type.to_lowercase() == "numeric" && final_decimal_places == 0 {
                final_decimal_places = 0;
            } else if final_decimal_places == 0 {
                 final_decimal_places = 2; // Estándar de 2 si es monto y no se especificó
            }
        }
    }
    
    // --- FASE 2: CONVERSIÓN Y FORMATEO DE SALIDA (Aplica si numeric_format_flag = true) ---
    
    // 3. Parsear a Decimal para precisión y manejo de escala
    let mut number = match Decimal::from_str(&number_string_for_decimal) {
        Ok(d) => d,
        Err(_) => return raw_value.to_string(), // Retorna el original si falla el parseo
    };

    // Ajustamos la escala a la cantidad de decimales deseada para el formato de salida
    number.set_scale(final_decimal_places as u32).expect("Fallo al configurar la escala.");

    if !numeric_format_flag {
        // Si no se pide reformatear, devolvemos la representación de string con el punto decimal estándar.
        // Si los decimales son 0, usamos to_string() para evitar decimales.
        return if final_decimal_places == 0 {
            number.to_string()
        } else {
            // Reemplazar el punto por coma para el formato de salida final
            number.to_string().replace('.', ",")
        };
    }

    // --- REFORMATEO DE SALIDA (Separadores de Miles y Decimales) ---

    // Obtener la representación de string del número ya con la escala correcta
    let number_string = number.to_string();
    let parts: Vec<&str> = number_string.split('.').collect();
    
    let integer_part = parts[0];
    let decimal_part = parts.get(1).map_or("00", |v| v); 

    // 4. Aplicar Separador de Miles (Punto '.')
    let mut final_integer = String::new();
    let mut count = 0;

    for char in integer_part.chars().rev() {
        if char == '-' {
            final_integer.push(char);
            continue;
        }
        if count > 0 && count % 3 == 0 {
            final_integer.push('.'); 
        }
        final_integer.push(char);
        count += 1;
    }
    
    let formatted_integer_part: String = final_integer.chars().rev().collect();

    // 5. Ensamblar el resultado final (ej: 1.234.567,89)
    // Usamos coma para el separador decimal.
    format!("{},{}", formatted_integer_part, decimal_part)
}

/// Procesa el archivo de datos de longitud fija y lo escribe a la salida estándar como CSV.
pub fn process_file(file_path: &str, 
                    fields: &[FieldDefinition],
                    schema: &ConfigSchema,
                    delim_character: &str,
                    long_format: bool,
                    format_numeric: bool,
                    dont_use_tables: bool,
                ) -> Result<(), Box<dyn Error>> {
    
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut output = io::stdout().lock();
    
    // --- 1. Escribir Encabezado ---
    if long_format {
        writeln!(output, "#,Columna,Valor")?;
    } else {
        let header: Vec<String> = fields.iter().map(|f| f.nombre.clone()).collect();
        writeln!(output, "{}", header.join(delim_character))?;
    }
    
    for (row_index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let row_num = row_index + 1; // Contar desde 1 para el usuario final
        let mut start_pos = 0;
        let mut record_parts = Vec::new();

        // 3. Iterar sobre la definición de campos para extraer los datos
        for field in fields.iter() {
            let end_pos = start_pos + field.len;
            let raw_value = line[start_pos..end_pos].trim().to_string();
            let mut final_value = raw_value.clone();

            // ******* Lógica de Lookup (Tablas) *******
            let should_lookup = !dont_use_tables;
            if field.tipo == "table" && should_lookup {
                let table_name = &field.param1; 
                if let Some(table) = schema.tables.get(table_name) {
                    if let Some(lookup_value) = table.get(&raw_value) {
                        let value = lookup_value.clone();
                        final_value = format!("{raw_value} - {value}");
                    }
                }
            }
            // *****************************************            
            

            // Asegurarse de no exceder la longitud de la línea
            if end_pos > line.len() {
                // Manejo de error si la línea es más corta de lo esperado
                eprintln!("Advertencia: Línea demasiado corta en registro. Ignorando campo '{}'.", field.nombre);
                record_parts.push("".to_string());
                break;
            }

            final_value = final_value.trim().to_string();

            if field.tipo == "zamount" || field.tipo == "amount" {
                final_value = format_field_value(&final_value, 
                                                &field.tipo, 
                                                format_numeric, 
                                                field.param1.parse::<usize>().unwrap_or(2) // Decimales
                ); 
            }
                        
            
            if long_format {
                writeln!(output, "{},\"{}\",\"{}\"", row_num, field.nombre, final_value)?;
            } else {
                // Almacenar para salida ancha normal
                record_parts.push(final_value);
            }
            start_pos = end_pos;
        }

        // 4. Escribir la línea como CSV a la salida estándar
        if !long_format {
            writeln!(output, "{}", record_parts.join(delim_character))?;
        }
    }

    Ok(())
}

/// Calcula la longitud total de un registro basándose en la suma de las longitudes de sus campos.
fn calculate_format_length(fields: &[FieldDefinition]) -> usize {
    fields.iter().map(|f| f.len).sum()
}

/// Lee la primera línea del archivo de datos y devuelve su longitud.
fn get_first_line_length(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Leer la primera línea. Si está vacía o es solo un salto, puede causar problemas.
    reader.read_line(&mut line)?;

    // La longitud debe ser la longitud del contenido útil, sin incluir el caracter de salto de línea (\n).
    // Usamos .trim_end() para quitar el salto de línea y espacios al final.
    Ok(line.trim_end().len()) 
}

/// Intenta identificar el formato comparando la longitud del archivo con las longitudes de todos los formatos.
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