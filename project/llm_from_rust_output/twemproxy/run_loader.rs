use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} file1.yaml ...", args[0]);
        return Ok(());
    }

    for (i, filename) in args.iter().skip(1).enumerate() {
        let number = i + 1;
        print!("[{}] Loading '{}': ", number, filename);
        std::io::stdout().flush()?;

        let mut file = File::open(Path::new(filename))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        match YamlLoader::load_from_str(&contents) {
            Ok(docs) => {
                println!("SUCCESS ({} documents)", docs.len());
            }
            Err(e) => {
                println!("FAILURE");
                eprintln!("Error parsing YAML file '{}': {}", filename, e);
            }
        }
    }

    Ok(())
}