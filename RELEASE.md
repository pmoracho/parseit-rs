# Notas de Lanzamiento - parseit-rs

## [1.0.3] - 2025-12-06

### ✨ Nuevas Características

- **Tabla interactiva mejorada**: La vista TUI (`--output-type term`) ahora calcula automáticamente el ancho de cada columna basándose en la longitud declarada del campo (`FieldDefinition.len`) o el tamaño del título, tomando el mayor de ambos.
- **Documentación completa**: Se han añadido doc-comments en español a todas las funciones públicas para mejorar la salida de `cargo doc`.
- **Mejor manejo del formato largo**: El flag `--long-format` ahora genera salida correctamente en formato transpuesto (fila, columna, valor) tanto para CSV como para la vista TUI.

### 🔧 Cambios

- **Signature actualizado**: `write_interactive()` ahora recibe `fields: &[FieldDefinition]` para adaptar los anchos de columna.
- **Enrutamiento mejorado**: `write_output()` pasa automáticamente las definiciones de campo a la función correspondiente.
- **Documentación API**: Todas las funciones clave en `parse.rs` y `config.rs` tienen ahora bloques de documentación detallados.

### 🐛 Correcciones

- Se corrigieron issues en la decodificación de caracteres especiales con `encoding_rs::WINDOWS_1252`.
- Mejoras en el handling de líneas de longitud variable o incompletas.

### 📦 Dependencias

```
csvlens             = 0.12.0
serde               = 1.0 (con features derive)
clap                = 4.4 (con features derive)
const_format        = 0.2
toon-format         = 0.3
rust_decimal        = 1.30
num-format          = 0.4
prettytable-rs      = 0.10
encoding_rs         = 0.8.35
tempfile            = 3.8
crossterm           = (vía csvlens)
ratatui             = (vía csvlens)
```

### 🎯 Mejoras de Rendimiento

- Binario compilado con optimizaciones de tamaño (`opt-level = "z"`) y LTO habilitado.
- Reducción de tamaño del ejecutable mediante `strip = true` y `panic = "abort"`.
- Compilación optimizada para distribución con perfil `dist`.

---

## [1.0.2] - 2025-11-25

### ✨ Nuevas Características

- **Soporte para formato largo**: Nuevo flag `--long-format` / `-l` que transpone la salida en formato (fila, columna, valor).
- **Visualización TUI mejorada**: Tabla interactiva con navegación por teclado (`↑`, `↓`, `Home`, `End`, `q`).
- **Lookups de tablas**: Sistema de enriquecimiento de datos mediante tablas de mapeo definidas en `parseit.toon`.

### 🔧 Cambios

- Refactorización de la arquitectura de salida en módulos separados.
- Mejorado handling de argumentos CLI con `clap`.
- Integración de `prettytable-rs` para visualización de formatos disponibles.

### 🐛 Correcciones

- Fixes en el parseo de archivos con saltos de línea inconsistentes.
- Mejor manejo de campos incompletos o malformados.

---

## [1.0.1] - 2025-11-10

### ✨ Nuevas Características

- **Deducción automática de formatos**: La herramienta detecta automáticamente el formato comparando la longitud del primer registro.
- **Múltiples formatos de salida**: Soporte para CSV y terminal interactivo.
- **Formateo numérico avanzado**: 
  - Soporte para tipos `zamount`, `amount` y `numeric`.
  - Decimales configurables.
  - Separadores de miles personalizables.
- **Archivo de configuración TOML**: Definición flexible de formatos y tablas de lookup.

### 🔧 Cambios

- Arquitectura modular: `config.rs`, `parse.rs`, `io.rs`, `main.rs`.
- Uso de `rust_decimal` para precisión en cálculos de montos.
- Soporte para codificación WINDOWS-1252 con `encoding_rs`.

### 🐛 Correcciones

- Handling robusto de caracteres especiales.
- Escape correcto de comillas en salida CSV.

---

## [1.0.0] - 2025-10-15

### 🎉 Lanzamiento Inicial

Primera versión estable de **parseit-rs**, reescritura completa del proyecto original `parseit` en Rust.

### ✨ Características Principales

- **Interpretación de registros de longitud fija**: Lee y parsea archivos de datos de longitud fija según esquemas definidos.
- **Configuración via TOML/TOON**: Archivos `parseit.toon` para definir formatos y tablas.
- **Múltiples opciones de salida**:
  - CSV con delimitador configurable
  - Terminal interactivo
- **Formateo numérico**: Decimales implícitos, separadores de miles.
- **Lookups de tablas**: Enriquecimiento de datos con descripciones.
- **CLI amigable**: Argumentos intuitivos con `clap`, help integrado.

### 📋 Módulos Principales

- `main.rs`: Punto de entrada, CLI y orquestación.
- `config.rs`: Carga y deserialización de configuración.
- `parse.rs`: Lógica principal de parseo y formateo.
- `io.rs`: Salida a CSV o terminal.

---

## Cómo Actualizar

### Desde versiones anteriores

Para actualizar a la última versión:

```bash
git pull origin main
cargo build --release
```

Si has instalado la herramienta globalmente:

```bash
cargo install --path . --force
```

---

## Roadmap Futuro

- [ ] Soporte para JSON como formato de salida.
- [ ] Validación de esquemas con reglas personalizadas.
- [ ] Caché de configuración para mejor rendimiento.
- [ ] Soporte para archivos de entrada comprimidos (gzip, bzip2).
- [ ] Plugin system para tipos de campos personalizados.
- [ ] Modo batch para procesar múltiples archivos.
- [ ] Integración con bases de datos (inserción directa).

---

## Notas de Compatibilidad

### Sistemas soportados

- ✅ Linux (x86_64, aarch64)
- ✅ macOS (Intel, Apple Silicon)
- ✅ Windows (x86_64)

### Requisitos

- Rust 1.70 o superior para compilación desde fuentes.
- No hay dependencias de sistema adicionales (self-contained).

---

## Contribuciones y Reportes

Si encuentras bugs o tienes sugerencias:

1. **GitHub Issues**: [parseit-rs/issues](https://github.com/pmoracho/parseit-rs/issues)
2. **Email**: pmoracho@gmail.com
3. **Pull Requests**: Toda contribución es bienvenida.

---

## Licencia

Consulta el archivo `LICENSE` en el repositorio.

---

## Agradecimientos

- Inspiración en el proyecto original [parseit](https://github.com/pmoracho/parseit).
- Uso de librerías de la comunidad Rust: `clap`, `serde`, `ratatui`, `crossterm`, y muchas más.

---

**Última actualización**: Diciembre 2025  
**Versión actual**: 1.0.3  
**Autor**: Patricio Moracho <pmoracho@gmail.com>
