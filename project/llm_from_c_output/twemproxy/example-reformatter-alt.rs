use std::env;
use std::io::{self, stdin, stdout};
use yaml_rust::{YamlEmitter, YamlLoader};

fn main() -> io::Result<()> {
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
                return Ok(());
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
        return Ok(());
    }

    // Read input from stdin
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    // Parse YAML
    let docs = match YamlLoader::load_from_str(&input) {
        Ok(docs) => docs,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return Ok(());
        }
    };

    // Emit YAML
    let mut out = String::new();
    let mut emitter = YamlEmitter::new(&mut out);
    emitter.canonical = canonical;
    emitter.unicode = unicode;

    for doc in docs {
        if let Err(e) = emitter.dump(&doc) {
            eprintln!("Emitter error: {}", e);
            return Ok(());
        }
    }

    // Write output to stdout
    print!("{}", out);

    Ok(())
}