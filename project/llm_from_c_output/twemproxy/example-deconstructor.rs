use yaml_rust::{yaml::YamlLoader, yaml::YamlEmitter};
use std::env;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut help = false;
    let mut canonical = false;
    let mut unicode = false;

    // Parse command line arguments
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-c" | "--canonical" => canonical = true,
            "-u" | "--unicode" => unicode = true,
            _ => {
                eprintln!("Unrecognized option: {}\nTry `{} --help` for more information.", arg, args[0]);
                std::process::exit(1);
            }
        }
    }

    if help {
        println!("{} <input\nor\n{} -h | --help\nDeconstruct a YAML stream\n\nOptions:\n-h, --help\t\tdisplay this help and exit\n-c, --canonical\t\toutput in the canonical YAML format\n-u, --unicode\t\toutput unescaped non-ASCII characters",
                args[0], args[0]);
        return Ok(());
    }

    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse YAML
    let docs = match YamlLoader::load_from_str(&input) {
        Ok(docs) => docs,
        Err(e) => {
            eprintln!("Error parsing YAML: {}", e);
            std::process::exit(1);
        }
    };

    // Emit YAML
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.canonical = canonical;
    emitter.unicode = unicode;

    for doc in docs {
        if let Err(e) = emitter.dump(&doc) {
            eprintln!("Error emitting YAML: {}", e);
            std::process::exit(1);
        }
    }

    print!("{}", out_str);
    Ok(())
}