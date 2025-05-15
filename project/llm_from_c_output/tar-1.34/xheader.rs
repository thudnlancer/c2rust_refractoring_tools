use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ptr;
use std::mem;
use std::io::{self, Write};
use std::fs;
use std::os::unix::fs::MetadataExt;
use libc::{uid_t, gid_t, time_t, mode_t};
use nix::sys::stat::Mode;
use nix::unistd::{Uid, Gid};
use glob::Pattern;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_HEADER_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ref KEYWORD_PATTERN_LIST: Mutex<Vec<KeywordEntry>> = Mutex::new(Vec::new());
    static ref KEYWORD_GLOBAL_OVERRIDE_LIST: Mutex<Vec<KeywordEntry>> = Mutex::new(Vec::new());
    static ref KEYWORD_OVERRIDE_LIST: Mutex<Vec<KeywordEntry>> = Mutex::new(Vec::new());
    static ref GLOBAL_HEADER_OVERRIDE_LIST: Mutex<Vec<KeywordEntry>> = Mutex::new(Vec::new());
}

struct KeywordEntry {
    pattern: String,
    value: Option<String>,
}

struct XHeader {
    buffer: Vec<u8>,
    size: usize,
    stk: Option<Vec<u8>>,
    string_length: usize,
}

struct TarStatInfo {
    orig_file_name: PathBuf,
    file_name: PathBuf,
    link_name: Option<PathBuf>,
    stat: fs::Metadata,
    atime: SystemTime,
    ctime: SystemTime,
    mtime: SystemTime,
    gname: Option<String>,
    uname: Option<String>,
    xhdr: XHeader,
    had_trailing_slash: bool,
    sparse_name_done: bool,
    real_size_set: bool,
    real_size: u64,
    sparse_map: Vec<SparseEntry>,
    sparse_map_avail: usize,
    sparse_map_size: usize,
    sparse_major: u32,
    sparse_minor: u32,
    dumpdir: Option<Vec<u8>>,
    xattr_map: HashMap<String, Vec<u8>>,
    acls_a_ptr: Option<Vec<u8>>,
    acls_a_len: usize,
    acls_d_ptr: Option<Vec<u8>>,
    acls_d_len: usize,
    cntx_name: Option<String>,
}

struct SparseEntry {
    offset: u64,
    numbytes: u64,
}

impl XHeader {
    fn new() -> Self {
        XHeader {
            buffer: Vec::new(),
            size: 0,
            stk: None,
            string_length: 0,
        }
    }

    fn init(&mut self) {
        if self.stk.is_none() {
            self.stk = Some(Vec::new());
        }
    }

    fn grow(&mut self, data: &[u8]) {
        if let Some(ref mut stk) = self.stk {
            stk.extend_from_slice(data);
            self.size += data.len();
        }
    }

    fn grow_byte(&mut self, byte: u8) {
        if let Some(ref mut stk) = self.stk {
            stk.push(byte);
            self.size += 1;
        }
    }

    fn blank(&mut self, len: usize) {
        if let Some(ref mut stk) = self.stk {
            stk.resize(stk.len() + len, 0);
            self.size += len;
        }
    }

    fn finish(&mut self) {
        if let Some(stk) = self.stk.take() {
            self.buffer = stk;
        }
    }

    fn destroy(&mut self) {
        self.buffer.clear();
        self.stk = None;
        self.size = 0;
    }
}

impl TarStatInfo {
    fn new() -> Self {
        TarStatInfo {
            orig_file_name: PathBuf::new(),
            file_name: PathBuf::new(),
            link_name: None,
            stat: fs::metadata(".").unwrap(),
            atime: SystemTime::now(),
            ctime: SystemTime::now(),
            mtime: SystemTime::now(),
            gname: None,
            uname: None,
            xhdr: XHeader::new(),
            had_trailing_slash: false,
            sparse_name_done: false,
            real_size_set: false,
            real_size: 0,
            sparse_map: Vec::new(),
            sparse_map_avail: 0,
            sparse_map_size: 0,
            sparse_major: 0,
            sparse_minor: 0,
            dumpdir: None,
            xattr_map: HashMap::new(),
            acls_a_ptr: None,
            acls_a_len: 0,
            acls_d_ptr: None,
            acls_d_len: 0,
            cntx_name: None,
        }
    }
}

fn xheader_keyword_deleted_p(kw: &str) -> bool {
    let list = KEYWORD_PATTERN_LIST.lock().unwrap();
    for entry in list.iter() {
        if Pattern::new(&entry.pattern)
            .map(|p| p.matches(kw))
            .unwrap_or(false)
        {
            return true;
        }
    }
    false
}

fn xheader_keyword_override_p(keyword: &str) -> bool {
    let list = KEYWORD_OVERRIDE_LIST.lock().unwrap();
    list.iter().any(|e| e.pattern == keyword)
}

fn xheader_list_append(root: &Mutex<Vec<KeywordEntry>>, kw: &str, value: Option<&str>) {
    let mut list = root.lock().unwrap();
    list.push(KeywordEntry {
        pattern: kw.to_string(),
        value: value.map(|s| s.to_string()),
    });
}

fn xheader_list_destroy(root: &Mutex<Vec<KeywordEntry>>) {
    let mut list = root.lock().unwrap();
    list.clear();
}

fn xheader_set_single_keyword(kw: &str) -> ! {
    panic!("Keyword {} is unknown or not yet implemented", kw);
}

fn assign_time_option(sval: &mut Option<String>, tval: &mut time_t, input: &str) {
    if let Ok(t) = parse_timespec(input) {
        *tval = t.tv_sec;
        *sval = Some(input.to_string());
    } else {
        eprintln!("Time stamp is out of allowed range");
    }
}

fn parse_timespec(input: &str) -> Result<libc::timespec, ()> {
    // Simplified implementation - replace with actual parsing logic
    Ok(libc::timespec {
        tv_sec: input.parse().map_err(|_| ())?,
        tv_nsec: 0,
    })
}

fn xheader_set_keyword_equal(kw: &str, eq: &str) {
    let mut global = true;
    let mut p = eq;

    if eq.is_empty() {
        panic!("Malformed pax option: {}", kw);
    }

    if eq.ends_with(':') {
        p = &eq[..eq.len()-1];
        global = false;
    }

    let p = p.trim_end();
    let value = eq[p.len()+1..].trim_start();

    if kw == "delete" {
        if xheader_protected_pattern_p(p) {
            panic!("Pattern {} cannot be used", p);
        }
        xheader_list_append(&KEYWORD_PATTERN_LIST, p, None);
    } else if kw == "exthdr.name" {
        // assign_string
    } else if kw == "globexthdr.name" {
        // assign_string
    } else if kw == "exthdr.mtime" {
        // assign_time_option
    } else if kw == "globexthdr.mtime" {
        // assign_time_option
    } else {
        if xheader_protected_keyword_p(kw) {
            panic!("Keyword {} cannot be overridden", kw);
        }
        if global {
            xheader_list_append(&KEYWORD_GLOBAL_OVERRIDE_LIST, kw, Some(value));
        } else {
            xheader_list_append(&KEYWORD_OVERRIDE_LIST, kw, Some(value));
        }
    }
}

fn xheader_set_option(string: &str) {
    for token in string.split(',') {
        if let Some(eq_pos) = token.find('=') {
            xheader_set_keyword_equal(&token[..eq_pos], &token[eq_pos..]);
        } else {
            xheader_set_single_keyword(token);
        }
    }
}

fn xheader_format_name(st: &TarStatInfo, fmt: &str, n: usize) -> String {
    let mut result = String::new();
    let mut chars = fmt.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(next) = chars.peek() {
                match next {
                    '%' => {
                        result.push('%');
                        chars.next();
                    }
                    'd' => {
                        if !st.orig_file_name.as_os_str().is_empty() {
                            if let Some(parent) = st.orig_file_name.parent() {
                                result.push_str(parent.to_str().unwrap_or(""));
                            }
                        }
                        chars.next();
                    }
                    'f' => {
                        if !st.orig_file_name.as_os_str().is_empty() {
                            if let Some(file_name) = st.orig_file_name.file_name() {
                                result.push_str(file_name.to_str().unwrap_or(""));
                            }
                        }
                        chars.next();
                    }
                    'p' => {
                        result.push_str(&std::process::id().to_string());
                        chars.next();
                    }
                    'n' => {
                        result.push_str(&n.to_string());
                        chars.next();
                    }
                    _ => {
                        result.push(c);
                        result.push(chars.next().unwrap());
                    }
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    // Remove trailing slashes
    while result.ends_with('/') {
        result.pop();
    }

    result
}

fn xheader_xhdr_name(st: &TarStatInfo) -> String {
    // TODO: Implement proper template selection
    let template = "%d/PaxHeaders/%f";
    xheader_format_name(st, template, 0)
}

fn xheader_ghdr_name() -> String {
    // TODO: Implement proper template selection
    let template = "/GlobalHead.%n";
    let tmp = std::env::var("TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
    let count = GLOBAL_HEADER_COUNT.fetch_add(1, Ordering::SeqCst);
    xheader_format_name(&TarStatInfo::new(), &format!("{}{}", tmp, template), count + 1)
}

fn xheader_write(header_type: char, name: &str, t: time_t, xhdr: &mut XHeader) {
    // TODO: Implement header writing logic
    xhdr.destroy();
    if header_type == 'g' {
        GLOBAL_HEADER_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

fn xheader_write_global(xhdr: &mut XHeader) {
    let list = KEYWORD_GLOBAL_OVERRIDE_LIST.lock().unwrap();
    if !list.is_empty() {
        xhdr.init();
        for entry in list.iter() {
            if let Some(value) = &entry.value {
                // code_string(value, &entry.pattern, xhdr);
            }
        }
    }
    
    if xhdr.stk.is_some() {
        let name = xheader_ghdr_name();
        xheader_write('g', &name, 0, xhdr);
    }
}

fn xheader_forbid_global() {
    let list = KEYWORD_GLOBAL_OVERRIDE_LIST.lock().unwrap();
    if !list.is_empty() {
        panic!("can't update global extended header record");
    }
}

fn xheader_xattr_init(st: &mut TarStatInfo) {
    st.xattr_map.clear();
    st.acls_a_ptr = None;
    st.acls_a_len = 0;
    st.acls_d_ptr = None;
    st.acls_d_len = 0;
    st.cntx_name = None;
}

fn xheader_xattr_free(xattr_map: &mut HashMap<String, Vec<u8>>) {
    xattr_map.clear();
}

fn xattr_decode_keyword(keyword: &mut String) {
    let mut result = String::new();
    let mut chars = keyword.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(next1) = chars.next() {
                if let Some(next2) = chars.next() {
                    if next1 == '3' && next2 == 'D' {
                        result.push('=');
                    } else if next1 == '2' && next2 == '5' {
                        result.push('%');
                    } else {
                        result.push(c);
                        result.push(next1);
                        result.push(next2);
                    }
                } else {
                    result.push(c);
                    result.push(next1);
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }
    
    *keyword = result;
}

fn xheader_xattr_add(st: &mut TarStatInfo, key: &str, val: &[u8]) {
    let full_key = format!("SCHILY.xattr.{}", key);
    st.xattr_map.insert(full_key, val.to_vec());
}

// ... Additional implementations would follow for the remaining functions ...

fn xheader_protected_pattern_p(pattern: &str) -> bool {
    // TODO: Implement pattern protection check
    false
}

fn xheader_protected_keyword_p(keyword: &str) -> bool {
    // TODO: Implement keyword protection check
    false
}