# 1. Lexer para el Lenguaje "Triangle"

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

---

# 2. Parser de Triangle

Esta guía explica el funcionamiento del parser que hemos implementado en Rust para el lenguaje Triangle, siguiendo un ejemplo simple y paso a paso desde la lectura del archivo de entrada, hasta la construcción del árbol de parsing.

## 2.1. Lectura del Archivo de Entrada
### Input: `salida.tok` (archivo generado por el lexer)
El archivo de entrada salida.tok contiene los tokens generados por el lexer, con la siguiente estructura:

Este archivo representa el programa que declara una constante `x` con valor `5` y luego asigna a `x` el valor `10`. Cada línea contiene el tipo de token, el lexema, la fila y la columna de donde fue obtenido.

### Lectura del archivo
El parser lee el archivo `salida.tok` y convierte cada línea en una estructura `Token` que contiene:

* Tipo de Token (`token_type`): Representa el tipo (ej: `Let`, `Const`, `Identifier`).
* Lexema (`lexeme`): El valor textual asociado al token (ej: `let`, `x`, `5`).
* Fila y Columna (`row`, `col`): Posición en el archivo fuente donde fue encontrado.

## 2.2. Inicialización del Parser
El parser se inicializa utilizando el vector de tokens leídos del archivo. El constructor `Parser::new(tokens)` toma el primer token como `current_token` y prepara el parser para comenzar el proceso de parsing:

El parser usa un índice para rastrear la posición actual en el vector de tokens.

## 2.3. Comenzando el Parsing
El proceso de parsing comienza llamando al método `parse`, que a su vez llama a `parse_command`. Este método intenta construir un árbol de parsing a partir de los tokens leídos.

### Ejemplo Inicial: `let ... in ...`
El primer token es `Let`, por lo que el parser debe manejar una expresión let. El método `parse_single_command` maneja este tipo de construcción:

1. Token `Let`:
    * El parser avanza al siguiente token usando `next_token()`
    * Luego llama a `parse_declaration_sequence()` para manejar las declaraciones dentro del bloque let

2. Declaración de Const:
    * El método `parse_declaration_sequence()` llama a `parse_single_declaration()`, que maneja la declaración de constantes, variables, funciones, etc
    * Encuentra `Const`, luego el identificador `x`, y finalmente el valor `5` después del símbolo `~`
    * Esta información se utiliza para crear un nodo `ASTNode::Const(name, value)`, donde `name` es `x` y `value` es `5`.

3. Token `In`:
    * Después de la declaración, el parser espera el token `In` para indicar el comienzo de la ejecución del comando dentro del bloque `let`
    * Si el token `In` es encontrado, se llama al método `parse_command()` nuevamente para manejar los comandos dentro del bloque

4. Asignación:
    * Dentro del bloque `In`, el parser encuentra el identificador `x`, seguido del token de asignación `:=` y el valor `10`
    * Esto resulta en la creación de un nodo `ASTNode::Assign`, que contiene el nombre `x` y la expresión `10`

## 2.4. Construcción del Árbol de Parsing
Durante el proceso de parsing, se crean varios nodos que representan la estructura del programa. Estos nodos se organizan en un árbol de parsing (AST).

### Estructura del Árbol
Para el ejemplo dado, el árbol de parsing generado se vería así:

El nodo `Let` contiene dos hijos:
1. Una declaración constante `Const`, con el nombre `x` y el valor `5`
2. Un bloque `In` que contiene una asignación `Assign`, donde `x` toma el valor `10`

Este árbol se genera recursivamente conforme el parser va avanzando a través de los tokens y utilizando los diferentes métodos de parsing como `parse_command`, `parse_single_command`, `parse_expression`, etc.

## 2.5. Escritura del Árbol en el Archivo de Salida
Una vez que se completa el parsing y se genera el árbol de parsing, el árbol se escribe en un archivo de salida, llamado `arbol.out`. El método `write_ast_to_file(ast, output_file)` toma el árbol y lo formatea en un estilo legible.

### Ejemplo del archivo `arbol.out`:
Cada nodo del árbol se escribe de manera jerárquica, mostrando la estructura y las relaciones entre los diferentes componentes del programa.

## 2.6. Manejo de Errores
Cada vez que se encuentra un token inesperado, se lanza un `SyntaxError` que detalla:
* Token esperado: Qué token esperaba el parser en ese momento.
* Token encontrado: Cuál fue el token real que se encontró.
* Posición: La fila y columna del error en el archivo de entrada.

Por ejemplo, si se espera un token `In` y se encuentra un `Semicolon`, el parser lanzaría un error como este:
```
Error at row 2, col 37: UnexpectedToken { expected: In, found: Semicolon, row: 2, col: 37 }
```

Esto facilita la depuración, ya que se puede ubicar rápidamente el error en el código fuente.

## 2.7. Resumen
* Lectura del Archivo: El archivo de entrada se lee y convierte en una lista de tokens.
* Inicialización del Parser: El parser se inicializa con los tokens y comienza a analizar el programa.
* Parsing Recursivo: Se utiliza un conjunto de métodos para analizar comandos (`let`, `if`, `begin ... end`, etc.) y construir el árbol de parsing.
* Construcción del Árbol: Cada construcción se representa como un nodo en el árbol de parsing, que finalmente se escribe en un archivo de salida.
* Manejo de Errores: Cualquier error durante el parsing se reporta con el detalle necesario para corregirlo fácilmente.

## 2.8. Ejecución
El lexer puede ser ejecutado mediante terminal usando el comando:
```bash
cargo run --bin parse salida.tok -o arbol.out
```