use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ConfigSchema {
    pub formats: HashMap<String, FormatDefinition>,
    #[allow(dead_code)]
    pub tables: HashMap<String, HashMap<String, String>>,
    #[allow(dead_code)]
    pub shortcuts: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct FormatDefinition {
    #[allow(dead_code)]
    pub category: String,
    #[allow(dead_code)]
    pub delimiter: String,
    pub fields: Vec<FieldDefinition>, 
}

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

// --- Lógica de Carga de Configuración ---
pub fn load_config(filename: &str) -> Result<ConfigSchema, Box<dyn Error>> {

    let content = fs::read_to_string(filename)?; 
    let content_clean = content.trim_start_matches('\u{feff}'); 
    let schema: ConfigSchema = toon_format::decode_default(content_clean)?;

    Ok(schema)
}