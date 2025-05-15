use std::env;
use std::io::{self, stdin, stdout};
use std::process;
use yaml_rust::{yaml::Event, YamlEmitter, YamlLoader, YamlParser};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut help = false;
    let mut canonical = false;
    let mut unicode = false;

    // Analyze command line options
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

    // Display help
    if help {
        println!("{} [--canonical] [--unicode] <input >output
or\n{} -h | --help\nReformat a YAML stream\n\nOptions:
-h, --help\t\tdisplay this help and exit
-c, --canonical\t\toutput in the canonical YAML format
-u, --unicode\t\toutput unescaped non-ASCII characters",
                args[0], args[0]);
        process::exit(0);
    }

    // Read YAML from stdin
    let input = io::read_to_string(stdin()).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Parse YAML
    let docs = YamlLoader::load_from_str(&input).unwrap_or_else(|e| {
        eprintln!("Parser error: {}", e);
        process::exit(1);
    });

    // Emit YAML
    let mut out = String::new();
    let mut emitter = YamlEmitter::new(&mut out);
    emitter.canonical = canonical;
    emitter.unicode = unicode;

    for doc in docs {
        emitter.dump(&doc).unwrap_or_else(|e| {
            eprintln!("Emitter error: {}", e);
            process::exit(1);
        });
    }

    // Write to stdout
    println!("{}", out);
}