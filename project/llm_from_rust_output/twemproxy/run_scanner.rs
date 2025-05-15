use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} file1.yaml ...", args[0]);
        return;
    }

    for (i, filename) in args.iter().skip(1).enumerate() {
        let number = i + 1;
        print!("[{}] Scanning '{}': ", number, filename);
        std::io::stdout().flush().unwrap();

        let result = scan_yaml_file(filename);
        match result {
            Ok(count) => println!("SUCCESS ({} tokens)", count),
            Err(e) => println!("FAILURE ({})", e),
        }
    }
}

fn scan_yaml_file(filename: &str) -> Result<usize, String> {
    let path = Path::new(filename);
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    let docs = YamlLoader::load_from_str(&contents).map_err(|e| e.to_string())?;
    Ok(docs.len())
}