use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use libc::{self, c_char, c_int, dirent, DIR};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{fstatat, FileStat, Mode, SFlag};
use nix::unistd::{close, getcwd};
use std::ffi::OsStr;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::time::{SystemTime, UNIX_EPOCH};

static DOUBLE_SLASH_IS_DISTINCT_ROOT: bool = false;

fn quote_n_colon(n: i32, arg: &str) -> String {
    // TODO: Implement proper quoting style
    format!("{}:{}", n, arg)
}

fn assign_string(string: &mut Option<String>, value: Option<&str>) {
    *string = value.map(|s| s.to_string());
}

fn assign_string_n(string: &mut Option<String>, value: Option<&str>, n: usize) {
    *string = value.map(|s| {
        let len = s.chars().take(n).count();
        s.chars().take(len).collect()
    });
}

fn unquote_string(string: &mut String) -> bool {
    let mut result = true;
    let mut source = 0;
    let mut destination = 0;
    let bytes = unsafe { string.as_bytes_mut() };

    while source < bytes.len() {
        if bytes[source] == b'\\' {
            source += 1;
            if source >= bytes.len() {
                result = false;
                bytes[destination] = b'\\';
                destination += 1;
                break;
            }

            match bytes[source] {
                b'\\' => {
                    bytes[destination] = b'\\';
                    source += 1;
                }
                b'a' => {
                    bytes[destination] = b'\x07';
                    source += 1;
                }
                b'b' => {
                    bytes[destination] = b'\x08';
                    source += 1;
                }
                b'f' => {
                    bytes[destination] = b'\x0c';
                    source += 1;
                }
                b'n' => {
                    bytes[destination] = b'\n';
                    source += 1;
                }
                b'r' => {
                    bytes[destination] = b'\r';
                    source += 1;
                }
                b't' => {
                    bytes[destination] = b'\t';
                    source += 1;
                }
                b'v' => {
                    bytes[destination] = b'\x0b';
                    source += 1;
                }
                b'?' => {
                    bytes[destination] = 0x7f;
                    source += 1;
                }
                b'0'..=b'7' => {
                    let mut value = (bytes[source] - b'0') as i32;
                    source += 1;

                    if source < bytes.len() && bytes[source] >= b'0' && bytes[source] <= b'7' {
                        value = value * 8 + (bytes[source] - b'0') as i32;
                        source += 1;

                        if source < bytes.len() && bytes[source] >= b'0' && bytes[source] <= b'7' {
                            value = value * 8 + (bytes[source] - b'0') as i32;
                            source += 1;
                        }
                    }
                    bytes[destination] = value as u8;
                }
                _ => {
                    result = false;
                    bytes[destination] = b'\\';
                    destination += 1;
                    if source < bytes.len() {
                        bytes[destination] = bytes[source];
                        source += 1;
                    }
                }
            }
            destination += 1;
        } else {
            if source != destination {
                bytes[destination] = bytes[source];
            }
            source += 1;
            destination += 1;
        }
    }

    if destination < bytes.len() {
        bytes[destination] = b'\0';
        string.truncate(destination);
    }
    result
}

fn zap_slashes(name: &mut String) -> &str {
    if name.is_empty() {
        return name;
    }

    while name.ends_with('/') && name.len() > 1 {
        name.pop();
    }
    name
}

fn normalize_filename_x(file_name: &mut String) {
    // Implementation omitted for brevity
    // Similar to C version but using Rust string operations
}

fn normalize_filename(cdidx: i32, name: &str) -> String {
    let mut copy = if is_relative_file_name(name) {
        let cdpath = tar_getcdpath(cdidx);
        let need_separator = !(DOUBLE_SLASH_IS_DISTINCT_ROOT && cdpath.len() == 2 && cdpath.starts_with("//"));
        let mut path = cdpath.to_string();
        if need_separator {
            path.push('/');
        }
        path.push_str(name);
        path
    } else {
        name.to_string()
    };

    normalize_filename_x(&mut copy);
    copy
}

fn replace_prefix(pname: &mut String, samp: &str, repl: &str) {
    if pname.starts_with(samp) && pname.len() > samp.len() && pname.chars().nth(samp.len()) == Some('/') {
        let suffix = &pname[samp.len()..];
        *pname = format!("{}{}", repl, suffix);
    }
}

struct Wd {
    name: String,
    abspath: Option<String>,
    fd: Option<RawFd>,
}

static mut WD: Vec<Wd> = Vec::new();
static mut WD_COUNT: usize = 0;
static mut WD_ALLOC: usize = 0;
static mut WDCACHE: [usize; 16] = [0; 16];
static mut WDCACHE_COUNT: usize = 0;
static mut CHDIR_CURRENT: usize = 0;
static mut CHDIR_FD: RawFd = libc::AT_FDCWD;

fn chdir_count() -> usize {
    unsafe {
        if WD_COUNT == 0 {
            return WD_COUNT;
        }
        WD_COUNT - 1
    }
}

fn chdir_arg(dir: &str) -> usize {
    unsafe {
        if WD_COUNT == WD_ALLOC {
            if WD_ALLOC == 0 {
                WD_ALLOC = 2;
            }
            WD.reserve(WD_ALLOC);
            WD_ALLOC *= 2;

            if WD_COUNT == 0 {
                WD.push(Wd {
                    name: ".".to_string(),
                    abspath: None,
                    fd: Some(libc::AT_FDCWD),
                });
                WD_COUNT += 1;
            }
        }

        // Optimize common cases
        let mut dir = dir;
        let mut trimmed = dir.trim_start_matches("./");
        while trimmed.starts_with('/') {
            trimmed = trimmed.trim_start_matches('/');
        }
        if trimmed.is_empty() || (trimmed == "." && !dir.contains('/')) {
            return WD_COUNT - 1;
        }

        WD.push(Wd {
            name: dir.to_string(),
            abspath: None,
            fd: None,
        });
        WD_COUNT += 1;
        WD_COUNT - 1
    }
}

fn chdir_do(i: usize) {
    unsafe {
        if CHDIR_CURRENT != i {
            let curr = &mut WD[i];
            let fd = match curr.fd {
                Some(fd) => fd,
                None => {
                    let parent_fd = if i > 0 {
                        chdir_do(i - 1);
                        CHDIR_FD
                    } else {
                        libc::AT_FDCWD
                    };

                    let fd = open(
                        Path::new(&curr.name),
                        OFlag::O_RDONLY | OFlag::O_DIRECTORY,
                        Mode::empty(),
                    ).unwrap_or_else(|_| {
                        panic!("Failed to open directory: {}", curr.name);
                    });

                    curr.fd = Some(fd);

                    // Update cache
                    if WDCACHE_COUNT < WDCACHE.len() {
                        WDCACHE[WDCACHE_COUNT] = i;
                        WDCACHE_COUNT += 1;
                    } else {
                        let stale = &mut WD[WDCACHE[WDCACHE.len() - 1]];
                        if let Some(fd) = stale.fd {
                            close(fd).unwrap_or_else(|_| {
                                eprintln!("Failed to close directory: {}", stale.name);
                            });
                        }
                        stale.fd = None;
                        WDCACHE[WDCACHE.len() - 1] = i;
                    }

                    // Move to front of cache
                    for ci in 1..WDCACHE_COUNT {
                        if WDCACHE[ci] == i {
                            WDCACHE.swap(0, ci);
                            break;
                        }
                    }

                    fd
                }
            };

            CHDIR_CURRENT = i;
            CHDIR_FD = fd;
        }
    }
}

fn tar_dirname() -> String {
    unsafe { WD[CHDIR_CURRENT].name.clone() }
}

fn tar_getcdpath(idx: usize) -> String {
    unsafe {
        if WD.is_empty() {
            static mut CWD: Option<String> = None;
            if CWD.is_none() {
                CWD = Some(
                    getcwd()
                        .unwrap_or_else(|_| panic!("Failed to get current working directory"))
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            return CWD.as_ref().unwrap().clone();
        }

        if WD[idx].abspath.is_none() {
            let mut i = idx;
            while i > 0 && WD[i].abspath.is_none() {
                i -= 1;
            }

            let save_cwdi = CHDIR_CURRENT;
            for i in i + 1..=idx {
                chdir_do(i);
                if i == 0 {
                    WD[i].abspath = Some(
                        getcwd()
                            .unwrap_or_else(|_| panic!("Failed to get current working directory"))
                            .to_string_lossy()
                            .into_owned(),
                    );
                } else if Path::new(&WD[i].name).is_absolute() {
                    WD[i].abspath = Some(WD[i].name.clone());
                } else {
                    let mut path = PathBuf::from(WD[i - 1].abspath.as_ref().unwrap());
                    path.push(&WD[i].name);
                    WD[i].abspath = Some(path.to_string_lossy().into_owned());
                }
            }
            chdir_do(save_cwdi);
        }

        WD[idx].abspath.as_ref().unwrap().clone()
    }
}

struct NameBuf {
    buffer: String,
    dir_length: usize,
}

fn namebuf_create(dir: &str) -> NameBuf {
    let mut buffer = dir.to_string();
    let dir_length = buffer.len();
    if !buffer.ends_with('/') {
        buffer.push('/');
    }
    NameBuf {
        buffer,
        dir_length: buffer.len(),
    }
}

fn namebuf_name(buf: &mut NameBuf, name: &str) -> &str {
    buf.buffer.truncate(buf.dir_length);
    buf.buffer.push_str(name);
    &buf.buffer
}

fn namebuf_add_dir(buf: &mut NameBuf, name: &str) {
    if !buf.buffer.ends_with('/') {
        buf.buffer.push('/');
        buf.dir_length += 1;
    }
    buf.buffer.push_str(name);
    buf.dir_length += name.len();
}

fn namebuf_finish(buf: NameBuf) -> String {
    let mut s = buf.buffer;
    if s.ends_with('/') {
        s.pop();
    }
    s
}

fn tar_savedir(name: &str, must_exist: bool) -> Option<Vec<String>> {
    let fd = unsafe {
        openat(
            CHDIR_FD,
            name.as_ptr() as *const c_char,
            libc::O_RDONLY | libc::O_DIRECTORY,
            0,
        )
    };

    if fd < 0 {
        if !must_exist && std::io::Error::last_os_error().raw_os_error() == Some(libc::ENOENT) {
            return None;
        }
        panic!("Failed to open directory: {}", name);
    }

    let dir = unsafe { libc::fdopendir(fd) };
    if dir.is_null() {
        unsafe { libc::close(fd) };
        panic!("Failed to open directory stream: {}", name);
    }

    let mut entries = Vec::new();
    loop {
        let entry = unsafe { libc::readdir(dir) };
        if entry.is_null() {
            break;
        }

        let name_cstr = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) };
        let name = name_cstr.to_string_lossy().into_owned();
        if name != "." && name != ".." {
            entries.push(name);
        }
    }

    unsafe { libc::closedir(dir) };
    Some(entries)
}

// Additional helper functions would be implemented similarly