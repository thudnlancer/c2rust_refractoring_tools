use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Write, stderr, stdout};
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Cbuf {
    pub string: Option<CString>,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct Delta {
    pub num: Option<CString>,
    pub date: Option<CString>,
    pub author: Option<CString>,
    pub lockedby: Option<CString>,
    pub state: Option<CString>,
    pub log: Option<Vec<u8>>,
    pub text: Option<Vec<u8>>,
    pub name: Option<CString>,
    pub pretty_log: Cbuf,
    pub branches: Option<Vec<Delta>>,
    pub commitid: Option<CString>,
    pub ilk: Option<Box<Delta>>,
    pub selector: bool,
    pub neck: i64,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub invoke: Option<CString>,
    pub name: Option<CString>,
    pub desc: Option<CString>,
    pub help: Option<CString>,
    pub tyag: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maker {
    NotMade = 0,
    Real = 1,
    Effective = 2,
}

#[derive(Debug, Clone)]
pub struct Sff {
    pub filename: Option<CString>,
    pub disposition: Maker,
}

#[derive(Debug, Clone, Copy)]
pub struct ZoneOffset {
    pub valid: bool,
    pub seconds: i64,
}

#[derive(Debug, Clone)]
pub struct Manifestation {
    pub filename: Option<CString>,
    pub standard_output: Option<File>,
    pub prev: Option<ManifestationPrev>,
}

#[derive(Debug, Clone)]
pub struct ManifestationPrev {
    pub valid: bool,
    pub author: Option<CString>,
    pub date: Option<CString>,
    pub name: Option<CString>,
    pub rev: Option<CString>,
    pub state: Option<CString>,
}

#[derive(Debug, Clone)]
pub struct Behavior {
    pub invdir: Option<CString>,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_locker_in_id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: i32,
    pub stick_with_euid: bool,
    pub ruid: i32,
    pub euid: i32,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: i32,
    pub pe: Option<CString>,
    pub zone_offset: ZoneOffset,
    pub username: Option<CString>,
    pub now: SystemTime,
    pub fixed_sigchld: bool,
    pub oerrloop: bool,
    pub cwd: Option<CString>,
    pub mem_limit: i64,
    pub sff: Option<Vec<Sff>>,
    // Other fields omitted for brevity
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub filename: Option<CString>,
    pub fd_lock: i32,
    // stat field omitted
    pub r: Option<Repo>,
    pub tip: Option<Delta>,
    pub log_lead: Cbuf,
}

#[derive(Debug, Clone)]
pub struct Repo {
    pub head: Option<CString>,
    pub branch: Option<CString>,
    pub access_count: usize,
    pub access: Option<Vec<Link>>,
    pub symbols_count: usize,
    pub symbols: Option<Vec<Link>>,
    pub locks_count: usize,
    pub locks: Option<Vec<Link>>,
    pub strict: bool,
    pub integrity: Option<Vec<u8>>,
    pub comment: Option<Vec<u8>>,
    pub expand: i32,
    pub deltas_count: usize,
    pub deltas: Option<Vec<Delta>>,
    pub desc: Option<Vec<u8>>,
    pub neck: i64,
    // Other fields omitted
}

#[derive(Debug, Clone)]
pub struct Link {
    // Link fields omitted
}

#[derive(Debug, Clone)]
pub struct Flow {
    pub from: Option<File>,
    pub rewr: Option<File>,
    pub to: Option<File>,
    pub res: Option<File>,
    pub result: Option<CString>,
    pub erroneous: bool,
}

#[derive(Debug, Clone)]
pub struct Top {
    pub program: Program,
    pub behavior: Behavior,
    pub manifestation: Manifestation,
    pub repository: Repository,
    pub flow: Flow,
}

static mut TOP: Option<Top> = None;

pub fn unbuffer_standard_error() -> io::Result<()> {
    unsafe {
        if let Some(ref mut top) = TOP {
            let stderr = stderr();
            let result = stderr.lock().set_unbuffered();
            top.behavior.unbuffered = result.is_ok();
            result
        } else {
            Ok(())
        }
    }
}

pub fn vcomplain(fmt: &str, args: std::fmt::Arguments) {
    unsafe {
        if let Some(ref mut top) = TOP {
            if let Some(ref mut stdout) = top.manifestation.standard_output {
                let _ = stdout.flush();
            } else {
                let _ = stdout().flush();
            }
        }
        
        let stderr = stderr();
        let mut handle = stderr.lock();
        let _ = handle.write_fmt(args);
        
        if let Some(ref top) = TOP {
            if !top.behavior.unbuffered {
                let _ = handle.flush();
            }
        }
    }
}

pub fn complain(fmt: &str) {
    vcomplain(fmt, format_args!("{}", fmt));
}

pub fn diagnose(fmt: &str) {
    unsafe {
        if let Some(ref top) = TOP {
            if !top.behavior.quiet {
                vcomplain(fmt, format_args!("{}", fmt));
                complain("\n");
            }
        }
    }
}

fn whoami(who: Option<&str>) {
    unsafe {
        if let Some(ref top) = TOP {
            complain(&format!("{}: ", top.program.name.unwrap().to_str().unwrap()));
            if let Some(w) = who {
                complain(&format!("{}: ", w));
            }
        }
    }
}

pub fn syserror(e: i32, who: Option<&str>) {
    unsafe {
        if let Some(ref mut top) = TOP {
            whoami(None);
            top.flow.erroneous = true;
            let errno = e;
            perror(who);
        }
    }
}

fn perror(who: Option<&str>) {
    let error = io::Error::last_os_error();
    if let Some(w) = who {
        eprintln!("{}: {}", w, error);
    } else {
        eprintln!("{}", error);
    }
}

pub fn generic_warn(who: Option<&str>, fmt: &str) {
    unsafe {
        if let Some(ref top) = TOP {
            if !top.behavior.quiet {
                whoami(who);
                complain("warning: ");
                vcomplain(fmt, format_args!("{}", fmt));
                complain("\n");
            }
        }
    }
}

pub fn generic_error(who: Option<&str>, fmt: &str) {
    unsafe {
        if let Some(ref mut top) = TOP {
            top.flow.erroneous = true;
            whoami(who);
            vcomplain(fmt, format_args!("{}", fmt));
            complain("\n");
        }
    }
}

fn die() {
    unsafe {
        if let Some(ref top) = TOP {
            complain(&format!("{} aborted\n", top.program.name.unwrap().to_str().unwrap()));
            std::process::exit(top.program.tyag);
        }
    }
}

pub fn generic_fatal(who: Option<&str>, fmt: &str) {
    unsafe {
        if let Some(ref mut top) = TOP {
            top.flow.erroneous = true;
            whoami(who);
            vcomplain(fmt, format_args!("{}", fmt));
            complain("\n");
            die();
        }
    }
}

pub fn fatal_syntax(lno: usize, fmt: &str) {
    unsafe {
        if let Some(ref top) = TOP {
            complain(&format!(
                "{}: {}:",
                top.program.name.unwrap().to_str().unwrap(),
                top.repository.filename.unwrap().to_str().unwrap()
            ));
            if lno != 0 {
                complain(&format!("{}:", lno));
            }
            complain(" ");
            vcomplain(fmt, format_args!("{}", fmt));
            complain("\n");
            die();
        }
    }
}

pub fn fatal_sys(who: Option<&str>) {
    syserror(io::Error::last_os_error().raw_os_error().unwrap_or(0), who);
    die();
}