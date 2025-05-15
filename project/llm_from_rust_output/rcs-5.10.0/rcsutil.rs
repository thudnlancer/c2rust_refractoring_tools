use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn new(sec: i64, nsec: i64) -> Self {
        Timespec { tv_sec: sec, tv_nsec: nsec }
    }

    pub fn unspecified() -> Self {
        Timespec::new(-1, 0)
    }
}

pub struct Stat {
    // Simplified stat struct with just mtime
    pub st_mtime: Timespec,
}

impl Stat {
    pub fn mtime(&self) -> Timespec {
        self.st_mtime
    }
}

pub struct Cbuf {
    pub string: *const c_char,
    pub size: usize,
}

impl Cbuf {
    pub fn empty() -> Self {
        Cbuf {
            string: ptr::null(),
            size: 0,
        }
    }
}

pub struct Top {
    pub behavior: Behavior,
    // Other fields omitted for brevity
}

pub struct Behavior {
    pub version: c_int,
    pub version_set: bool,
    pub fixed_sigchld: bool,
    // Other fields omitted for brevity
}

impl Top {
    pub fn new() -> Self {
        Top {
            behavior: Behavior {
                version: 0,
                version_set: false,
                fixed_sigchld: false,
            },
        }
    }
}

pub fn make_timespec(sec: i64, nsec: i64) -> Timespec {
    Timespec::new(sec, nsec)
}

pub fn get_stat_mtime(st: &Stat) -> Timespec {
    st.mtime()
}

pub fn file_mtime(enable: bool, st: &Stat) -> Timespec {
    if enable {
        get_stat_mtime(st)
    } else {
        Timespec::unspecified()
    }
}

pub fn set_empty_log_message(cb: &mut Cbuf) {
    let msg = CString::new("*** empty log message ***").unwrap();
    cb.string = msg.as_ptr();
    cb.size = msg.to_bytes().len();
}

pub fn str_save(s: &str) -> *mut c_char {
    let cstr = CString::new(s).unwrap();
    cstr.into_raw()
}

pub fn parse_revpairs(
    option: char,
    arg: &str,
    put: impl Fn(&str, &str, bool),
) -> Result<(), String> {
    let separator = if arg.contains(':') { ':' } else { '-' };
    
    if separator == '-' && arg.contains('-') {
        eprintln!("warning: `-' is obsolete in `-{}{}'; use `:' instead", option, arg);
    }

    for pair in arg.split(|c| c == ',' || c == ';') {
        let trimmed = pair.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(pos) = trimmed.find(separator) {
            let (b, e) = trimmed.split_at(pos);
            let b = b.trim();
            let e = e[1..].trim();
            put(b, e, true);
        } else {
            put(trimmed, "", false);
        }
    }

    Ok(())
}

pub fn set_rcs_version(top: &mut Top, version_str: &str) -> Result<(), String> {
    if version_str.is_empty() {
        return Ok(());
    }

    if top.behavior.version_set {
        eprintln!("warning: redefinition of version");
    }

    let version = version_str.parse::<i32>().map_err(|_| {
        format!("{} isn't a number", version_str)
    })?;

    if version < 3 || version > 5 {
        return Err(format!("{} out of range 3..5", version_str));
    }

    top.behavior.version = version - 5;
    top.behavior.version_set = true;
    Ok(())
}

pub fn run_command(
    infd: Option<i32>,
    outname: Option<&str>,
    args: &[&str],
) -> Result<i32, String> {
    // Simplified command execution
    let status = std::process::Command::new(args[0])
        .args(&args[1..])
        .stdin(if let Some(fd) = infd {
            unsafe { std::os::unix::io::FromRawFd::from_raw_fd(fd) }
        } else {
            std::process::Stdio::inherit()
        })
        .stdout(if let Some(name) = outname {
            std::process::Stdio::from(std::fs::File::create(name)?)
        } else {
            std::process::Stdio::inherit()
        })
        .status()?;

    Ok(status.code().unwrap_or(1))
}

pub fn get_current_time() -> Timespec {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    Timespec::new(
        duration.as_secs() as i64,
        duration.subsec_nanos() as i64,
    )
}