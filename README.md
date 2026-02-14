# parseit-rs

**parseit-rs** es una herramienta CLI moderna y eficiente, escrita en **Rust**,
para procesar e interpretar archivos de datos con registros de **longitud
fija**. Está diseñada especialmente para trabajar con archivos de intercambio de
datos con el ARCA, aunque soporta cualquier esquema de longitud fija definido en
su archivo de configuración.

Es una reescritura completa del proyecto original en ***Python** 
[parseit](https://github.com/pmoracho/parseit), con mejoras en rendimiento,
seguridad y funcionalidades.

## 🎯 Características

- ✅ **Interpretación automática de formatos**: Deduce el formato de un archivo
  comparando su longitud con los esquemas definidos.
- ✅ **Múltiples formatos de salida**:
  - CSV (valores separados por delimitador configurable)
  - Terminal interactivo (TUI basado en Ratatui y Csvlens)
  - Formato largo/transpuesto (fila, columna, valor)
  - Salida HTML
- ✅ **Formateo numérico inteligente**: Soporta montos (zamount, amount) con
  decimales configurables y separadores de miles.
- ✅ **Lookup de tablas**: Enriquece datos con descripciones usando tablas de
  mapeo externas. Ejemplo: cuando el ARCA pide un tipo de documento, en vez de mostrar "80" se muestra "80 - CUIT".
- ✅ **Visualización TUI**: Tabla interactiva con navegación por teclado y anchos
  de columna adaptativos, mediante **csvlens** y **ratatui**.
- ✅ **Configuración flexible**: Archivos de configuración en formato TOON.
- ✅ **Decodificación robusta**: Soporta codificación WINDOWS-1252 (ISO-8859-1) además de UTF-8.
- ✅ **Manejo de errores mejorado**: Mensajes claros para problemas comunes.
- ✅ **Selección de columnas y filas**: Filtrado de datos para mostrar solo lo
  relevante.
- ✅ **Soporte multiplataforma**: Funciona en Linux, macOS y Windows.


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

**Nota**: La vista TUI es ideal para inspeccionar datos rápidamente sin salir del terminal. 
Para más infomación, consultar el sitio de csvlens: https://github.com/YS-L/csvlens


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

- **csvlens**: Visualización TUI de tablas CSV
- **serde**: Deserialización de archivos de configuración TOML
- **clap**: Parseo de argumentos CLI
- **const_format**: Formateo de cadenas en tiempo de compilación
- **toon-format**: Parseo de formato TOON para configuraciones
- **rust_decimal**: Aritmética decimal precisa para montos
- **num-format**: Formateo de números con separadores de miles
- **prettytable-rs**: Renderizado de tablas de texto para terminal
- **encoding_rs**: Decodificación de archivos con codificación WINDOWS-1252
- **tempfile**: Manejo de archivos temporales para tablas de lookup

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

## ❓ Preguntas frecuentes (FAQ)

**P: ¿Cómo defino un nuevo formato?**  
R: Edita `parseit.toon` (en el CWD o directorio del ejecutable) y añade una
nueva clave debajo de `formats` con la estructura adecuada.

**P: ¿Qué pasa si el archivo no tiene el formato esperado?**  
R: La herramienta intentará deducir el formato. Si no encuentra coincidencia,
mostrará un error.

**P: ¿Puedo usar separadores personalizados en CSV?**  
R: Sí, usa `--delim-character ";"` (o el separador que necesites).

**P: ¿Cómo escapo caracteres especiales en los valores?**  
R: Los valores CSV se escapan automáticamente (comillas dobles se duplican).

**P: ¿Puedo procesar archivos con codificación diferente a UTF-8?**  
R: Sí, se soporta WINDOWS-1252 (ISO-8859-1) para archivos provenientes del ARCA.

**P: ¿Cómo puedo ver los formatos disponibles?**  
R: Usa `parseit --show-formats` para listar los formatos definidos en la configuración.
---

## Formatos ARCA soportados

    ┌───────────────────┬────────────────────────────────┬──────────────┬────────────────┐
    │ CATEGORÍA         │ NOMBRE DEL FORMATO             │ Nº DE CAMPOS │ LONGITUD TOTAL │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Arciba       │ arciba-creditos                │ 13           │ 119            │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Arciba       │ arciba-debitos                 │ 21           │ 215            │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Rg3685       │ compras-comprobantes           │ 25           │ 325            │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Rg3685       │ compras-comprobantes-alicuotas │ 8            │ 84             │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Rg3685       │ ventas-comprobantes            │ 22           │ 266            │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Rg3685       │ ventas-comprobantes-alicuotas  │ 6            │ 62             │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Sicore       │ sicore-retenciones             │ 17           │ 144            │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Sicore       │ sicore-sujetos                 │ 7            │ 83             │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Sifere       │ sifere-percepciones            │ 8            │ 51             │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Afip.Sifere       │ sifere-retenciones             │ 9            │ 79             │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Agip.Padrones     │ Padron-iibb-general            │ 12           │ 110            │
    ├───────────────────┼────────────────────────────────┼──────────────┼────────────────┤
    │ Ejemplos.Sencillo │ sample                         │ 3            │ 46             │
    └───────────────────┴────────────────────────────────┴──────────────┴────────────────┘

**Última actualización**: Febrero 2026  
**Versión actual**: 1.0.5

