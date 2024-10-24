// Integrantes
// - Esteban Solano
// - Matias Leer
// - Melissa Carvajal

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// Función principal para imprimir tokens desde un archivo
fn main() {
    let args: Vec<String> = env::args().collect();
    let archivo_entrada = if args.len() > 1 {
        &args[1]
    } else {
        "tokens.out"
    };

    let ruta_entrada = Path::new(archivo_entrada);
    let archivo = File::open(ruta_entrada).expect("Error abriendo el archivo");
    let reader = io::BufReader::new(archivo);

    for line in reader.lines() {
        match line {
            Ok(token) => println!("{}", token),
            Err(err) => eprintln!("Error leyendo el token: {}", err),
        }
    }
}
