/// Resuelve una cadena de selección de columnas en índices basados en los encabezados proporcionados.
/// Soporta números de columna (1-indexed), rangos (ej: "1-3") y nombres de columna.
/// #Arguments
/// - `range_str`: Cadena que especifica las columnas a seleccionar (ej: "1,3,5" o "FECHA,MONTO").
/// - `headers`: Vector de nombres de columnas disponibles.
/// - Retorna un vector de índices (0-indexed) correspondientes a las columnas seleccionadas.
/// #Ejemplo
/// ```
/// let headers = vec!["ID".to_string(), "FECHA".to_string(), "MONTO".to_string()];
/// let indices = resolve_columns("1,FECHA,3", &headers);
/// assert_eq!(indices, vec![0, 1, 2]);
/// ```
/// 
pub fn resolve_columns(range_str: &str, headers: &[String]) -> Vec<usize> {
    let mut indices = Vec::new();
    let max_cols = headers.len();

    for part in range_str.split(',') {
        let part = part.trim();
        
        // 1. Intentar como Rango (ej: "1-3")
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() == 2 {
                if let (Ok(s), Ok(e)) = (bounds[0].parse::<usize>(), bounds[1].parse::<usize>()) {
                    for i in s.max(1)..=e.min(max_cols) {
                        indices.push(i - 1);
                    }
                    continue; // Ir a la siguiente parte
                }
            }
        }

        // 2. Intentar como Número simple (ej: "5")
        if let Ok(idx) = part.parse::<usize>() {
            if idx > 0 && idx <= max_cols {
                indices.push(idx - 1);
                continue;
            }
        }

        // 3. Intentar como Nombre de Columna (ej: "FECHA")
        if let Some(pos) = headers.iter().position(|h| h.to_uppercase() == part.to_uppercase()) {
            indices.push(pos);
        }
    }

    indices.dedup(); 
    indices
}

pub fn resolve_rows(range_str: &str, total_rows: usize) -> Vec<usize> {
    let mut indices = Vec::new();

    for part in range_str.split(',') {
        let part = part.trim();
        
        // 1. Intentar como Rango (ej: "1-10")
        if part.contains('-') {
            let bounds: Vec<&str> = part.split('-').collect();
            if bounds.len() == 2 {
                if let (Ok(s), Ok(e)) = (bounds[0].parse::<usize>(), bounds[1].parse::<usize>()) {
                    for i in s.max(1)..=e.min(total_rows) {
                        indices.push(i - 1);
                    }
                    continue; 
                }
            }
        }

        // 2. Intentar como Número simple (ej: "5")
        if let Ok(idx) = part.parse::<usize>() {
            if idx > 0 && idx <= total_rows {
                indices.push(idx - 1);
            }
        }
    }

    indices.dedup(); 
    indices
}