use std::env;
use std::io::{self, Read, Write};
use std::process;
use yaml_rust::{YamlEmitter, YamlLoader, Yaml};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut help = false;
    let mut canonical = false;
    let mut unicode = false;

    // Parse command line options
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-c" | "--canonical" => canonical = true,
            "-u" | "--unicode" => unicode = true,
            _ => {
                eprintln!("Unrecognized option: {}\nTry `{} --help` for more information.", 
                         arg, args[0]);
                process::exit(1);
            }
        }
    }

    // Display help if requested
    if help {
        println!("{} <input\nor\n{} -h | --help\nDeconstruct a YAML stream\n\nOptions:\n\
                 -h, --help\t\tdisplay this help and exit\n\
                 -c, --canonical\t\toutput in the canonical YAML format\n\
                 -u, --unicode\t\toutput unescaped non-ASCII characters",
                 args[0], args[0]);
        return;
    }

    // Read input from stdin
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("Failed to read input: {}", e);
        process::exit(1);
    }

    // Parse YAML
    let docs = match YamlLoader::load_from_str(&input) {
        Ok(docs) => docs,
        Err(e) => {
            eprintln!("Failed to parse YAML: {}", e);
            process::exit(1);
        }
    };

    // Emit YAML
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.canonical = canonical;
    emitter.unicode = unicode;

    for doc in docs {
        if let Err(e) = emitter.dump(&doc) {
            eprintln!("Failed to emit YAML: {}", e);
            process::exit(1);
        }
    }

    // Output result
    if let Err(e) = io::stdout().write_all(out_str.as_bytes()) {
        eprintln!("Failed to write output: {}", e);
        process::exit(1);
    }
}