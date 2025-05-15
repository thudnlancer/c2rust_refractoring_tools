use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug)]
enum ExcludeType {
    Hash,
    Pattern,
}

#[derive(Debug)]
struct ExcludeSegment {
    next: Option<Box<ExcludeSegment>>,
    exclude_type: ExcludeType,
    options: c_int,
    data: ExcludeData,
}

#[derive(Debug)]
enum ExcludeData {
    HashTable(HashSet<String>),
    Pattern(Vec<PatternOptions>),
}

#[derive(Debug)]
struct PatternOptions {
    options: c_int,
    pattern: String,
    regex: Option<Regex>,
}

#[derive(Debug)]
struct Exclude {
    head: Option<Box<ExcludeSegment>>,
    patbuf: Vec<String>,
}

impl Exclude {
    fn new() -> Self {
        Exclude {
            head: None,
            patbuf: Vec::new(),
        }
    }

    fn add_pattern_buffer(&mut self, buf: String) {
        self.patbuf.push(buf);
    }

    fn add_exclude(&mut self, pattern: &str, options: c_int) {
        if (options & ((1 << 27) | (1 << 28)) != 0 && Self::has_wildcards(pattern, options) {
            self.add_pattern_exclude(pattern, options);
        } else {
            self.add_hash_exclude(pattern, options);
        }
    }

    fn has_wildcards(pattern: &str, options: c_int) -> bool {
        let mut chars = pattern.chars();
        while let Some(c) = chars.next() {
            match c {
                '.' | '{' | '}' | '(' | ')' if (options & (1 << 27)) != 0 => return true,
                '\\' if (options & (1 << 27)) == 0 => {
                    if chars.next().is_some() {
                        continue;
                    }
                }
                '+' | '@' | '!' if (options & (1 << 5)) != 0 && chars.next() == Some('(') => {
                    return true
                }
                '?' | '*' | '[' => return true,
                _ => {}
            }
        }
        false
    }

    fn add_pattern_exclude(&mut self, pattern: &str, options: c_int) {
        let seg = if let Some(seg) = &mut self.head {
            if matches!(seg.exclude_type, ExcludeType::Pattern)
                && (seg.options & (1 << 29)) == (options & (1 << 29))
            {
                seg
            } else {
                let new_seg = Box::new(ExcludeSegment {
                    next: self.head.take(),
                    exclude_type: ExcludeType::Pattern,
                    options,
                    data: ExcludeData::Pattern(Vec::new()),
                });
                self.head = Some(new_seg);
                self.head.as_mut().unwrap()
            }
        } else {
            let new_seg = Box::new(ExcludeSegment {
                next: None,
                exclude_type: ExcludeType::Pattern,
                options,
                data: ExcludeData::Pattern(Vec::new()),
            });
            self.head = Some(new_seg);
            self.head.as_mut().unwrap()
        };

        if let ExcludeData::Pattern(patterns) = &mut seg.data {
            let mut pat = PatternOptions {
                options,
                pattern: String::new(),
                regex: None,
            };

            if (options & (1 << 27)) != 0 {
                let cflags = (1 << 3) | 1 | if (options & (1 << 4)) != 0 { 1 << 1 } else { 0 };
                let pat_str = if (options & (1 << 3)) != 0 {
                    let trimmed = pattern.trim_end_matches('/');
                    if trimmed.is_empty() {
                        return;
                    }
                    format!("{}(/.*)?", trimmed)
                } else {
                    pattern.to_string()
                };
                pat.regex = Regex::new(&pat_str).ok();
                if pat.regex.is_none() {
                    return;
                }
            } else {
                pat.pattern = if (options & (1 << 26)) != 0 {
                    let s = pattern.to_string();
                    self.add_pattern_buffer(s.clone());
                    s
                } else {
                    pattern.to_string()
                };
            }
            patterns.push(pat);
        }
    }

    fn add_hash_exclude(&mut self, pattern: &str, options: c_int) {
        let exclude_hash_flags = (1 << 29) | (1 << 30) | (1 << 3) | (1 << 4);
        let seg = if let Some(seg) = &mut self.head {
            if matches!(seg.exclude_type, ExcludeType::Hash)
                && (seg.options & exclude_hash_flags) == (options & exclude_hash_flags)
            {
                seg
            } else {
                let new_seg = Box::new(ExcludeSegment {
                    next: self.head.take(),
                    exclude_type: ExcludeType::Hash,
                    options,
                    data: ExcludeData::HashTable(HashSet::new()),
                });
                self.head = Some(new_seg);
                self.head.as_mut().unwrap()
            }
        } else {
            let new_seg = Box::new(ExcludeSegment {
                next: None,
                exclude_type: ExcludeType::Hash,
                options,
                data: ExcludeData::HashTable(HashSet::new()),
            });
            self.head = Some(new_seg);
            self.head.as_mut().unwrap()
        };

        if let ExcludeData::HashTable(table) = &mut seg.data {
            let mut s = pattern.to_string();
            if (options & ((1 << 28) | (1 << 1))) == (1 << 28) {
                s = s.replace("\\", "");
            }
            table.insert(s);
        }
    }

    fn excluded_file_name(&self, f: &str) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut seg = self.head.as_ref();
        let mut invert = false;

        while let Some(current_seg) = seg {
            match &current_seg.data {
                ExcludeData::HashTable(table) => {
                    if Self::file_name_matches(table, f, current_seg.options) {
                        break;
                    }
                }
                ExcludeData::Pattern(patterns) => {
                    if Self::file_pattern_matches(patterns, f) {
                        break;
                    }
                }
            }

            if current_seg.next.is_none() {
                invert = true;
                break;
            }
            seg = current_seg.next.as_ref();
        }

        invert ^ (seg.unwrap().options & (1 << 29) == 0)
    }

    fn file_name_matches(table: &HashSet<String>, f: &str, options: c_int) -> bool {
        let mut buffer = f.to_string();
        loop {
            if table.contains(&buffer) {
                return true;
            }

            if (options & (1 << 3)) == 0 {
                break;
            }

            if let Some(pos) = buffer.rfind('/') {
                buffer.truncate(pos);
            } else {
                break;
            }
        }

        if (options & (1 << 30)) != 0 {
            return false;
        }

        let mut remaining = f;
        while let Some(pos) = remaining.find('/') {
            remaining = &remaining[pos + 1..];
            if Self::file_name_matches(table, remaining, options) {
                return true;
            }
        }

        false
    }

    fn file_pattern_matches(patterns: &[PatternOptions], f: &str) -> bool {
        patterns.iter().any(|pat| {
            if let Some(re) = &pat.regex {
                re.is_match(f)
            } else {
                Self::exclude_fnmatch(&pat.pattern, f, pat.options)
            }
        })
    }

    fn exclude_fnmatch(pattern: &str, f: &str, options: c_int) -> bool {
        let matched = if (options & (1 << 28)) != 0 {
            Self::fnmatch(pattern, f, options)
        } else {
            Self::fnmatch_no_wildcards(pattern, f, options)
        };

        if (options & (1 << 30)) != 0 {
            matched
        } else {
            matched || f.split('/').skip(1).any(|part| {
                if (options & (1 << 28)) != 0 {
                    Self::fnmatch(pattern, part, options)
                } else {
                    Self::fnmatch_no_wildcards(pattern, part, options)
                }
            })
        }
    }

    fn fnmatch(pattern: &str, f: &str, options: c_int) -> bool {
        // Simplified fnmatch implementation
        pattern == f || (options & (1 << 4) != 0 && pattern.eq_ignore_ascii_case(f))
    }

    fn fnmatch_no_wildcards(pattern: &str, f: &str, options: c_int) -> bool {
        if (options & (1 << 3)) == 0 {
            if (options & (1 << 4)) != 0 {
                pattern.eq_ignore_ascii_case(f)
            } else {
                pattern == f
            }
        } else if (options & (1 << 4)) == 0 {
            let patlen = pattern.len();
            if f.len() >= patlen && &f[..patlen] == pattern {
                if f.len() == patlen || &f[patlen..patlen + 1] == "/" {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            let parts = f.split('/');
            parts.any(|part| pattern.eq_ignore_ascii_case(part))
        }
    }

    fn add_exclude_file(
        &mut self,
        file_name: &str,
        options: c_int,
        line_end: char,
    ) -> std::io::Result<()> {
        let file = if file_name == "-" {
            Box::new(std::io::stdin()) as Box<dyn BufRead>
        } else {
            Box::new(BufReader::new(File::open(file_name)?))
        };

        for line in file.lines() {
            let line = line?;
            let trimmed = line.trim_end_matches(|c| c == line_end || c.is_whitespace());
            if !trimmed.is_empty() {
                self.add_exclude(trimmed, options);
            }
        }

        Ok(())
    }
}