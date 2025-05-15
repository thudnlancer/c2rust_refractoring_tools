use std::env;
use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::mem;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{uid_t, geteuid, getlogin, getpwnam, getpwuid, passwd};
use users::{get_user_by_name, get_user_by_uid};

fn print_oom() {
    eprintln!("Out of memory error");
}

fn get_homedir() -> Option<String> {
    if let Ok(homedir) = env::var("HOME") {
        return Some(homedir);
    }

    let username = env::var("LOGNAME")
        .ok()
        .or_else(|| unsafe { CString::from_raw(getlogin()).into_string().ok() });

    let pw = username.and_then(|name| get_user_by_name(&name));

    if pw.is_none() {
        let uid = unsafe { geteuid() };
        return get_user_by_uid(uid).map(|u| u.home_dir().to_string_lossy().into_owned());
    }

    pw.map(|u| u.home_dir().to_string_lossy().into_owned())
}

fn get_mcwd_file_name() -> PathBuf {
    if let Ok(mcwd_path) = env::var("MCWD") {
        if !mcwd_path.is_empty() {
            return PathBuf::from(mcwd_path);
        }
    }

    let homedir = get_homedir().unwrap_or_else(|| "/tmp".to_string());
    PathBuf::from(homedir).join(".mcwd")
}

pub fn unlink_mcwd() -> io::Result<()> {
    let file = get_mcwd_file_name();
    if file.exists() {
        fs::remove_file(file)
    } else {
        Ok(())
    }
}

pub fn open_mcwd(mode: &str) -> io::Result<File> {
    let file = get_mcwd_file_name();

    if mode.starts_with('r') {
        if let Ok(metadata) = fs::metadata(&file) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let mtime = metadata.mtime() as u64;
            if now - mtime > 6 * 60 * 60 {
                eprintln!("Warning: \"{}\" is out of date, removing it", file.display());
                fs::remove_file(&file)?;
                return Err(io::Error::new(io::ErrorKind::NotFound, "File removed"));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    }

    OpenOptions::new()
        .read(mode.contains('r'))
        .write(mode.contains('w'))
        .create(mode.contains('w') || mode.contains('a'))
        .append(mode.contains('a'))
        .truncate(mode.contains('w'))
        .open(file)
}

pub fn safe_malloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
    unsafe {
        let ptr = std::alloc::alloc(layout);
        if ptr.is_null() {
            print_oom();
            process::exit(1);
        }
        ptr
    }
}

pub fn print_sector(message: &str, data: &[u8]) {
    println!("{}:", message);

    for (row, chunk) in data.chunks(16).enumerate() {
        print!("{:03x}  ", row * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in 0..(16 - chunk.len()) {
            print!("   ");
        }
        for byte in chunk {
            if byte.is_ascii_graphic() {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn get_time_now() -> u64 {
    static mut HAVE_TIME: bool = false;
    static mut SHARED_NOW: u64 = 0;

    unsafe {
        if !HAVE_TIME {
            if let Ok(source_date_epoch) = env::var("SOURCE_DATE_EPOCH") {
                match source_date_epoch.parse::<u64>() {
                    Ok(epoch) => {
                        SHARED_NOW = epoch;
                        HAVE_TIME = true;
                    }
                    Err(e) => {
                        eprintln!("SOURCE_DATE_EPOCH \"{}\" invalid: {}", source_date_epoch, e);
                    }
                }
            }

            if !HAVE_TIME {
                SHARED_NOW = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                HAVE_TIME = true;
            }
        }
        SHARED_NOW
    }
}

pub fn str_to_off_with_end(str: &str) -> (i64, &str) {
    let mut siz = 0i64;
    let mut remaining = str;

    if let Ok((num, rest)) = lexical_core::parse_partial::<i64>(remaining.as_bytes()) {
        siz = num;
        remaining = &remaining[rest.len()..];
    }

    if let Some(c) = remaining.chars().next() {
        match c.to_ascii_lowercase() {
            's' => siz <<= 9,
            'k' => siz <<= 10,
            'm' => siz <<= 20,
            'g' => siz <<= 30,
            't' => siz <<= 40,
            _ => return (siz, remaining),
        }
        remaining = &remaining[1..];
    }

    (siz, remaining)
}

pub fn str_to_offset(str: &str) -> Option<i64> {
    let (ofs, remaining) = str_to_off_with_end(str);
    if ofs <= 0 || !remaining.is_empty() {
        None
    } else {
        Some(ofs)
    }
}