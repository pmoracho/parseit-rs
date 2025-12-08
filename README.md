# parseit-rs

**parseit-rs** es una herramienta CLI moderna y eficiente, escrita en **Rust**,
para procesar e interpretar archivos de datos con registros de **longitud
fija**. Está diseñada especialmente para trabajar con archivos de intercambio de
datos con el ARCA, aunque soporta cualquier esquema de longitud fija definido en
su archivo de configuración.

Es una reescritura completa del proyecto original
[parseit](https://github.com/pmoracho/parseit), con mejoras en rendimiento,
seguridad y funcionalidades.

## 🎯 Características

- ✅ **Interpretación automática de formatos**: Deduce el formato de un archivo
  comparando su longitud con los esquemas definidos.
- ✅ **Múltiples formatos de salida**:
  - CSV (valores separados por delimitador configurable)
  - Terminal interactivo (TUI basado en Ratatui y Csvlens)
  - Formato largo/transpuesto (fila, columna, valor)
- ✅ **Formateo numérico inteligente**: Soporta montos (zamount, amount) con
  decimales configurables y separadores de miles.
- ✅ **Lookup de tablas**: Enriquece datos con descripciones usando tablas de
  mapeo externas.
- ✅ **Visualización TUI**: Tabla interactiva con navegación por teclado y anchos
  de columna adaptativos.
- ✅ **Configuración flexible**: Archivos de configuración en formato TOON.
- ✅ **Decodificación robusta**: Soporta codificación WINDOWS-1252 (ISO-8859-1).

## 📦 Instalación desde el código fuente

### Requisitos previos

- Rust 1.70+ ([Instalar Rust](https://rustup.rs/))

### Compilación desde fuentes

```bash
git clone https://github.com/pmoracho/parseit-rs.git
cd parseit-rs
cargo build --release
```

El binario compilado estará en `target/release/parseit` (Linux/macOS) o
`target/release/parseit.exe` (Windows).

### Instalación global

```bash
cargo install --path .
```

## 🚀 Uso

### Sintaxis básica

```bash
parseit --data-file <ARCHIVO> [OPTIONS]
```

### Ejemplos

#### 1. Procesar un archivo con formato automático y salida CSV

```bash
parseit -d datos.dat -o csv
```

#### 2. Especificar un formato conocido

```bash
parseit -d datos.dat --format-name sample -o csv
```

#### 3. Ver tabla interactiva en terminal

```bash
parseit -d datos.dat -o term
```

#### 4. Formato largo (transpuesto)

```bash
parseit -d datos.dat --long-format -o csv
```

#### 5. Formateo numérico con separadores de miles

```bash
parseit -d datos.dat --format-numeric -o csv
```

#### 6. Sin tablas de lookup (valores crudos)

```bash
parseit -d datos.dat --dont-use-tables -o csv
```

#### 7. Listar formatos disponibles

```bash
parseit --show-formats
```

### Opciones disponibles

| Opción | Corto | Valor por defecto | Descripción |
|--------|-------|-------------------|-------------|
| `--data-file` | `-d` | (requerido) | Ruta al archivo de datos de longitud fija |
| `--format-name` | `-f` | (auto) | Nombre del formato a usar (se deduce si no se proporciona) |
| `--output-type` | `-o` | `csv` | Tipo de salida: `csv` o `term` |
| `--delim-character` | `-c` | `,` | Delimitador para CSV |
| `--long-format` | `-l` | `false` | Formato transpuesto (fila, columna, valor) |
| `--format-numeric` | `-n` | `false` | Aplicar separadores de miles a montos |
| `--dont-use-tables` | `-t` | `false` | Omitir lookups de tablas, usar valores crudos |
| `--show-formats` | `-s` | `false` | Mostrar formatos disponibles y salir |

## 📋 Archivo de configuración

La herramienta busca un archivo `parseit.toon` (formato TOML) en:

1. Directorio actual (CWD)
2. Directorio del ejecutable

### Estructura del archivo de configuración

```toml
[formats]

[formats.sample]
category = "ARCA"
delimiter = ","
[[formats.sample.fields]]
nombre = "idOperacion"
len = 8
tipo = "string"
param1 = ""
param2 = ""

[[formats.sample.fields]]
nombre = "monto"
len = 10
tipo = "zamount"
param1 = "2"
param2 = ""

[tables]

[tables.sifere-jurisdicciones]
"01" = "Buenos Aires"
"02" = "CABA"
"03" = "Catamarca"
```

### Tipos de campo soportados

- `string`: Texto simple (sin procesamiento especial)
- `numeric`: Número sin decimales configurables
- `amount`: Monto con decimales (estándar: 2)
- `zamount`: Monto de longitud fija con ceros a izquierda con decimales implícitos
- `table`: Campo que se enriquece con lookup en tablas

## 🎮 Vista interactiva (Terminal TUI)

Cuando usas `--output-type term`, se abre una tabla interactiva con:

**Controles**:
- `↑` / `↓` o `k` / `j`: Navegar entre filas
- `Home` / `End`: Primera/última fila
- `q` / `Esc` / `Ctrl+C`: Salir

**Características**:
- Las columnas se dimensionan automáticamente en función de `field.len` y el tamaño del título.
- Las filas seleccionadas se destacan en amarillo.
- Soporta desplazamiento horizontal para archivos muy anchos.

## 📊 Formatos de salida

### CSV (por defecto)

```
idOperacion,monto,jurisdicción
00000001,"1.234,56","01 - Buenos Aires"
00000002,"2.345,67","02 - CABA"
```

### Formato largo (`--long-format`)

```
#,Columna,Valor
1,idOperacion,00000001
1,monto,"1.234,56"
1,jurisdicción,"01 - Buenos Aires"
2,idOperacion,00000002
2,monto,"2.345,67"
2,jurisdicción,"02 - CABA"
```

## 📁 Estructura del proyecto

```
parseit-rs/
├── src/
│   ├── main.rs          # Punto de entrada, parseo de argumentos CLI
│   ├── config.rs        # Carga y manejo de configuración (TOML/TOON)
│   ├── parse.rs         # Lógica principal: lectura, parseo, formateo de datos
│   └── io.rs            # Escritura de salidas (CSV, TUI)
├── Cargo.toml           # Dependencias y metadatos del proyecto
├── parseit.toon         # Archivo de configuración de ejemplo
└── README.md            # Este archivo
```

## 🔧 Módulos

### `config.rs`
Maneja la carga y deserialización de archivos de configuración TOML/TOON. Define
estructuras como `ConfigSchema`, `FormatDefinition` y `FieldDefinition`.

### `parse.rs`
Contiene la lógica principal:
- **`parse_to_records`**: Lee el archivo, parsea registros de longitud fija, aplica lookups y formateo.
- **`format_field_value`**: Formatea números según reglas de decimales y separadores.
- **`deduce_format`**: Detecta el formato automáticamente.
- **`write_interactive`**: Renderiza tabla TUI con Ratatui.
- **`write_csv_output`**: Escribe CSV con escapado de comillas.

### `io.rs`
Enrutamiento de salida hacia CSV o terminal interactivo.

### `main.rs`
Interfaz CLI con `clap`, manejo de argumentos y orquestación del flujo.

## 🛠️ Desarrollo

### Generar documentación

```bash
cargo doc --open
```

Se genera documentación interactiva con detalles de funciones, parámetros y ejemplos.

### Ejecutar pruebas (si existen)

```bash
cargo test
```

### Compilar en modo debug

```bash
cargo build
```

### Compilar versión optimizada

```bash
cargo build --release
```

## 📚 Dependencias

- **clap**: Parseo de argumentos CLI
- **serde**: Deserialización de TOML
- **rust_decimal**: Aritmética decimal precisa
- **encoding_rs**: Decodificación WINDOWS-1252
- **ratatui**: UI de terminal interactiva
- **crossterm**: Control de terminal
- **prettytable-rs**: Tablas de texto
- **toon-format**: Parseo de formato TOON

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Por favor:

1. Haz fork del repositorio
2. Crea una rama para tu feature (`git checkout -b feature/mi-feature`)
3. Commit tus cambios (`git commit -am 'Agrega mi feature'`)
4. Push a la rama (`git push origin feature/mi-feature`)
5. Abre un Pull Request

## 📄 Licencia

Consulta el archivo `LICENSE` (si existe) o contacta al autor.

## 👤 Autor

**Patricio Moracho**  
Email: pmoracho@gmail.com  
GitHub: [@pmoracho](https://github.com/pmoracho)

---

## Ejemplos de casos de uso

### Procesar datos ARCA y exportar a CSV

```bash
parseit -d archivo_arca.dat -f arca_format -o csv > salida.csv
```

### Inspeccionar datos de forma interactiva

```bash
parseit -d archivo_arca.dat -o term
```

### Obtener datos en formato largo para análisis posterior

```bash
parseit -d archivo_arca.dat --long-format -o csv | grep "monto"
```

### Aplicar formateo numérico y usar delimitador personalizado

```bash
parseit -d archivo_arca.dat --format-numeric -c ";" -o csv
```

## ❓ Preguntas frecuentes (FAQ)

**P: ¿Cómo defino un nuevo formato?**  
R: Edita `parseit.toon` (en el CWD o directorio del ejecutable) y añade una
nueva sección `[formats.tunuevo]` con los campos correspondientes.

**P: ¿Qué pasa si el archivo no tiene el formato esperado?**  
R: La herramienta intentará deducir el formato. Si no encuentra coincidencia,
mostrará un error.

**P: ¿Puedo usar separadores personalizados en CSV?**  
R: Sí, usa `--delim-character ";"` (o el separador que necesites).

**P: ¿Cómo escapo caracteres especiales en los valores?**  
R: Los valores CSV se escapan automáticamente (comillas dobles se duplican).

---

**Última actualización**: Diciembre 2025

