use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} file1.yaml ...", args[0]);
        return;
    }

    for (number, filename) in args.iter().skip(1).enumerate() {
        let number = number + 1;
        print!("[{}] Loading '{}': ", number, filename);
        std::io::stdout().flush().unwrap();

        match process_yaml_file(filename) {
            Ok(count) => println!("SUCCESS ({} documents)", count),
            Err(e) => println!("FAILURE (0 documents) - {}", e),
        }
    }
}

fn process_yaml_file(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let docs = YamlLoader::load_from_str(&contents)?;
    Ok(docs.len())
}