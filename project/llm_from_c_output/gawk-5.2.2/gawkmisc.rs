use std::env;
use std::ffi::{CStr, CString};
use std::fs::{File, Metadata};
use std::io;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::os::unix::io::{AsRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::sys::stat::{fstat, FileStat, SFlag};
use libc::{mode_t, stat};

const QUOTE: char = '\'';
const DEFPATH: &str = "/usr/local/bin:/usr/bin:/bin";
const DEFLIBPATH: &str = "/usr/local/lib:/usr/lib:/lib";
const ENVSEP: char = ':';
const INVALID_HANDLE: RawFd = -1;

/// Pull out the "gawk" part from how the OS called us
pub fn gawk_name(filespec: &str) -> &str {
    filespec.rsplit('/').next().unwrap_or(filespec)
}

/// Fixup the command line (no-op in Rust)
pub fn os_arg_fixup(_argc: &mut i32, _argv: &mut Vec<String>) {
    // no-op
}

/// Open special per-OS devices (no-op in Rust)
pub fn os_devopen(_name: &str, _flag: i32) -> RawFd {
    INVALID_HANDLE
}

/// Determine optimal buffer size
pub fn optimal_bufsize(fd: RawFd, stb: &mut stat) -> io::Result<usize> {
    unsafe {
        std::ptr::write_bytes(stb, 0, 1);
    }
    
    let stat = fstat(fd)?;
    unsafe {
        std::ptr::copy_nonoverlapping(&stat as *const _ as *const stat, stb, 1);
    }

    static mut ENV_VAL: usize = 0;
    static mut FIRST: bool = true;
    static mut EXACT: bool = false;

    unsafe {
        if FIRST {
            FIRST = false;
            
            if let Ok(val) = env::var("AWKBUFSIZE") {
                if val == "exact" {
                    EXACT = true;
                } else if val.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                    ENV_VAL = val.parse().unwrap_or(0);
                    return Ok(ENV_VAL);
                }
            }
        } else if !EXACT && ENV_VAL > 0 {
            return Ok(ENV_VAL);
        }
    }

    let def_blocksize = if cfg!(target_os = "solaris") {
        8192 // Common Solaris block size
    } else {
        4096 // Common Linux/Unix block size
    };

    let st_mode = unsafe { stb.st_mode };
    let st_size = unsafe { stb.st_size };

    if SFlag::from_bits_truncate(st_mode).contains(SFlag::S_IFREG) 
        && st_size > 0 
        && (st_size < def_blocksize as i64 || unsafe { EXACT })
    {
        return Ok(st_size as usize);
    }

    Ok(def_blocksize)
}

/// Return true if path has directory components
pub fn ispath(file: &str) -> bool {
    file.contains('/')
}

/// Return true if char is a directory separator
pub fn isdirpunct(c: char) -> bool {
    c == '/'
}

/// Set close on exec flag, print warning if fails
pub fn os_close_on_exec(fd: RawFd, name: &str, what: &str, dir: &str) {
    if fd <= 2 {
        return;
    }

    match fcntl(fd, FcntlArg::F_GETFD) {
        Ok(flags) => {
            let new_flags = FdFlag::from_bits_truncate(flags) | FdFlag::FD_CLOEXEC;
            if let Err(e) = fcntl(fd, FcntlArg::F_SETFD(new_flags)) {
                eprintln!("{} {} `{}': could not set close-on-exec: {}", 
                    what, dir, name, e);
            }
        }
        Err(e) => {
            eprintln!("{} {} `{}': could not get fd flags: {}", 
                what, dir, name, e);
        }
    }
}

/// Is this an fd on a directory?
pub fn os_isdir(fd: RawFd) -> bool {
    match fstat(fd) {
        Ok(stat) => SFlag::from_bits_truncate(stat.st_mode).contains(SFlag::S_IFDIR),
        Err(_) => false,
    }
}

/// fd can be read from
pub fn os_isreadable(iobuf: &AwkInputBuf, isdir: &mut bool) -> bool {
    *isdir = false;

    if iobuf.fd == INVALID_HANDLE {
        return false;
    }

    match SFlag::from_bits_truncate(iobuf.sbuf.st_mode) {
        SFlag::S_IFREG | SFlag::S_IFCHR => true,
        SFlag::S_IFDIR => {
            *isdir = true;
            false
        }
        _ => false,
    }
}

/// True if running setuid root
pub fn os_is_setuid() -> bool {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    euid == 0 && euid != uid
}

/// Set binary mode on file (no-op on Unix)
pub fn os_setbinmode(_fd: RawFd, _mode: i32) -> i32 {
    0
}

/// Restore the original mode of the console device (no-op)
pub fn os_restore_mode(_fd: RawFd) {
    // no-op
}

/// Return true if fd is a tty
pub fn os_isatty(fd: RawFd) -> bool {
    unsafe { libc::isatty(fd) != 0 }
}

/// Return true if files are identical
pub fn files_are_same(path: &str, src: &SrcFile) -> bool {
    match std::fs::metadata(path) {
        Ok(metadata) => {
            metadata.dev() == src.sbuf.st_dev as u64 && 
            metadata.ino() == src.sbuf.st_ino as u64
        }
        Err(_) => false,
    }
}

/// Initialize sockets (no-op)
pub fn init_sockets() {
    // no-op
}

/// Maybe set errno (no-op)
pub fn os_maybe_set_errno() {
    // no-op
}

// Types to match the C structs
#[repr(C)]
pub struct AwkInputBuf {
    pub fd: RawFd,
    pub sbuf: stat,
}

#[repr(C)]
pub struct SrcFile {
    pub sbuf: stat,
}