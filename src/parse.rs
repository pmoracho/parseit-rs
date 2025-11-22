use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::error::Error;
use crate::config::{ConfigSchema, FieldDefinition};

/// Procesa el archivo de datos de longitud fija y lo escribe a la salida estándar como CSV.
pub fn process_file(file_path: &str, 
                    fields: &[FieldDefinition],
                    schema: &ConfigSchema
                ) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut output = io::stdout().lock();
    
    // 1. Escribir encabezado CSV
    let header: Vec<String> = fields.iter().map(|f| f.nombre.clone()).collect();
    writeln!(output, "{}", header.join(","))?;

    // 2. Procesar cada línea del archivo de datos
    for line_result in reader.lines() {
        let line = line_result?;
        let mut start_pos = 0;
        let mut record_parts = Vec::new();

        // 3. Iterar sobre la definición de campos para extraer los datos
        for field in fields.iter() {
            let end_pos = start_pos + field.len;
            let raw_value = line[start_pos..end_pos].trim().to_string();
            let mut final_value = raw_value.clone();

            // ******* Lógica de Lookup (Tablas) *******
            if field.tipo == "table" {
                let table_name = &field.param1; 
                if let Some(table) = schema.tables.get(table_name) {
                    if let Some(lookup_value) = table.get(&raw_value) {
                        let value = lookup_value.clone();
                        final_value = format!("{raw_value} - {value}");
                    }
                }
            }
            // *****************************************            

            // ******* Lógica de Monto (zamount) *******
            if field.tipo == "zamount" && !raw_value.is_empty() {
                // El número de decimales está en param1 (ej: "2")
                if let Ok(decimals) = field.param1.parse::<usize>() {
                    if raw_value.len() > decimals {
                        // Insertar el punto decimal
                        let point_position = raw_value.len() - decimals;
                        
                        let integer_part = &raw_value[..point_position];
                        let decimal_part = &raw_value[point_position..];
                        
                        final_value = format!("{}.{}", integer_part.trim_start_matches('0'), decimal_part);
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

            record_parts.push(final_value);
            start_pos = end_pos;
        }

        // 4. Escribir la línea como CSV a la salida estándar
        writeln!(output, "{}", record_parts.join(","))?;
    }

    Ok(())
}