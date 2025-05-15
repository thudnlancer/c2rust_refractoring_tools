use std::cmp::Ordering;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::mem;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;
use std::slice;
use yaml_rust::{yaml, YamlEmitter, YamlLoader};

const BUFFER_SIZE: usize = 65536;
const MAX_EVENTS: usize = 1024;

fn copy_event(event_to: &mut yaml::Event, event_from: &yaml::Event) -> bool {
    match event_from {
        yaml::Event::StreamStart(encoding) => {
            *event_to = yaml::Event::StreamStart(*encoding);
            true
        }
        yaml::Event::StreamEnd => {
            *event_to = yaml::Event::StreamEnd;
            true
        }
        yaml::Event::DocumentStart(doc_start) => {
            *event_to = yaml::Event::DocumentStart(doc_start.clone());
            true
        }
        yaml::Event::DocumentEnd(doc_end) => {
            *event_to = yaml::Event::DocumentEnd(*doc_end);
            true
        }
        yaml::Event::Alias(alias) => {
            *event_to = yaml::Event::Alias(alias.clone());
            true
        }
        yaml::Event::Scalar(scalar) => {
            *event_to = yaml::Event::Scalar(scalar.clone());
            true
        }
        yaml::Event::SequenceStart(seq_start) => {
            *event_to = yaml::Event::SequenceStart(seq_start.clone());
            true
        }
        yaml::Event::SequenceEnd => {
            *event_to = yaml::Event::SequenceEnd;
            true
        }
        yaml::Event::MappingStart(map_start) => {
            *event_to = yaml::Event::MappingStart(map_start.clone());
            true
        }
        yaml::Event::MappingEnd => {
            *event_to = yaml::Event::MappingEnd;
            true
        }
        _ => false,
    }
}

fn compare_events(event1: &yaml::Event, event2: &yaml::Event) -> bool {
    match (event1, event2) {
        (yaml::Event::StreamStart(e1), (yaml::Event::StreamStart(e2)) => e1 == e2,
        (yaml::Event::DocumentStart(d1), (yaml::Event::DocumentStart(d2))) => {
            d1.version == d2.version
                && d1.tags == d2.tags
                && d1.implicit == d2.implicit
        }
        (yaml::Event::DocumentEnd(d1), (yaml::Event::DocumentEnd(d2)) => d1 == d2,
        (yaml::Event::Alias(a1), (yaml::Event::Alias(a2))) => a1 == a2,
        (yaml::Event::Scalar(s1), (yaml::Event::Scalar(s2))) => {
            s1.tag == s2.tag
                && s1.value == s2.value
                && s1.style == s2.style
                && s1.plain_implicit == s2.plain_implicit
                && s1.quoted_implicit == s2.quoted_implicit
        }
        (yaml::Event::SequenceStart(s1), (yaml::Event::SequenceStart(s2))) => {
            s1.tag == s2.tag && s1.implicit == s2.implicit && s1.style == s2.style
        }
        (yaml::Event::MappingStart(m1), (yaml::Event::MappingStart(m2))) => {
            m1.tag == m2.tag && m1.implicit == m2.implicit && m1.style == m2.style
        }
        (yaml::Event::StreamEnd, yaml::Event::StreamEnd)
        | (yaml::Event::SequenceEnd, yaml::Event::SequenceEnd)
        | (yaml::Event::MappingEnd, yaml::Event::MappingEnd) => true,
        _ => false,
    }
}

fn print_output(name: &str, buffer: &[u8], size: usize, count: Option<usize>) -> io::Result<()> {
    if let Some(cnt) = count {
        println!("FAILED (at the event #{})\nSOURCE:", cnt + 1);
    }

    let mut file = File::open(name)?;
    let mut data = vec![0; BUFFER_SIZE];
    let mut total_size = 0;

    loop {
        let data_size = file.read(&mut data)?;
        if data_size == 0 {
            break;
        }
        io::stdout().write_all(&data[..data_size])?;
        total_size += data_size;
    }

    println!("#### (length: {})", total_size);
    println!("OUTPUT:\n{}#### (length: {})", String::from_utf8_lossy(buffer), size);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().collect::<Vec<_>>();
    let mut canonical = false;
    let mut unicode = false;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-c" => {
                canonical = true;
                args.remove(i);
            }
            "-u" => {
                unicode = true;
                args.remove(i);
            }
            opt if opt.starts_with('-') => {
                println!("Unknown option: '{}'", opt);
                return Ok(());
            }
            _ => i += 1,
        }
    }

    if args.len() < 2 {
        println!("Usage: {} [-c] [-u] file1.yaml ...", args[0]);
        return Ok(());
    }

    for (number, filename) in args.iter().skip(1).enumerate() {
        let mut buffer = vec![0; BUFFER_SIZE];
        let mut events = Vec::with_capacity(MAX_EVENTS);
        let mut written = 0;
        let mut error = false;

        print!("[{}] Parsing, emitting, and parsing again '{}': ", number + 1, filename);
        io::stdout().flush()?;

        let yaml_str = std::fs::read_to_string(filename)?;
        let docs = YamlLoader::load_from_str(&yaml_str)?;

        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            if canonical {
                emitter.canonical = true;
            }
            if unicode {
                emitter.unicode = true;
            }
            for doc in &docs {
                emitter.dump(doc)?;
            }
        }
        written = out_str.len();
        buffer[..written].copy_from_slice(out_str.as_bytes());

        if !error {
            let new_docs = YamlLoader::load_from_str(&out_str)?;
            if docs != new_docs {
                print_output(filename, &buffer, written, Some(0))?;
                error = true;
            }
        }

        if !error {
            println!("PASSED (length: {})", written);
            print_output(filename, &buffer, written, None)?;
        }
    }

    Ok(())
}