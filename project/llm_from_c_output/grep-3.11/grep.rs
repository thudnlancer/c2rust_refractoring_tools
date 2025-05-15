use std::{
    ffi::{OsStr, OsString},
    fs::{self, File},
    io::{self, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process,
};

mod grep {
    use super::*;

    pub struct GrepOptions {
        pub ignore_case: bool,
        pub word_regexp: bool,
        pub line_regexp: bool,
        pub eol_byte: u8,
    }

    impl Default for GrepOptions {
        fn default() -> Self {
            Self {
                ignore_case: false,
                word_regexp: false,
                line_regexp: false,
                eol_byte: b'\n',
            }
        }
    }

    pub struct Grep {
        options: GrepOptions,
        patterns: Vec<String>,
    }

    impl Grep {
        pub fn new(options: GrepOptions, patterns: Vec<String>) -> Self {
            Self { options, patterns }
        }

        pub fn search_file(&self, path: &Path) -> io::Result<bool> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            for line in reader.split(self.options.eol_byte) {
                let line = line?;
                if self.matches(&line) {
                    return Ok(true);
                }
            }

            Ok(false)
        }

        fn matches(&self, line: &[u8]) -> bool {
            for pattern in &self.patterns {
                if self.match_pattern(line, pattern) {
                    return true;
                }
            }
            false
        }

        fn match_pattern(&self, line: &[u8], pattern: &str) -> bool {
            let line_str = String::from_utf8_lossy(line);
            let mut pat = pattern.to_string();

            if self.options.ignore_case {
                pat = pat.to_lowercase();
                let line_lower = line_str.to_lowercase();
                return line_lower.contains(&pat);
            }

            line_str.contains(&pat)
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} PATTERN [FILE...]", args[0]);
        process::exit(1);
    }

    let pattern = &args[1];
    let files = if args.len() > 2 {
        &args[2..]
    } else {
        &["-".to_string()]
    };

    let options = grep::GrepOptions::default();
    let grep = grep::Grep::new(options, vec![pattern.clone()]);

    let mut found = false;
    for file in files {
        if file == "-" {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buffer = Vec::new();
            handle.read_to_end(&mut buffer).unwrap();
            if grep.matches(&buffer) {
                println!("(standard input)");
                found = true;
            }
        } else {
            let path = Path::new(file);
            if let Ok(matched) = grep.search_file(path) {
                if matched {
                    println!("{}", path.display());
                    found = true;
                }
            }
        }
    }

    if !found {
        process::exit(1);
    }
}