use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn remove_parentheses(input: &str) -> String {
    let new = input.replace("(", "").replace(")", "").replace(",", "").replace("[", "").replace("]", "");
    new
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let archivo_entrada = if args.len() > 1 {
        &args[1]
    } else {
        "tree.out"
    };

    let ruta_entrada = Path::new(archivo_entrada);
    let archivo = File::open(ruta_entrada).expect("Error abriendo el archivo");
    let reader = io::BufReader::new(archivo);

    let mut input = String::new();
    for line in reader.lines() {
        let line_content = line?;
        input.push_str(&line_content);
        input.push('\n');
    }

    let result = remove_parentheses(&input);
    println!("{}", result);

    Ok(())
}
