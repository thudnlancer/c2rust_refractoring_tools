use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;
use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;
use std::str;

#[derive(Debug)]
struct Exclude {
    patterns: Vec<String>,
    regexes: Vec<Regex>,
    hash_table: HashSet<String>,
    options: i32,
}

impl Exclude {
    fn new() -> Self {
        Exclude {
            patterns: Vec::new(),
            regexes: Vec::new(),
            hash_table: HashSet::new(),
            options: 0,
        }
    }

    fn add_pattern(&mut self, pattern: &str, options: i32) {
        if options & (1 << 27) != 0 || options & (1 << 28) != 0 {
            if Self::pattern_has_wildcards(pattern, options) {
                if options & (1 << 27) != 0 {
                    let regex = Self::compile_regex(pattern, options);
                    self.regexes.push(regex);
                } else {
                    self.patterns.push(pattern.to_string());
                }
            }
        } else {
            let mut pattern = pattern.to_string();
            if options & (1 << 28) != 0 && options & (1 << 1) == 0 {
                pattern = Self::unescape_pattern(&pattern);
            }
            self.hash_table.insert(pattern);
        }
    }

    fn pattern_has_wildcards(pattern: &str, options: i32) -> bool {
        let mut chars = pattern.chars();
        while let Some(c) = chars.next() {
            match c {
                '.' | '{' | '}' | '(' | ')' if options & (1 << 27) != 0 => return true,
                '\\' if options & (1 << 27) != 0 => {
                    if chars.next().is_some() {
                        continue;
                    }
                }
                '+' | '@' | '!' if options & (1 << 5) != 0 => {
                    if chars.next() == Some('(') {
                        return true;
                    }
                }
                '?' | '*' | '[' => return true,
                _ => {}
            }
        }
        false
    }

    fn unescape_pattern(pattern: &str) -> String {
        let mut result = String::new();
        let mut chars = pattern.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.next() {
                    result.push(next);
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    fn compile_regex(pattern: &str, options: i32) -> Regex {
        let mut pattern = pattern.to_string();
        let case_insensitive = options & (1 << 4) != 0;
        
        if options & (1 << 3) != 0 {
            let len = pattern.trim_end_matches('/').len();
            if len == 0 {
                pattern = String::new();
            } else {
                pattern = format!("{}(/.*)?", &pattern[..len]);
            }
        }

        let mut builder = regex::RegexBuilder::new(&pattern);
        builder.case_insensitive(case_insensitive);
        builder.build().unwrap()
    }

    fn excluded_file_name(&self, filename: &str) -> bool {
        if self.hash_table.contains(filename) {
            return true;
        }

        for pattern in &self.patterns {
            if Self::fnmatch(pattern, filename, self.options) {
                return true;
            }
        }

        for regex in &self.regexes {
            if regex.is_match(filename) {
                return true;
            }
        }

        false
    }

    fn fnmatch(pattern: &str, filename: &str, options: i32) -> bool {
        if options & (1 << 3) == 0 {
            if options & (1 << 4) != 0 {
                pattern.to_lowercase() == filename.to_lowercase()
            } else {
                pattern == filename
            }
        } else if options & (1 << 4) == 0 {
            let patlen = pattern.len();
            if filename.starts_with(pattern) {
                let remaining = &filename[patlen..];
                remaining.is_empty() || remaining == "/"
            } else {
                false
            }
        } else {
            let parts: Vec<&str> = filename.split('/').collect();
            parts.iter().any(|part| {
                pattern.to_lowercase() == part.to_lowercase()
            })
        }
    }

    fn add_exclude_file(
        &mut self,
        file_name: &str,
        options: i32,
        line_end: char,
    ) -> std::io::Result<()> {
        let file = if file_name == "-" {
            Box::new(std::io::stdin()) as Box<dyn Read>
        } else {
            Box::new(File::open(file_name)?) as Box<dyn Read>
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let line = line.trim_end_matches(line_end);
            if !line.is_empty() {
                self.add_pattern(line, options);
            }
        }

        Ok(())
    }
}