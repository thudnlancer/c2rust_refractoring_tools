use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use yaml_rust::{YamlLoader, YamlEmitter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} file1.yaml ...", args[0]);
        return Ok(());
    }

    for (i, filename) in args.iter().skip(1).enumerate() {
        let number = i + 1;
        print!("[{}] Parsing '{}': ", number, filename);
        std::io::stdout().flush()?;

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let docs = match YamlLoader::load_from_reader(reader) {
            Ok(docs) => docs,
            Err(e) => {
                println!("FAILURE (0 events)");
                eprintln!("Error parsing YAML file {}: {}", filename, e);
                continue;
            }
        };

        let count = docs.len();
        println!("SUCCESS ({} events)", count);
    }

    Ok(())
}