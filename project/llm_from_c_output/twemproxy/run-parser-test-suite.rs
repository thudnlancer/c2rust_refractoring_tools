use std::{
    env, fs,
    io::{self, Read, Write},
    process,
};
use yaml_rust::{yaml::Event, YamlEmitter, YamlLoader};

fn print_escaped(s: &str) {
    for c in s.chars() {
        match c {
            '\\' => print!("\\\\"),
            '\0' => print!("\\0"),
            '\b' => print!("\\b"),
            '\n' => print!("\\n"),
            '\r' => print!("\\r"),
            '\t' => print!("\\t"),
            _ => print!("{}", c),
        }
    }
}

fn usage(ret: i32) -> ! {
    eprintln!("Usage: libyaml-parser [--flow (on|off|keep)] [<input-file>]");
    process::exit(ret);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut flow = -1; // default no flow style collections
    let mut input_file = None;

    let mut i = 1;
    while i < args.len() {
        if args[i].starts_with("--flow") {
            if i + 1 == args.len() {
                usage(1);
            }
            i += 1;
            match args[i].as_str() {
                "keep" => flow = 0,
                "on" => flow = 1,
                "off" => flow = -1,
                _ => usage(1),
            }
        } else if args[i] == "--help" || args[i] == "-h" {
            usage(0);
        } else if input_file.is_none() {
            input_file = Some(&args[i]);
        } else {
            usage(1);
        }
        i += 1;
    }

    let input = if let Some(file) = input_file {
        fs::File::open(file)?
    } else {
        io::stdin()
    };

    let docs = YamlLoader::load_from_reader(input)?;
    for doc in docs {
        let mut events = Vec::new();
        YamlEmitter::new(&mut events).dump(&doc)?;

        for event in events {
            match event {
                Event::Nothing => println!("???"),
                Event::StreamStart => println!("+STR"),
                Event::StreamEnd => println!("-STR"),
                Event::DocumentStart => println!("+DOC"),
                Event::DocumentEnd => println!("-DOC"),
                Event::MappingStart(style) => {
                    print!("+MAP");
                    if (flow == 0 && style.is_flow()) || flow == 1 {
                        print!(" {{}}");
                    }
                    println!();
                }
                Event::MappingEnd => println!("-MAP"),
                Event::SequenceStart(style) => {
                    print!("+SEQ");
                    if (flow == 0 && style.is_flow()) || flow == 1 {
                        print!(" []");
                    }
                    println!();
                }
                Event::SequenceEnd => println!("-SEQ"),
                Event::Scalar(value, style, tag, anchor) => {
                    print!("=VAL");
                    if let Some(anchor) = anchor {
                        print!(" &{}", anchor);
                    }
                    if let Some(tag) = tag {
                        print!(" <{}>", tag);
                    }
                    match style {
                        yaml_rust::yaml::ScalarStyle::Plain => print!(" :"),
                        yaml_rust::yaml::ScalarStyle::SingleQuoted => print!(" '"),
                        yaml_rust::yaml::ScalarStyle::DoubleQuoted => print!(" \""),
                        yaml_rust::yaml::ScalarStyle::Literal => print!(" |"),
                        yaml_rust::yaml::ScalarStyle::Folded => print!(" >"),
                        _ => unreachable!(),
                    }
                    print_escaped(&value);
                    println!();
                }
                Event::Alias(anchor) => println!("=ALI *{}", anchor),
            }
        }
    }

    Ok(())
}