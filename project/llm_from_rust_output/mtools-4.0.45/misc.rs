use std::env;
use std::ffi::{CString, CStr, OsString};
use std::fs::{File, remove_file, metadata};
use std::io::{self, Read, Write};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::ptr;
use libc::{uid_t, geteuid, getlogin, getpwuid, getpwnam, passwd, c_char, c_void};

const MCWD_FILE: &str = ".mcwd";
const DEFAULT_TEMP_DIR: &str = "/tmp";

fn print_oom() {
    eprintln!("Out of memory error");
}

fn get_homedir() -> Option<PathBuf> {
    if let Ok(home) = env::var("HOME") {
        return Some(PathBuf::from(home));
    }

    let username = env::var("LOGNAME")
        .ok()
        .or_else(|| unsafe {
            getlogin()
                .as_ref()
                .map(|cstr| CStr::from_ptr(cstr).to_string_lossy().into_owned())
        });

    let pw = username.and_then(|name| {
        unsafe {
            let cname = CString::new(name).ok()?;
            getpwnam(cname.as_ptr()).as_ref()
        }
    }).or_else(|| {
        unsafe {
            let uid = geteuid();
            getpwuid(uid).as_ref()
        }
    });

    pw.and_then(|p| {
        unsafe {
            CStr::from_ptr(p.pw_dir).to_str().ok()
        }
    }).map(PathBuf::from)
}

fn get_mcwd_file_path() -> PathBuf {
    if let Ok(mcwd_path) = env::var("MCWD") {
        if !mcwd_path.is_empty() {
            return PathBuf::from(mcwd_path);
        }
    }

    let base_dir = get_homedir().unwrap_or_else(|| PathBuf::from(DEFAULT_TEMP_DIR));
    base_dir.join(MCWD_FILE)
}

pub fn unlink_mcwd() -> io::Result<()> {
    let path = get_mcwd_file_path();
    if path.exists() {
        remove_file(path)
    } else {
        Ok(())
    }
}

pub fn open_mcwd(mode: &str) -> io::Result<File> {
    let path = get_mcwd_file_path();
    
    if mode == "r" {
        if let Ok(metadata) = metadata(&path) {
            let modified = metadata.modified()?;
            let now = SystemTime::now();
            
            if let Ok(duration) = now.duration_since(modified) {
                if duration.as_secs() > 6 * 60 * 60 {
                    eprintln!("Warning: \"{}\" is out of date, removing it", path.display());
                    remove_file(&path)?;
                    return Err(io::Error::new(io::ErrorKind::NotFound, "File removed due to being outdated"));
                }
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    }

    File::options()
        .read(mode.contains('r'))
        .write(mode.contains('w'))
        .create(mode.contains('w'))
        .open(path)
}

pub fn safe_malloc(size: usize) -> *mut c_void {
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe {
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            print_oom();
            std::process::exit(1);
        }
        ptr as *mut c_void
    }
}

pub fn print_sector(message: &str, data: &[u8]) {
    println!("{}:", message);
    
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:03x}  ", i * 16);
        
        for &byte in chunk {
            print!("{:02x} ", byte);
        }
        
        print!("{:width$}", "", width = (16 - chunk.len()) * 3);
        
        for &byte in chunk {
            if byte.is_ascii_graphic() || byte == b' ' {
                print!("{}", byte as char);
            } else {
                print!(".");
            }
        }
        
        println!();
    }
}

pub fn get_time_now() -> u64 {
    if let Ok(source_date_epoch) = env::var("SOURCE_DATE_EPOCH") {
        if let Ok(epoch) = source_date_epoch.parse::<u64>() {
            return epoch;
        }
    }
    
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn str_to_offset(s: &str) -> Option<i64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num_part, suffix) = s.split_at(
        s.find(|c: char| !c.is_ascii_digit())
            .unwrap_or(s.len())
    );

    let mut value = num_part.parse::<i64>().ok()?;
    
    match suffix.chars().next() {
        Some('s') | Some('S') => value <<= 9,
        Some('k') | Some('K') => value <<= 10,
        Some('m') | Some('M') => value <<= 20,
        Some('g') | Some('G') => value <<= 30,
        Some('t') | Some('T') => value <<= 40,
        Some(_) => return None,
        None => {}
    }

    if value <= 0 {
        None
    } else {
        Some(value)
    }
}