use std::cmp::Ordering;
use std::ffi::CStr;
use std::fs::File;
use std::io::{self, Read, Write};
use std::mem;
use std::path::Path;
use std::ptr;
use std::str;

use yaml_rust::{yaml::YamlLoader, YamlEmitter};

const BUFFER_SIZE: usize = 65536;
const MAX_DOCUMENTS: usize = 16;

fn copy_document(document_to: &mut yaml_rust::Yaml, document_from: &yaml_rust::Yaml) -> bool {
    match document_from {
        Yaml::Hash(hash_from) => {
            let mut hash_to = yaml_rust::yaml::Hash::new();
            for (k, v) in hash_from {
                let key = copy_document(&mut Yaml::Null, k)?;
                let value = copy_document(&mut Yaml::Null, v)?;
                hash_to.insert(key, value);
            }
            *document_to = Yaml::Hash(hash_to);
        }
        Yaml::Array(arr_from) => {
            let mut arr_to = Vec::new();
            for item in arr_from {
                let copied = copy_document(&mut Yaml::Null, item)?;
                arr_to.push(copied);
            }
            *document_to = Yaml::Array(arr_to);
        }
        Yaml::String(s) => {
            *document_to = Yaml::String(s.clone());
        }
        Yaml::Integer(i) => {
            *document_to = Yaml::Integer(*i);
        }
        Yaml::Real(r) => {
            *document_to = Yaml::Real(r.clone());
        }
        Yaml::Boolean(b) => {
            *document_to = Yaml::Boolean(*b);
        }
        Yaml::Null => {
            *document_to = Yaml::Null;
        }
        _ => return false,
    }
    true
}

fn compare_nodes(node1: &yaml_rust::Yaml, node2: &yaml_rust::Yaml, level: usize) -> bool {
    if level > 1000 {
        return false;
    }

    match (node1, node2) {
        (Yaml::Hash(h1), Yaml::Hash(h2)) => {
            if h1.len() != h2.len() {
                return false;
            }
            for (k1, v1) in h1 {
                if let Some(v2) = h2.get(k1) {
                    if !compare_nodes(v1, v2, level + 1) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        }
        (Yaml::Array(a1), Yaml::Array(a2)) => {
            if a1.len() != a2.len() {
                return false;
            }
            for (i1, i2) in a1.iter().zip(a2.iter()) {
                if !compare_nodes(i1, i2, level + 1) {
                    return false;
                }
            }
            true
        }
        (Yaml::String(s1), Yaml::String(s2)) => s1 == s2,
        (Yaml::Integer(i1), Yaml::Integer(i2)) => i1 == i2,
        (Yaml::Real(r1), Yaml::Real(r2)) => r1 == r2,
        (Yaml::Boolean(b1), Yaml::Boolean(b2)) => b1 == b2,
        (Yaml::Null, Yaml::Null) => true,
        _ => false,
    }
}

fn compare_documents(doc1: &[yaml_rust::Yaml], doc2: &[yaml_rust::Yaml]) -> bool {
    if doc1.len() != doc2.len() {
        return false;
    }

    for (d1, d2) in doc1.iter().zip(doc2.iter()) {
        if !compare_nodes(d1, d2, 0) {
            return false;
        }
    }
    true
}

fn print_output(name: &str, buffer: &[u8], size: usize, count: Option<usize>) -> io::Result<()> {
    if let Some(cnt) = count {
        println!("FAILED (at the document #{})\nSOURCE:", cnt + 1);
    }

    let mut file = File::open(name)?;
    let mut total_size = 0;
    let mut data = vec![0; BUFFER_SIZE];

    loop {
        let data_size = file.read(&mut data)?;
        if data_size == 0 {
            break;
        }
        io::stdout().write_all(&data[..data_size])?;
        total_size += data_size;
    }

    println!("#### (length: {})", total_size);
    println!("OUTPUT:\n{}#### (length: {})", 
        str::from_utf8(buffer).unwrap_or("[INVALID UTF8]"), 
        size
    );
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

    for (number, filename) in args.iter().enumerate().skip(1) {
        println!("[{}] Loading, dumping, and loading again '{}': ", number, filename);
        io::stdout().flush()?;

        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let docs = match YamlLoader::load_from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                println!("FAILED to parse: {}", e);
                continue;
            }
        };

        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        if canonical {
            emitter.canonical = true;
        }
        if unicode {
            emitter.unicode = true;
        }

        let mut dumped_docs = Vec::with_capacity(docs.len());
        for doc in &docs {
            emitter.dump(doc).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Emitter error: {}", e))
            })?;
            let mut copied = Yaml::Null;
            if !copy_document(&mut copied, doc) {
                println!("FAILED to copy document");
                continue;
            }
            dumped_docs.push(copied);
        }

        let reloaded_docs = match YamlLoader::load_from_str(&out_str) {
            Ok(d) => d,
            Err(e) => {
                println!("FAILED to reload: {}", e);
                print_output(filename, out_str.as_bytes(), out_str.len(), None)?;
                continue;
            }
        };

        if !compare_documents(&dumped_docs, &reloaded_docs) {
            println!("FAILED comparison");
            print_output(filename, out_str.as_bytes(), out_str.len(), None)?;
            continue;
        }

        println!("PASSED (length: {})", out_str.len());
        print_output(filename, out_str.as_bytes(), out_str.len(), None)?;
    }

    Ok(())
}