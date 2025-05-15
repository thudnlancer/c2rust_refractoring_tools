use std::env;
use std::fs::File;
use std::io::{self, Read};
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} file1.yaml ...", args[0]);
        return Ok(());
    }

    for (i, filename) in args.iter().skip(1).enumerate() {
        let number = i + 1;
        print!("[{}] Scanning '{}': ", number, filename);
        io::stdout().flush()?;

        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let docs = match YamlLoader::load_from_str(&contents) {
            Ok(docs) => docs,
            Err(e) => {
                println!("FAILURE (0 tokens)");
                continue;
            }
        };

        let mut count = 0;
        let mut emitter = YamlEmitter::new(Vec::new());
        
        for doc in docs {
            if let Err(_) = emitter.dump(&doc) {
                println!("FAILURE ({} tokens)", count);
                continue;
            }
            count += 1;
        }

        println!("SUCCESS ({} tokens)", count);
    }

    Ok(())
}