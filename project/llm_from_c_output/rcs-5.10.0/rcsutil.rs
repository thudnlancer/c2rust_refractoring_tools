use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::{Command, ExitStatus};
use std::io::{self, Write, Error, ErrorKind};
use std::env;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use std::collections::HashMap;
use std::time::Duration;

static FIXED_SIGCHLD: AtomicBool = AtomicBool::new(false);
static VERSION_SET: AtomicBool = AtomicBool::new(false);
static VERSION: AtomicUsize = AtomicUsize::new(0);

const DIFF_TROUBLE: i32 = 2;
const DIFF_FAILURE: i32 = 1;
const EXIT_FAILURE: i32 = 1;
const MEMORY_UNLIMITED: usize = usize::MAX;
const SSIZE_MAX: usize = isize::MAX as usize;
const TIME_UNSPECIFIED: i64 = -1;
const ZERO_NANOSECONDS: u32 = 0;

struct Timespec {
    tv_sec: i64,
    tv_nsec: u32,
}

struct Cbuf {
    string: String,
    size: usize,
}

struct Program {
    invoke: String,
}

struct Top {
    program: Program,
}

static mut TOP: Option<Top> = None;
static mut PLEXUS: Option<HashMap<String, String>> = None;
static mut SINGLE: Option<HashMap<String, String>> = None;

fn make_space(name: &str) -> Option<HashMap<String, String>> {
    Some(HashMap::new())
}

fn close_space(space: &mut Option<HashMap<String, String>>) {
    *space = None;
}

fn unbuffer_standard_error() {
    // Rust's stderr is unbuffered by default
}

fn gettime(now: &mut Timespec) {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    now.tv_sec = duration.as_secs() as i64;
    now.tv_nsec = duration.subsec_nanos();
}

fn make_timespec(sec: i64, nsec: u32) -> Timespec {
    Timespec { tv_sec: sec, tv_nsec }
}

fn exit_diff_trouble(fmt: &str) -> ! {
    eprintln!("{}", fmt);
    std::process::exit(DIFF_TROUBLE);
}

fn thank_you_and_goodnight(how: i32) -> ! {
    if how & 0x01 != 0 {
        // ORCSerror();
    }
    if how & 0x02 != 0 {
        // dirtempunlink();
    }
    if how & 0x04 != 0 {
        // tempunlink();
    }
    std::process::exit(if how & 0x08 != 0 {
        DIFF_FAILURE
    } else {
        EXIT_FAILURE
    });
}

fn gnurcs_init(program: Program) {
    unsafe {
        TOP = Some(Top { program });
        PLEXUS = make_space("plexus");
        SINGLE = make_space("single");
        unbuffer_standard_error();
        
        // Initialize other global state
        gettime(&mut TOP.as_mut().unwrap().now);
        
        // Set memory limit
        let mem_limit = env::var("RCS_MEM_LIMIT")
            .ok()
            .and_then(|v| if v.is_empty() { None } else { Some(v) })
            .and_then(|v| v.parse::<usize>().ok())
            .map(|lim| lim.max(0))
            .unwrap_or(MEMORY_UNLIMITED);
        
        TOP.as_mut().unwrap().mem_limit = mem_limit;
    }
}

fn gnurcs_goodbye() {
    unsafe {
        TOP = None;
        close_space(&mut SINGLE);
        close_space(&mut PLEXUS);
    }
}

fn bad_option(option: &str) {
    eprintln!("unknown option: {}", option);
}

fn redefined(c: char) {
    eprintln!("redefinition of -{} option", c);
}

fn chk_set_rev(rev: &mut Option<String>, arg: String) {
    if arg.is_empty() {
        return;
    }
    if rev.is_some() {
        eprintln!("redefinition of revision");
    }
    *rev = Some(arg);
}

fn minus_p(xrev: &str, rev: &str) -> Cbuf {
    println!("retrieving revision {}", xrev);
    let string = format!("-p{}", rev);
    Cbuf {
        string,
        size: string.len(),
    }
}

fn set_empty_log_message(cb: &mut Cbuf) {
    cb.string = String::new();
    cb.size = 0;
}

fn ffree() {
    unsafe {
        if let Some(single) = &mut SINGLE {
            single.clear();
        }
    }
}

fn str_save(s: &str) -> String {
    s.to_string()
}

fn cgetenv(name: &str) -> Option<String> {
    env::var(name).ok()
}

fn awrite(buf: &[u8], f: &mut File) -> io::Result<()> {
    let mut written = 0;
    while written < buf.len() {
        let chunk_size = (buf.len() - written).min(SSIZE_MAX);
        f.write_all(&buf[written..written + chunk_size])?;
        written += chunk_size;
    }
    Ok(())
}

fn runv(infd: Option<i32>, outname: Option<&str>, args: &[String]) -> io::Result<ExitStatus> {
    if !FIXED_SIGCHLD.load(Ordering::SeqCst) {
        FIXED_SIGCHLD.store(true, Ordering::SeqCst);
        // maybe_reset_sigchld();
    }

    let mut command = Command::new(&args[0]);
    command.args(&args[1..]);

    if let Some(fd) = infd {
        if fd != 0 {
            command.stdin(unsafe { File::from_raw_fd(fd) });
        }
    }

    if let Some(name) = outname {
        command.stdout(File::create(name)?);
    }

    let status = command.status()?;
    if !status.success() {
        if let Some(signal) = status.signal() {
            eprintln!("{} got a fatal signal {}", args[0], signal);
            return Err(Error::new(ErrorKind::Other, "command failed"));
        }
        return Err(Error::new(ErrorKind::Other, "command failed"));
    }
    Ok(status)
}

fn run(infd: Option<i32>, outname: Option<&str>, args: &[&str]) -> io::Result<ExitStatus> {
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    runv(infd, outname, &args)
}

fn set_rcs_version(str: &str) {
    let s = &str[2..];
    if !s.is_empty() {
        if VERSION_SET.load(Ordering::SeqCst) {
            redefined('V');
        }
        VERSION_SET.store(true, Ordering::SeqCst);
        
        let v = s.parse::<usize>().unwrap_or(VERSION_DEFAULT);
        if v < VERSION_MIN || v > VERSION_MAX {
            eprintln!("{} out of range {}..{}", str, VERSION_MIN, VERSION_MAX);
            return;
        }
        
        VERSION.store(v, Ordering::SeqCst);
    } else {
        // display_version
    }
}

fn get_rcs_init(argc: i32, argv: &[String]) -> (i32, Vec<String>) {
    if let Ok(rcsinit) = env::var("RCSINIT") {
        let mut new_argv = vec![argv[0].clone()];
        let mut new_argc = 1;
        
        for arg in rcsinit.split_whitespace() {
            new_argv.push(arg.to_string());
            new_argc += 1;
        }
        
        for arg in &argv[1..] {
            new_argv.push(arg.clone());
        }
        
        (new_argc, new_argv)
    } else {
        (argc, argv.to_vec())
    }
}

fn unspecified_timespec() -> Timespec {
    make_timespec(TIME_UNSPECIFIED, ZERO_NANOSECONDS)
}

fn file_mtime(enable: bool, st: Option<std::fs::Metadata>) -> Timespec {
    if enable {
        if let Some(metadata) = st {
            let mtime = metadata.modified().unwrap();
            let duration = mtime.duration_since(UNIX_EPOCH).unwrap();
            make_timespec(duration.as_secs() as i64, duration.subsec_nanos())
        } else {
            unspecified_timespec()
        }
    } else {
        unspecified_timespec()
    }
}