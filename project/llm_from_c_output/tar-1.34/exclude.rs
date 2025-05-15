use std::collections::HashMap;
use std::ffi::{CString, OsStr};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::ptr;
use std::str;
use regex::Regex;
use fnmatch::fnmatch;
use lazy_static::lazy_static;
use std::os::unix::ffi::OsStrExt;

lazy_static! {
    static ref EXCLUDE_ANCHORED: u32 = 1 << 30;
    static ref EXCLUDE_INCLUDE: u32 = 1 << 29;
    static ref EXCLUDE_WILDCARDS: u32 = 1 << 28;
    static ref EXCLUDE_REGEX: u32 = 1 << 27;
    static ref EXCLUDE_ALLOC: u32 = 1 << 26;
}

struct PatOpts {
    options: u32,
    pattern: Option<String>,
    regex: Option<Regex>,
}

struct ExcludePattern {
    exclude: Vec<PatOpts>,
}

enum ExcludeType {
    Hash,
    Pattern,
}

struct ExcludeSegment {
    next: Option<Box<ExcludeSegment>>,
    type_: ExcludeType,
    options: u32,
    table: Option<HashMap<String, bool>>,
    pat: Option<ExcludePattern>,
}

struct PatternBuffer {
    next: Option<Box<PatternBuffer>>,
    base: String,
}

struct Exclude {
    head: Option<Box<ExcludeSegment>>,
    patbuf: Option<Box<PatternBuffer>>,
}

impl Exclude {
    fn new() -> Self {
        Exclude {
            head: None,
            patbuf: None,
        }
    }

    fn add_pattern_buffer(&mut self, buf: String) {
        let pbuf = Box::new(PatternBuffer {
            next: self.patbuf.take(),
            base: buf,
        });
        self.patbuf = Some(pbuf);
    }

    fn fnmatch_pattern_has_wildcards(s: &str, options: u32) -> bool {
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '.' | '{' | '}' | '(' | ')' if options & *EXCLUDE_REGEX != 0 => return true,
                '\\' if options & *EXCLUDE_REGEX == 0 => {
                    if options & fnmatch::FNM_NOESCAPE == 0 && chars.peek().is_some() {
                        chars.next();
                    }
                }
                '+' | '@' | '!' if options & fnmatch::FNM_EXTMATCH != 0 && chars.peek() == Some(&'(') => {
                    return true;
                }
                '?' | '*' | '[' => return true,
                _ => {}
            }
        }
        false
    }

    fn unescape_pattern(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' && chars.peek().is_some() {
                result.push(chars.next().unwrap());
            } else {
                result.push(c);
            }
        }
        result
    }

    fn new_exclude_segment(&mut self, type_: ExcludeType, options: u32) {
        let seg = Box::new(ExcludeSegment {
            next: self.head.take(),
            type_,
            options,
            table: match type_ {
                ExcludeType::Hash => Some(HashMap::new()),
                _ => None,
            },
            pat: match type_ {
                ExcludeType::Pattern => Some(ExcludePattern { exclude: Vec::new() }),
                _ => None,
            },
        });
        self.head = Some(seg);
    }

    fn add_exclude(&mut self, pattern: &str, options: u32) {
        if (options & (*EXCLUDE_REGEX | *EXCLUDE_WILDCARDS) != 0)
            && Self::fnmatch_pattern_has_wildcards(pattern, options)
        {
            if !(self.head.is_some()
                && matches!(self.head.as_ref().unwrap().type_, ExcludeType::Pattern)
                && (self.head.as_ref().unwrap().options & *EXCLUDE_INCLUDE)
                    == (options & *EXCLUDE_INCLUDE))
            {
                self.new_exclude_segment(ExcludeType::Pattern, options);
            }

            let head = self.head.as_mut().unwrap();
            if let Some(pat) = &mut head.pat {
                let patopts = PatOpts {
                    options,
                    pattern: if options & *EXCLUDE_REGEX == 0 {
                        if options & *EXCLUDE_ALLOC != 0 {
                            let s = pattern.to_string();
                            self.add_pattern_buffer(s.clone());
                            Some(s)
                        } else {
                            Some(pattern.to_string())
                        }
                    } else {
                        None
                    },
                    regex: if options & *EXCLUDE_REGEX != 0 {
                        let cflags = if options & fnmatch::FNM_CASEFOLD != 0 {
                            regex::RegexBuilder::new(pattern)
                                .case_insensitive(true)
                                .build()
                                .ok()
                        } else {
                            Regex::new(pattern).ok()
                        };
                        cflags
                    } else {
                        None
                    },
                };
                pat.exclude.push(patopts);
            }
        } else {
            let exclude_hash_flags = *EXCLUDE_INCLUDE | *EXCLUDE_ANCHORED | fnmatch::FNM_LEADING_DIR | fnmatch::FNM_CASEFOLD;
            if !(self.head.is_some()
                && matches!(self.head.as_ref().unwrap().type_, ExcludeType::Hash)
                && (self.head.as_ref().unwrap().options & exclude_hash_flags)
                    == (options & exclude_hash_flags))
            {
                self.new_exclude_segment(ExcludeType::Hash, options);
            }

            let head = self.head.as_mut().unwrap();
            if let Some(table) = &mut head.table {
                let s = if (options & (*EXCLUDE_WILDCARDS | fnmatch::FNM_NOESCAPE)) == *EXCLUDE_WILDCARDS {
                    Self::unescape_pattern(pattern)
                } else {
                    pattern.to_string()
                };
                table.insert(s, true);
            }
        }
    }

    fn excluded_file_name(&self, f: &str) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut invert = false;
        let mut filename = None;
        let mut seg = self.head.as_ref();

        while let Some(current) = seg {
            match current.type_ {
                ExcludeType::Hash => {
                    if filename.is_none() {
                        filename = Some(f.to_string());
                    }
                    if Self::file_name_matches(current, f, filename.as_mut().unwrap()) {
                        break;
                    }
                }
                ExcludeType::Pattern => {
                    if Self::file_pattern_matches(current, f) {
                        break;
                    }
                }
            }

            seg = current.next.as_ref();
            if seg.is_none() {
                invert = true;
                break;
            }
        }

        invert ^ !(seg.unwrap().options & *EXCLUDE_INCLUDE != 0)
    }

    fn file_pattern_matches(seg: &ExcludeSegment, f: &str) -> bool {
        if let Some(pat) = &seg.pat {
            for patopts in &pat.exclude {
                if Self::exclude_patopts(patopts, f) {
                    return true;
                }
            }
        }
        false
    }

    fn file_name_matches(seg: &ExcludeSegment, f: &str, buffer: &mut String) -> bool {
        let options = seg.options;
        if let Some(table) = &seg.table {
            let mut f = f;
            loop {
                *buffer = f.to_string();
                loop {
                    if table.contains_key(buffer) {
                        return true;
                    }
                    if options & fnmatch::FNM_LEADING_DIR != 0 {
                        if let Some(p) = buffer.rfind('/') {
                            buffer.truncate(p);
                            continue;
                        }
                    }
                    break;
                }

                if options & *EXCLUDE_ANCHORED == 0 {
                    if let Some(p) = f.find('/') {
                        f = &f[p + 1..];
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        false
    }

    fn exclude_patopts(opts: &PatOpts, f: &str) -> bool {
        let options = opts.options;
        if options & *EXCLUDE_REGEX != 0 {
            if let Some(re) = &opts.regex {
                re.is_match(f)
            } else {
                false
            }
        } else if let Some(pattern) = &opts.pattern {
            Self::exclude_fnmatch(pattern, f, options)
        } else {
            false
        }
    }

    fn exclude_fnmatch(pattern: &str, f: &str, options: u32) -> bool {
        let matcher = if options & *EXCLUDE_WILDCARDS != 0 {
            fnmatch::fnmatch
        } else {
            Self::fnmatch_no_wildcards
        };
        let mut matched = matcher(pattern, f, options as i32) == 0;
        if options & *EXCLUDE_ANCHORED == 0 {
            for p in f.split('/') {
                if matched {
                    break;
                }
                matched = matcher(pattern, p, options as i32) == 0;
            }
        }
        matched
    }

    fn fnmatch_no_wildcards(pattern: &str, f: &str, options: i32) -> i32 {
        if options & fnmatch::FNM_LEADING_DIR == 0 {
            if options & fnmatch::FNM_CASEFOLD != 0 {
                if pattern.to_lowercase() == f.to_lowercase() {
                    0
                } else {
                    1
                }
            } else if pattern == f {
                0
            } else {
                1
            }
        } else if options & fnmatch::FNM_CASEFOLD == 0 {
            let patlen = pattern.len();
            if f.len() >= patlen && &f[..patlen] == pattern {
                if f.len() == patlen || f.as_bytes()[patlen] == b'/' {
                    0
                } else {
                    1
                }
            } else {
                1
            }
        } else {
            let pattern_lower = pattern.to_lowercase();
            let mut f_lower = f.to_lowercase();
            for i in 0..=f.len() {
                if i < f.len() {
                    if f.as_bytes()[i] == b'/' {
                        f_lower.truncate(i);
                        if pattern_lower == f_lower {
                            return 0;
                        }
                        f_lower = f.to_lowercase();
                    }
                } else {
                    if pattern_lower == f_lower {
                        return 0;
                    }
                }
            }
            1
        }
    }
}

fn add_exclude_file(
    add_func: fn(&mut Exclude, &str, u32),
    ex: &mut Exclude,
    file_name: &str,
    options: u32,
    line_end: char,
) -> io::Result<()> {
    let use_stdin = file_name == "-";
    let in_file: Box<dyn Read> = if use_stdin {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(file_name)?)
    };
    let reader = BufReader::new(in_file);
    let mut buf = String::new();

    for line in reader.lines() {
        let line = line?;
        if line_end == ' ' {
            let trimmed = line.trim_end();
            if trimmed.is_empty() {
                continue;
            }
            add_func(ex, trimmed, options);
        } else {
            for pattern in line.split(line_end) {
                if !pattern.is_empty() {
                    add_func(ex, pattern, options);
                }
            }
        }
    }

    Ok(())
}