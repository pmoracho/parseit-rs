//! Módulo de configuración para parseit-rs.
//! Define estructuras y funciones para cargar y manejar la configuración desde archivos TOML.
//! Utiliza la biblioteca `serde` para deserialización y `toon_format` para decodificación TOML.
//! Proporciona funciones para cargar la configuración desde rutas específicas y calcular longitudes de formatos.
//! También define constantes relacionadas con la configuración.
//! 
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Nombre del archivo de configuración esperado.
/// Se busca en el CWD y en el directorio del ejecutable.
pub const CONFIG_FILE: &str = "parseit.toon";

/// Estructura que representa el esquema de configuración completo.
/// - formatos: Mapa de nombres de formatos a sus definiciones.
/// - tablas: Mapa de nombres de tablas a sus datos (no usado directamente aquí).
/// - atajos: Mapa de atajos a sus valores (no usado directamente aquí).
#[derive(Debug, Deserialize)]
pub struct ConfigSchema {
    pub formats: HashMap<String, FormatDefinition>,
    #[allow(dead_code)]
    pub tables: HashMap<String, HashMap<String, String>>,
    #[allow(dead_code)]
    pub shortcuts: HashMap<String, String>,
}

/// Definición de un formato específico.
/// - category: Categoría del formato (no usado directamente aquí).
/// - delimiter: Delimitador utilizado en el formato (no usado directamente aquí).
/// - fields: Vector de definiciones de campos que componen el formato.
#[derive(Debug, Deserialize)]
pub struct FormatDefinition {
    #[allow(dead_code)]
    pub category: String,
    #[allow(dead_code)]
    pub delimiter: String,
    pub fields: Vec<FieldDefinition>, 
}

/// Definición de un campo dentro de un formato
/// - nombre: Nombre del campo
/// - len: Longitud del campo
/// - tipo: Tipo de dato (ej: string, integer, etc.)
/// - param1, param2: Parámetros adicionales (dependiendo del tipo)
#[derive(Debug, Deserialize)]
pub struct FieldDefinition {
    pub nombre: String,
    pub len: usize,
    #[allow(dead_code)]
    pub tipo: String,
    #[allow(dead_code)]
    pub param1: String,
    #[allow(dead_code)]
    pub param2: String,
}

/// Intenta cargar el archivo de configuración primero desde el CWD, luego desde el directorio del ejecutable.
/// 
/// ## Argumentos
/// - `path`: Ruta al archivo de configuración.
/// - Retorna un `ConfigSchema` si se carga exitosamente, o un error si falla.
    /// 
/// ## Retorno
/// `Result<ConfigSchema, Box<dyn Error>>` - Esquema de configuración o error.
/// 
/// ## Errores
/// Retorna un error si el archivo no se puede leer o si el contenido no es válido
/// de acuerdo al esquema esperado.
/// 
/// ## Ejemplo
/// ```
/// let schema = load_config(Path::new("parseit.toon"))?;
/// ```
pub fn load_config(path: &Path) -> Result<ConfigSchema, Box<dyn Error>> {

    let content = fs::read_to_string(path)?; 
    let content_clean = content.trim_start_matches('\u{feff}'); 
    let schema: ConfigSchema = toon_format::decode_default(content_clean)?;

    Ok(schema)
}

/// Intenta cargar el archivo de configuración desde múltiples rutas posibles.
/// 
/// ## Argumentos
/// - Primero intenta desde el directorio actual de ejecución (CWD).
/// - Luego intenta desde el directorio del ejecutable.
/// - Retorna un `ConfigSchema` si se carga exitosamente, o un error si no se encuentra.
/// 
/// ## Retorno
/// `Result<ConfigSchema, Box<dyn Error>>` - Esquema de configuración o error.
/// 
/// ## Errores
/// Retorna un error si no se encuentra el archivo de configuración en ninguna de las rutas.
/// 
/// #Ejemplo
/// ```
/// let schema = load_config_from_paths()?;
/// ```
pub fn load_config_from_paths() -> Result<ConfigSchema, Box<dyn Error>> {
    
    // Lista de rutas a intentar, en orden de prioridad.
    let mut search_paths: Vec<PathBuf> = Vec::new();

    // 1. Ruta Actual de Ejecución (Current Working Directory - CWD)
    if let Ok(cwd) = std::env::current_dir() {
        let config_path = cwd.join(CONFIG_FILE);
        search_paths.push(config_path);
    }
    
    // 2. Ruta del Ejecutable (Donde está instalado el binario)
    if let Ok(mut exe_path) = std::env::current_exe() {
        // Obtenemos el directorio padre (eliminamos el nombre del ejecutable)
        if exe_path.pop() {
            let config_path = exe_path.join(CONFIG_FILE);
            search_paths.push(config_path);
        }
    }

    // 3. Iterar y cargar la configuración
    for path in &search_paths {
        if path.exists() {
            return crate::config::load_config(path); // Asume que load_config ahora toma PathBuf o &Path
        }
    }

    // 4. Si ninguna ruta funciona, retornar error.
    Err(format!(
        "No se pudo encontrar el archivo de configuración '{}' en ninguna de las rutas de búsqueda.",
        CONFIG_FILE
    ).into())
}

/// Calcula la longitud total de un formato sumando las longitudes de sus campos.
/// #Arguments
/// - `fields`: Vector de definiciones de campos del formato.
/// - Retorna la suma de las longitudes de los campos.
/// #Ejemplo
/// ```
/// let total_length = calculate_format_length(&fields);
/// assert_eq!(total_length, 42);
/// ```
pub fn calculate_format_length(fields: &[FieldDefinition]) -> usize {
    fields.iter().map(|f| f.len).sum()
}
