use std::ffi::CString;
use std::os::unix::fs::PermissionsExt;
use std::io::{Error, ErrorKind};
use std::fmt;

// Error hook function pointer
pub static mut ERROR_HOOK: Option<fn()> = None;

/// Decode MODE from its binary form and encode it into a 9-byte String
pub fn pax_decode_mode(mode: u32) -> String {
    let mut s = String::with_capacity(9);
    
    s.push(if mode & libc::S_IRUSR != 0 { 'r' } else { '-' });
    s.push(if mode & libc::S_IWUSR != 0 { 'w' } else { '-' });
    s.push(if mode & libc::S_ISUID != 0 {
        if mode & libc::S_IXUSR != 0 { 's' } else { 'S' }
    } else {
        if mode & libc::S_IXUSR != 0 { 'x' } else { '-' }
    });
    
    s.push(if mode & libc::S_IRGRP != 0 { 'r' } else { '-' });
    s.push(if mode & libc::S_IWGRP != 0 { 'w' } else { '-' });
    s.push(if mode & libc::S_ISGID != 0 {
        if mode & libc::S_IXGRP != 0 { 's' } else { 'S' }
    } else {
        if mode & libc::S_IXGRP != 0 { 'x' } else { '-' }
    });
    
    s.push(if mode & libc::S_IROTH != 0 { 'r' } else { '-' });
    s.push(if mode & libc::S_IWOTH != 0 { 'w' } else { '-' });
    s.push(if mode & libc::S_ISVTX != 0 {
        if mode & libc::S_IXOTH != 0 { 't' } else { 'T' }
    } else {
        if mode & libc::S_IXOTH != 0 { 'x' } else { '-' }
    });
    
    s
}

/// Report an error associated with the system call and optional name
pub fn call_arg_error(call: &str, name: Option<&str>) -> std::io::Result<()> {
    let e = Error::last_os_error();
    let msg = match name {
        Some(n) => format!("{}: Cannot {}", n, call),
        None => format!("Cannot {}", call),
    };
    Err(Error::new(e.kind(), msg))
}

/// Report a fatal error associated with the system call and optional name
pub fn call_arg_fatal(call: &str, name: Option<&str>) -> ! {
    if let Err(e) = call_arg_error(call, name) {
        panic!("{}", e);
    }
    panic!("Fatal error occurred");
}

/// Report a warning associated with the system call and optional name
pub fn call_arg_warn(call: &str, name: Option<&str>) {
    if let Err(e) = call_arg_error(call, name) {
        eprintln!("Warning: {}", e);
    }
}

pub fn chmod_error_details(name: &str, mode: u32) -> std::io::Result<()> {
    let buf = pax_decode_mode(mode);
    let e = Error::last_os_error();
    Err(Error::new(e.kind(), format!("{}: Cannot change mode to {}", name, buf)))
}

pub fn chown_error_details(name: &str, uid: u32, gid: u32) -> std::io::Result<()> {
    let e = Error::last_os_error();
    Err(Error::new(e.kind(), format!("{}: Cannot change ownership to uid {}, gid {}", name, uid, gid)))
}

pub fn close_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("close", name)
}

pub fn close_warn(name: Option<&str>) {
    call_arg_warn("close", name)
}

pub fn exec_fatal(name: Option<&str>) -> ! {
    call_arg_fatal("exec", name)
}

pub fn link_error(target: &str, source: &str) -> std::io::Result<()> {
    let e = Error::last_os_error();
    Err(Error::new(e.kind(), format!("{}: Cannot hard link to {}", source, target)))
}

pub fn mkdir_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("mkdir", name)
}

pub fn mkfifo_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("mkfifo", name)
}

pub fn mknod_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("mknod", name)
}

pub fn open_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("open", name)
}

pub fn open_fatal(name: Option<&str>) -> ! {
    call_arg_fatal("open", name)
}

pub fn open_warn(name: Option<&str>) {
    call_arg_warn("open", name)
}

pub fn read_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("read", name)
}

pub fn read_error_details(name: &str, offset: u64, size: usize) -> std::io::Result<()> {
    let e = Error::last_os_error();
    let msg = if size == 1 {
        format!("{}: Read error at byte {}, while reading 1 byte", name, offset)
    } else {
        format!("{}: Read error at byte {}, while reading {} bytes", name, offset, size)
    };
    Err(Error::new(e.kind(), msg))
}

pub fn read_warn_details(name: &str, offset: u64, size: usize) {
    let msg = if size == 1 {
        format!("{}: Warning: Read error at byte {}, while reading 1 byte", name, offset)
    } else {
        format!("{}: Warning: Read error at byte {}, while reading {} bytes", name, offset, size)
    };
    eprintln!("{}", msg);
}

pub fn read_fatal(name: Option<&str>) -> ! {
    call_arg_fatal("read", name)
}

pub fn read_fatal_details(name: &str, offset: u64, size: usize) -> ! {
    let e = Error::last_os_error();
    let msg = if size == 1 {
        format!("{}: Read error at byte {}, while reading 1 byte", name, offset)
    } else {
        format!("{}: Read error at byte {}, while reading {} bytes", name, offset, size)
    };
    panic!("{}: {}", e, msg);
}

pub fn readlink_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("readlink", name)
}

pub fn readlink_warn(name: Option<&str>) {
    call_arg_warn("readlink", name)
}

pub fn rmdir_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("rmdir", name)
}

pub fn savedir_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("savedir", name)
}

pub fn savedir_warn(name: Option<&str>) {
    call_arg_warn("savedir", name)
}

pub fn seek_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("seek", name)
}

pub fn seek_error_details(name: &str, offset: u64) -> std::io::Result<()> {
    let e = Error::last_os_error();
    Err(Error::new(e.kind(), format!("{}: Cannot seek to {}", name, offset)))
}

pub fn seek_warn(name: Option<&str>) {
    call_arg_warn("seek", name)
}

pub fn seek_warn_details(name: &str, offset: u64) {
    eprintln!("{}: Warning: Cannot seek to {}", name, offset);
}

pub fn symlink_error(contents: &str, name: &str) -> std::io::Result<()> {
    let e = Error::last_os_error();
    Err(Error::new(e.kind(), format!("{}: Cannot create symlink to {}", name, contents)))
}

pub fn stat_fatal(name: Option<&str>) -> ! {
    call_arg_fatal("stat", name)
}

pub fn stat_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("stat", name)
}

pub fn stat_warn(name: Option<&str>) {
    call_arg_warn("stat", name)
}

pub fn truncate_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("truncate", name)
}

pub fn truncate_warn(name: Option<&str>) {
    call_arg_warn("truncate", name)
}

pub fn unlink_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("unlink", name)
}

pub fn utime_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("utime", name)
}

pub fn waitpid_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("waitpid", name)
}

pub fn write_error(name: Option<&str>) -> std::io::Result<()> {
    call_arg_error("write", name)
}

pub fn write_error_details(name: &str, status: usize, size: usize) -> std::io::Result<()> {
    if status == 0 {
        write_error(Some(name))
    } else {
        let msg = if size == 1 {
            format!("{}: Wrote only {} of 1 byte", name, status)
        } else {
            format!("{}: Wrote only {} of {} bytes", name, status, size)
        };
        Err(Error::new(ErrorKind::Other, msg))
    }
}

pub fn chdir_fatal(name: Option<&str>) -> ! {
    call_arg_fatal("chdir", name)
}