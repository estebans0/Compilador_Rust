# Lexer para el Lenguaje "Triangle"

Este proyecto implementa un lexer simple para el lenguaje "Triangle", siguiendo su sintaxis y lexicón. El lexer analiza un archivo fuente y lo divide en tokens como `Integer-Literal`, `Char-Literal`, `Identifier`, `Operator`, `Comment`, `Blank`, y otros, tal como se define en la gramática de "Triangle".

## Estructura del Proyecto

- **`tokenize.rs`**: Realiza el análisis léxico de un archivo fuente y genera una lista de tokens en un archivo de salida (`tokens.out` por defecto).
- **`tokens.rs`**: Lee el archivo de salida generado por `tokenize.rs` y muestra los tokens con sus tipos en la consola.

## Instalación y Configuración (Windows)

### Paso 1: Instalar Rust

Si no tienes Rust instalado, descárgalo e instálalo desde [https://rustup.rs/](https://rustup.rs/).

### Paso 2: Crear el Directorio del Proyecto

Abre una terminal (Command Prompt, PowerShell, etc.) y crea un nuevo directorio para el proyecto:

```bash
mkdir t1_lexer
cd t1_lexer
```

### Paso 3: Inicializar un Proyecto en Rust

Dentro del directorio t1_lexer, ejecutar:

```bash
cargo init
```

Esto creará un archivo Cargo.toml y un directorio src.

### Paso 4: Crear los Archivos de Código Fuente

Navega al directorio src y crea dos archivos: tokenize.rs y tokens.rs:

```bash
cd src
notepad tokenize.rs
```

Pega el contenido del archivo tokenize.rs en el archivo, guarda y cierra. Luego, haz lo mismo con tokens.rs:

```bash
notepad tokens.rs
```

### Paso 5: Modificar Cargo.toml

En el directorio raíz, abre el archivo Cargo.toml y agrega dos targets binarios:

```toml
[package]
name = "t1_lexer"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "tokenize"
path = "src/tokenize.rs"

[[bin]]
name = "tokens"
path = "src/tokens.rs"
```

### Paso 6: Ejecutar el Lexer

1. Tokenizar un Archivo Fuente

Prepara un archivo prueba.tri con código fuente de "Triangle" y ejecuta:

```bash
cargo run --bin tokenize prueba.tri -o salida.tok
```

Esto generará un archivo de tokens salida.tok.

2. Mostrar los Tokens

Para leer los tokens generados:

```bash
cargo run --bin tokens salida.tok
```

Este comando mostrará todos los tokens con sus tipos.
