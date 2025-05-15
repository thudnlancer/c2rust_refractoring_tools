use std::collections::HashMap;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::ptr;
use std::str;
use regex::Regex;
use fnmatch::fnmatch;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref EXCLUDE_MUTEX: Mutex<()> = Mutex::new(());
}

const EXCLUDE_ANCHORED: i32 = 1 << 30;
const EXCLUDE_INCLUDE: i32 = 1 << 29;
const EXCLUDE_WILDCARDS: i32 = 1 << 28;
const EXCLUDE_REGEX: i32 = 1 << 27;
const EXCLUDE_ALLOC: i32 = 1 << 26;

#[derive(Debug)]
enum ExcludeType {
    Hash,
    Pattern,
}

#[derive(Debug)]
struct PatOpts {
    options: i32,
    pattern: Option<String>,
    regex: Option<Regex>,
}

#[derive(Debug)]
struct ExcludePattern {
    exclude: Vec<PatOpts>,
}

#[derive(Debug)]
struct ExcludeSegment {
    next: Option<Box<ExcludeSegment>>,
    exclude_type: ExcludeType,
    options: i32,
    table: Option<HashMap<String, ()>>,
    pattern: Option<ExcludePattern>,
}

#[derive(Debug)]
struct PatternBuffer {
    next: Option<Box<PatternBuffer>>,
    base: String,
}

#[derive(Debug)]
pub struct Exclude {
    head: Option<Box<ExcludeSegment>>,
    patbuf: Option<Box<PatternBuffer>>,
}

impl Exclude {
    pub fn new() -> Self {
        Exclude {
            head: None,
            patbuf: None,
        }
    }

    pub fn add_pattern_buffer(&mut self, buf: String) {
        let pbuf = Box::new(PatternBuffer {
            next: self.patbuf.take(),
            base: buf,
        });
        self.patbuf = Some(pbuf);
    }

    pub fn free(&mut self) {
        let _lock = EXCLUDE_MUTEX.lock().unwrap();
        self.head = None;
        self.patbuf = None;
    }

    fn new_exclude_segment(&mut self, exclude_type: ExcludeType, options: i32) {
        let mut seg = Box::new(ExcludeSegment {
            next: self.head.take(),
            exclude_type,
            options,
            table: None,
            pattern: None,
        });

        match seg.exclude_type {
            ExcludeType::Hash => {
                seg.table = Some(HashMap::new());
            }
            ExcludeType::Pattern => {
                seg.pattern = Some(ExcludePattern { exclude: Vec::new() });
            }
        }

        self.head = Some(seg);
    }

    pub fn add_exclude(&mut self, pattern: &str, options: i32) {
        if (options & (EXCLUDE_REGEX | EXCLUDE_WILDCARDS)) != 0
            && Self::fnmatch_pattern_has_wildcards(pattern, options)
        {
            if !(self.head.is_some()
                && matches!(self.head.as_ref().unwrap().exclude_type, ExcludeType::Pattern)
                && ((self.head.as_ref().unwrap().options & EXCLUDE_INCLUDE)
                    == (options & EXCLUDE_INCLUDE)))
            {
                self.new_exclude_segment(ExcludeType::Pattern, options);
            }

            let head = self.head.as_mut().unwrap();
            if let Some(pat) = &mut head.pattern {
                let mut patopts = PatOpts {
                    options,
                    pattern: None,
                    regex: None,
                };

                if (options & EXCLUDE_REGEX) != 0 {
                    let cflags = if (options & FNM_CASEFOLD) != 0 {
                        regex::RegexBuilder::new(pattern)
                            .case_insensitive(true)
                            .build()
                    } else {
                        Regex::new(pattern)
                    };

                    if let Ok(re) = cflags {
                        patopts.regex = Some(re);
                        pat.exclude.push(patopts);
                    }
                } else {
                    if (options & EXCLUDE_ALLOC) != 0 {
                        let pattern = pattern.to_string();
                        self.add_pattern_buffer(pattern.clone());
                        patopts.pattern = Some(pattern);
                    } else {
                        patopts.pattern = Some(pattern.to_string());
                    }
                    pat.exclude.push(patopts);
                }
            }
        } else {
            let exclude_hash_flags = EXCLUDE_INCLUDE | EXCLUDE_ANCHORED | FNM_LEADING_DIR | FNM_CASEFOLD;
            if !(self.head.is_some()
                && matches!(self.head.as_ref().unwrap().exclude_type, ExcludeType::Hash)
                && ((self.head.as_ref().unwrap().options & exclude_hash_flags)
                    == (options & exclude_hash_flags)))
            {
                self.new_exclude_segment(ExcludeType::Hash, options);
            }

            let head = self.head.as_mut().unwrap();
            if let Some(table) = &mut head.table {
                let mut str = pattern.to_string();
                if (options & (EXCLUDE_WILDCARDS | FNM_NOESCAPE)) == EXCLUDE_WILDCARDS {
                    Self::unescape_pattern(&mut str);
                }
                table.insert(str, ());
            }
        }
    }

    fn unescape_pattern(pattern: &mut String) {
        let mut q = 0;
        let mut r = 0;
        let bytes = pattern.as_bytes();
        let len = bytes.len();
        let mut new_bytes = Vec::with_capacity(len);

        while q < len {
            if bytes[q] == b'\\' && q + 1 < len {
                q += 1;
            }
            new_bytes.push(bytes[q]);
            q += 1;
            r += 1;
        }

        *pattern = String::from_utf8(new_bytes).unwrap();
    }

    pub fn fnmatch_pattern_has_wildcards(pattern: &str, options: i32) -> bool {
        let bytes = pattern.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'.' | b'{' | b'}' | b'(' | b')' if (options & EXCLUDE_REGEX) != 0 => return true,
                b'\\' => {
                    if (options & EXCLUDE_REGEX) == 0 {
                        i += if (options & FNM_NOESCAPE) == 0 && i + 1 < bytes.len() {
                            1
                        } else {
                            0
                        };
                    }
                }
                b'+' | b'@' | b'!' if (options & FNM_EXTMATCH) != 0 && i + 1 < bytes.len() && bytes[i + 1] == b'(' => {
                    return true;
                }
                b'?' | b'*' | b'[' => return true,
                _ => (),
            }
            i += 1;
        }
        false
    }

    pub fn excluded_file_name(&self, f: &str) -> bool {
        if self.head.is_none() {
            return false;
        }

        let mut invert = false;
        let mut filename = None;
        let mut seg = self.head.as_ref();

        while let Some(segment) = seg {
            match segment.exclude_type {
                ExcludeType::Hash => {
                    if filename.is_none() {
                        filename = Some(f.to_string());
                    }
                    if Self::file_name_matches(segment, f, filename.as_mut().unwrap()) {
                        break;
                    }
                }
                ExcludeType::Pattern => {
                    if Self::file_pattern_matches(segment, f) {
                        break;
                    }
                }
            }

            if segment.next.is_none() {
                invert = true;
                break;
            }
            seg = segment.next.as_ref();
        }

        invert ^ !(seg.unwrap().options & EXCLUDE_INCLUDE != 0)
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
                    if (options & FNM_LEADING_DIR) != 0 {
                        if let Some(p) = buffer.rfind('/') {
                            buffer.truncate(p);
                            continue;
                        }
                    }
                    break;
                }

                if (options & EXCLUDE_ANCHORED) == 0 {
                    if let Some(pos) = f.find('/') {
                        f = &f[pos + 1..];
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

    fn file_pattern_matches(seg: &ExcludeSegment, f: &str) -> bool {
        if let Some(pat) = &seg.pattern {
            for patopts in &pat.exclude {
                if Self::exclude_patopts(patopts, f) {
                    return true;
                }
            }
        }
        false
    }

    fn exclude_patopts(opts: &PatOpts, f: &str) -> bool {
        let options = opts.options;
        if (options & EXCLUDE_REGEX) != 0 {
            if let Some(re) = &opts.regex {
                re.is_match(f)
            } else {
                false
            }
        } else {
            Self::exclude_fnmatch(
                opts.pattern.as_ref().unwrap(),
                f,
                options,
            )
        }
    }

    fn exclude_fnmatch(pattern: &str, f: &str, options: i32) -> bool {
        let matcher = if (options & EXCLUDE_WILDCARDS) != 0 {
            fnmatch
        } else {
            Self::fnmatch_no_wildcards
        };

        let mut matched = matcher(pattern, f, options);
        if (options & EXCLUDE_ANCHORED) == 0 {
            let mut p = f;
            while !matched && !p.is_empty() {
                if let Some(pos) = p.find('/') {
                    if pos + 1 < p.len() {
                        matched = matcher(pattern, &p[pos + 1..], options);
                        p = &p[pos + 1..];
                        continue;
                    }
                }
                break;
            }
        }
        matched
    }

    fn fnmatch_no_wildcards(pattern: &str, f: &str, options: i32) -> bool {
        if (options & FNM_LEADING_DIR) == 0 {
            if (options & FNM_CASEFOLD) != 0 {
                pattern.to_lowercase() == f.to_lowercase()
            } else {
                pattern == f
            }
        } else if (options & FNM_CASEFOLD) == 0 {
            let patlen = pattern.len();
            if f.len() >= patlen && &f[..patlen] == pattern {
                if f.len() == patlen || f.as_bytes()[patlen] == b'/' {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            let mut matched = false;
            let mut fcopy = f.to_string();
            let mut p = 0;
            loop {
                if let Some(pos) = fcopy[p..].find('/') {
                    let end = p + pos;
                    fcopy.truncate(end);
                    if pattern.to_lowercase() == fcopy.to_lowercase() {
                        matched = true;
                        break;
                    }
                    fcopy = f.to_string();
                    p = end + 1;
                } else {
                    break;
                }
            }
            matched
        }
    }

    pub fn add_exclude_file(
        &mut self,
        file_name: &str,
        options: i32,
        line_end: char,
    ) -> io::Result<()> {
        if file_name == "-" {
            self.add_exclude_fp(io::stdin(), options, line_end)
        } else {
            let file = File::open(file_name)?;
            self.add_exclude_fp(file, options, line_end)
        }
    }

    fn add_exclude_fp<R: Read>(
        &mut self,
        fp: R,
        options: i32,
        line_end: char,
    ) -> io::Result<()> {
        let mut buf = Vec::new();
        let mut reader = BufReader::new(fp);
        reader.read_to_end(&mut buf)?;

        let mut pattern_start = 0;
        for (i, &c) in buf.iter().enumerate() {
            if c == line_end as u8 {
                let mut pattern_end = i;
                if line_end.is_whitespace() {
                    while pattern_end > pattern_start
                        && buf[pattern_end - 1].is_ascii_whitespace()
                    {
                        pattern_end -= 1;
                    }
                    if pattern_end == pattern_start {
                        pattern_start = i + 1;
                        continue;
                    }
                }

                buf[pattern_end] = b'\0';
                let pattern = unsafe {
                    CStr::from_ptr(buf[pattern_start..].as_ptr() as *const i8)
                        .to_str()
                        .unwrap()
                };
                self.add_exclude(pattern, options);
                pattern_start = i + 1;
            }
        }

        Ok(())
    }
}

impl Default for Exclude {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Exclude {
    fn drop(&mut self) {
        self.free();
    }
}