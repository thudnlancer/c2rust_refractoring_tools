use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;
use yaml_rust::{yaml::YamlLoader, emitter::{YamlEmitter, Emitter}};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut input: Box<dyn BufRead> = Box::new(io::stdin().lock());
    let mut version_directive = None;
    let mut flow = -1; // default no flow style collections
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                usage(0);
                process::exit(0);
            }
            "--flow" => {
                if i + 1 == args.len() {
                    usage(1);
                    process::exit(1);
                }
                i += 1;
                flow = match args[i].as_str() {
                    "keep" => 0,
                    "on" => 1,
                    "off" => -1,
                    _ => {
                        usage(1);
                        process::exit(1);
                    }
                };
            }
            "--directive" => {
                if i + 1 == args.len() {
                    usage(1);
                    process::exit(1);
                }
                i += 1;
                version_directive = match args[i].as_str() {
                    "1.1" => Some((1, 1)),
                    "1.2" => Some((1, 2)),
                    _ => {
                        usage(1);
                        process::exit(1);
                    }
                };
            }
            _ => {
                let file = File::open(&args[i])?;
                input = Box::new(BufReader::new(file));
                break;
            }
        }
        i += 1;
    }

    let mut emitter = Emitter::new(io::stdout());
    emitter.canonical = false;
    emitter.unicode = true;

    for line in input.lines() {
        let line = line?;
        let mut anchor = String::new();
        let mut tag = String::new();

        let ok = if line.starts_with("+STR") {
            emitter.stream_start()?
        } else if line.starts_with("-STR") {
            emitter.stream_end()?
        } else if line.starts_with("+DOC") {
            let implicit = !line[4..].starts_with(" ---");
            emitter.document_start(version_directive, None, None, implicit)?
        } else if line.starts_with("-DOC") {
            let implicit = !line[4..].starts_with(" ...");
            emitter.document_end(implicit)?
        } else if line.starts_with("+MAP") {
            let style = if flow == 1 {
                yaml_rust::emitter::EmitterStyle::Flow
            } else if flow == 0 && line[5..].starts_with("{}") {
                yaml_rust::emitter::EmitterStyle::Flow
            } else {
                yaml_rust::emitter::EmitterStyle::Block
            };
            emitter.mapping_start(
                get_anchor('&', &line, &mut anchor),
                get_tag(&line, &mut tag),
                false,
                style,
            )?
        } else if line.starts_with("-MAP") {
            emitter.mapping_end()?
        } else if line.starts_with("+SEQ") {
            let style = if flow == 1 {
                yaml_rust::emitter::EmitterStyle::Flow
            } else if flow == 0 && line[5..].starts_with("[]") {
                yaml_rust::emitter::EmitterStyle::Flow
            } else {
                yaml_rust::emitter::EmitterStyle::Block
            };
            emitter.sequence_start(
                get_anchor('&', &line, &mut anchor),
                get_tag(&line, &mut tag),
                false,
                style,
            )?
        } else if line.starts_with("-SEQ") {
            emitter.sequence_end()?
        } else if line.starts_with("=VAL") {
            let (value, style) = get_value(&line);
            let implicit = get_tag(&line, &mut tag).is_none();
            emitter.scalar(
                get_anchor('&', &line, &mut anchor),
                get_tag(&line, &mut tag),
                value,
                implicit,
                implicit,
                style,
            )?
        } else if line.starts_with("=ALI") {
            emitter.alias(get_anchor('*', &line, &mut anchor))?
        } else {
            eprintln!("Unknown event: '{}'", line);
            process::exit(1);
        };

        if !ok {
            eprintln!("Emitter error");
            process::exit(1);
        }
    }

    Ok(())
}

fn get_anchor(sigil: char, line: &str, anchor: &mut String) -> Option<&str> {
    let start = line.find(sigil)? + 1;
    let end = line[start..].find(' ').map(|i| start + i).unwrap_or(line.len());
    *anchor = line[start..end].to_string();
    Some(anchor)
}

fn get_tag(line: &str, tag: &mut String) -> Option<&str> {
    let start = line.find('<')? + 1;
    let end = line.find('>')?;
    *tag = line[start..end].to_string();
    Some(tag)
}

fn get_value(line: &str) -> (String, yaml_rust::emitter::EmitterStyle) {
    let mut style = yaml_rust::emitter::EmitterStyle::Plain;
    let mut value = String::new();
    let mut chars = line[4..].chars().peekable();

    // Skip until space
    while let Some(c) = chars.next() {
        if c == ' ' {
            break;
        }
    }

    match chars.peek() {
        Some(':') => style = yaml_rust::emitter::EmitterStyle::Plain,
        Some('\'') => style = yaml_rust::emitter::EmitterStyle::SingleQuoted,
        Some('"') => style = yaml_rust::emitter::EmitterStyle::DoubleQuoted,
        Some('|') => style = yaml_rust::emitter::EmitterStyle::Literal,
        Some('>') => style = yaml_rust::emitter::EmitterStyle::Folded,
        _ => panic!("Invalid value style"),
    }
    chars.next(); // Skip style char

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') => value.push('\\'),
                Some('0') => value.push('\0'),
                Some('b') => value.push('\x08'),
                Some('n') => value.push('\n'),
                Some('r') => value.push('\r'),
                Some('t') => value.push('\t'),
                _ => panic!("Invalid escape sequence"),
            }
        } else {
            value.push(c);
        }
    }

    (value, style)
}

fn usage(ret: i32) {
    eprintln!("Usage: run-emitter-test-suite [--directive (1.1|1.2)] [--flow (on|off|keep)] [<input-file>]");
    process::exit(ret);
}