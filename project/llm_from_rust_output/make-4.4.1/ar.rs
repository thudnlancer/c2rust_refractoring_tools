use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;
use std::time::SystemTime;
use libc::{size_t, time_t};
use fnmatch::fnmatch;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub hname: String,
    pub vpath: Option<String>,
    pub deps: Vec<Dep>,
    pub cmds: Vec<Command>,
    pub stem: Option<String>,
    pub also_make: Vec<Dep>,
    pub prev: Option<Box<File>>,
    pub last: Option<Box<File>>,
    pub renamed: Option<Box<File>>,
    pub variables: Vec<VariableSet>,
    pub pat_variables: Vec<VariableSet>,
    pub parent: Option<Box<File>>,
    pub double_colon: Option<Box<File>>,
    pub last_mtime: u64,
    pub mtime_before_update: u64,
    pub considered: u32,
    pub command_flags: i32,
    pub update_status: UpdateStatus,
    pub command_state: CommandState,
    pub builtin: bool,
    pub precious: bool,
    pub loaded: bool,
    pub unloaded: bool,
    pub low_resolution_time: bool,
    pub tried_implicit: bool,
    pub updating: bool,
    pub updated: bool,
    pub is_target: bool,
    pub cmd_target: bool,
    pub phony: bool,
    pub intermediate: bool,
    pub is_explicit: bool,
    pub secondary: bool,
    pub notintermediate: bool,
    pub dontcare: bool,
    pub ignore_vpath: bool,
    pub pat_searched: bool,
    pub no_diag: bool,
    pub was_shuffled: bool,
    pub snapped: bool,
}

#[derive(Debug, Clone)]
pub enum UpdateStatus {
    Success,
    None,
    Question,
    Failed,
}

#[derive(Debug, Clone)]
pub enum CommandState {
    NotStarted,
    DepsRunning,
    Running,
    Finished,
}

#[derive(Debug, Clone)]
pub struct Dep {
    pub next: Option<Box<Dep>>,
    pub name: String,
    pub file: Option<Box<File>>,
    pub shuf: Option<Box<Dep>>,
    pub stem: Option<String>,
    pub flags: u8,
    pub changed: bool,
    pub ignore_mtime: bool,
    pub staticpattern: bool,
    pub need_2nd_expansion: bool,
    pub ignore_automatic_vars: bool,
    pub is_explicit: bool,
    pub wait_here: bool,
}

#[derive(Debug, Clone)]
pub struct Floc {
    pub filenm: String,
    pub lineno: u64,
    pub offset: u64,
}

#[derive(Debug, Clone)]
pub struct Nameseq {
    pub next: Option<Box<Nameseq>>,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ArGlobState {
    pub arname: String,
    pub pattern: String,
    pub size: usize,
    pub chain: Option<Box<Nameseq>>,
    pub n: u32,
}

pub fn ar_name(name: &str) -> bool {
    let p = name.find('(');
    if p.is_none() || p.unwrap() == 0 {
        return false;
    }
    let end = name.rfind(')');
    if end.is_none() || end.unwrap() == p.unwrap() + 1 {
        return false;
    }
    if name.chars().nth(p.unwrap() + 1) == Some('(')
        && name.chars().nth(end.unwrap() - 1) == Some(')')
    {
        panic!("attempt to use unsupported feature: '{}'", name);
    }
    true
}

pub fn ar_parse_name(name: &str) -> (String, String) {
    let p = name.find('(').expect("Internal: ar_parse_name: bad name");
    let arname = name[..p].to_string();
    let memname = name[p + 1..name.len() - 1].to_string();
    (arname, memname)
}

fn ar_member_date_1(
    _desc: i32,
    mem: &str,
    truncated: bool,
    _hdrpos: i64,
    _datapos: i64,
    _size: i64,
    date: i64,
    _uid: i32,
    _gid: i32,
    _mode: u32,
    name: &str,
) -> i64 {
    if ar_name_equal(name, mem, truncated) {
        date
    } else {
        0
    }
}

pub fn ar_member_date(name: &str) -> Option<SystemTime> {
    let (arname, memname) = ar_parse_name(name);
    let arfile = lookup_file(&arname).or_else(|| {
        if file_exists_p(&arname) {
            Some(enter_file(&arname))
        } else {
            None
        }
    });
    if let Some(file) = arfile {
        f_mtime(file, false);
    }
    let val = ar_scan(&arname, ar_member_date_1, &memname);
    if val > 0 {
        Some(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(val as u64))
    } else {
        None
    }
}

pub fn ar_touch(name: &str) -> Result<(), String> {
    let (arname, memname) = ar_parse_name(name);
    let arfile = enter_file(&arname);
    f_mtime(arfile, false);
    match ar_member_touch(&arname, &memname) {
        -1 => Err(format!("touch: Archive '{}' does not exist", arname)),
        -2 => Err(format!("touch: '{}' is not a valid archive", arname)),
        -3 => Err(format!("touch: {}", arname)),
        1 => Err(format!(
            "touch: Member '{}' does not exist in '{}'",
            memname, arname
        )),
        0 => Ok(()),
        _ => Err(format!(
            "touch: Bad return code from ar_member_touch on '{}'",
            name
        )),
    }
}

fn ar_glob_match(
    _desc: i32,
    mem: &str,
    _truncated: bool,
    _hdrpos: i64,
    _datapos: i64,
    _size: i64,
    _date: i64,
    _uid: i32,
    _gid: i32,
    _mode: u32,
    state: &mut ArGlobState,
) -> i64 {
    if fnmatch(&state.pattern, mem, fnmatch::Flags::NOESCAPE | fnmatch::Flags::PATHNAME) == 0 {
        let new_name = format!("{}({})", state.arname, mem);
        let new = Nameseq {
            name: new_name,
            next: state.chain.take(),
        };
        state.chain = Some(Box::new(new));
        state.n += 1;
    }
    0
}

fn ar_glob_pattern_p(pattern: &str, quote: bool) -> bool {
    let mut opened = false;
    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '?' | '*' => return true,
            '\\' => {
                if quote {
                    chars.next();
                }
            }
            '[' => opened = true,
            ']' => {
                if opened {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

pub fn ar_glob(arname: &str, member_pattern: &str, size: usize) -> Option<Box<Nameseq>> {
    if !ar_glob_pattern_p(member_pattern, true) {
        return None;
    }
    let mut state = ArGlobState {
        arname: arname.to_string(),
        pattern: member_pattern.to_string(),
        size,
        chain: None,
        n: 0,
    };
    ar_scan(arname, ar_glob_match, &mut state);
    if state.chain.is_none() {
        return None;
    }
    let mut names: Vec<String> = Vec::with_capacity(state.n as usize);
    let mut n = state.chain.as_ref();
    while let Some(node) = n {
        names.push(node.name.clone());
        n = node.next.as_ref();
    }
    names.sort();
    let mut n = state.chain.as_mut();
    let mut i = 0;
    while let Some(node) = n {
        node.name = names[i].clone();
        i += 1;
        n = node.next.as_mut();
    }
    state.chain
}