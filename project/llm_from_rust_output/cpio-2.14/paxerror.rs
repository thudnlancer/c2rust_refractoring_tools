use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_long, c_ulong};
use std::ptr;

static mut exit_status: c_int = 0;
static mut error_hook: Option<fn()> = None;

type uid_t = c_uint;
type gid_t = c_uint;
type mode_t = c_uint;
type off_t = c_long;
type size_t = c_ulong;
type intmax_t = c_long;
type uintmax_t = c_ulong;

fn pax_decode_mode(mode: mode_t) -> String {
    let mut result = String::with_capacity(10);
    
    result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    result.push(match (mode & 0o4000, mode & 0o100) {
        (0, 0) => '-',
        (0, _) => 'x',
        (_, 0) => 'S',
        _ => 's',
    });
    
    result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    result.push(match (mode & 0o2000, mode & 0o010) {
        (0, 0) => '-',
        (0, _) => 'x',
        (_, 0) => 'S',
        _ => 's',
    });
    
    result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    result.push(match (mode & 0o1000, mode & 0o001) {
        (0, 0) => '-',
        (0, _) => 'x',
        (_, 0) => 'T',
        _ => 't',
    });
    
    result
}

fn call_arg_error(call: &str, name: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Cannot {}: {}", name, call, e);
    unsafe { exit_status = 2 };
}

fn call_arg_fatal(call: &str, name: &str) {
    call_arg_error(call, name);
    std::process::exit(1);
}

fn call_arg_warn(call: &str, name: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Warning: Cannot {}: {}", name, call, e);
}

fn chmod_error_details(name: &str, mode: mode_t) {
    let mode_str = pax_decode_mode(mode);
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Cannot change mode to {}: {}", name, mode_str, e);
    unsafe { exit_status = 2 };
}

fn chown_error_details(name: &str, uid: uid_t, gid: gid_t) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Cannot change ownership to uid {}, gid {}: {}", name, uid, gid, e);
    unsafe { exit_status = 2 };
}

// Wrapper functions for common operations
fn close_error(name: &str) { call_arg_error("close", name); }
fn close_warn(name: &str) { call_arg_warn("close", name); }
fn exec_fatal(name: &str) { call_arg_fatal("exec", name); }
fn link_error(target: &str, source: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Cannot hard link to {}: {}", source, target, e);
    unsafe { exit_status = 2 };
}
fn mkdir_error(name: &str) { call_arg_error("mkdir", name); }
fn mkfifo_error(name: &str) { call_arg_error("mkfifo", name); }
fn mknod_error(name: &str) { call_arg_error("mknod", name); }
fn open_error(name: &str) { call_arg_error("open", name); }
fn open_fatal(name: &str) { call_arg_fatal("open", name); }
fn open_warn(name: &str) { call_arg_warn("open", name); }
fn read_error(name: &str) { call_arg_error("read", name); }
fn read_fatal(name: &str) { call_arg_fatal("read", name); }
fn readlink_error(name: &str) { call_arg_error("readlink", name); }
fn readlink_warn(name: &str) { call_arg_warn("readlink", name); }
fn rmdir_error(name: &str) { call_arg_error("rmdir", name); }
fn savedir_error(name: &str) { call_arg_error("savedir", name); }
fn savedir_warn(name: &str) { call_arg_warn("savedir", name); }
fn seek_error(name: &str) { call_arg_error("seek", name); }
fn seek_warn(name: &str) { call_arg_warn("seek", name); }
fn stat_fatal(name: &str) { call_arg_fatal("stat", name); }
fn stat_error(name: &str) { call_arg_error("stat", name); }
fn stat_warn(name: &str) { call_arg_warn("stat", name); }
fn truncate_error(name: &str) { call_arg_error("truncate", name); }
fn truncate_warn(name: &str) { call_arg_warn("truncate", name); }
fn unlink_error(name: &str) { call_arg_error("unlink", name); }
fn utime_error(name: &str) { call_arg_error("utime", name); }
fn waitpid_error(name: &str) { call_arg_error("waitpid", name); }
fn write_error(name: &str) { call_arg_error("write", name); }
fn chdir_fatal(name: &str) { call_arg_fatal("chdir", name); }

fn read_error_details(name: &str, offset: off_t, size: size_t) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    let msg = if size == 1 {
        format!("{}: Read error at byte {}, while reading {} byte: {}", name, offset, size, e)
    } else {
        format!("{}: Read error at byte {}, while reading {} bytes: {}", name, offset, size, e)
    };
    eprintln!("{}", msg);
    unsafe { exit_status = 2 };
}

fn read_warn_details(name: &str, offset: off_t, size: size_t) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    let msg = if size == 1 {
        format!("{}: Warning: Read error at byte {}, while reading {} byte: {}", name, offset, size, e)
    } else {
        format!("{}: Warning: Read error at byte {}, while reading {} bytes: {}", name, offset, size, e)
    };
    eprintln!("{}", msg);
}

fn read_fatal_details(name: &str, offset: off_t, size: size_t) {
    read_error_details(name, offset, size);
    std::process::exit(1);
}

fn seek_error_details(name: &str, offset: off_t) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Cannot seek to {}: {}", name, offset, e);
    unsafe { exit_status = 2 };
}

fn seek_warn_details(name: &str, offset: off_t) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Warning: Cannot seek to {}: {}", name, offset, e);
}

fn symlink_error(contents: &str, name: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { error_hook } {
        hook();
    }
    eprintln!("{}: Cannot create symlink to {}: {}", name, contents, e);
    unsafe { exit_status = 2 };
}

fn write_error_details(name: &str, status: size_t, size: size_t) {
    if status == 0 {
        write_error(name);
    } else {
        if let Some(hook) = unsafe { error_hook } {
            hook();
        }
        let msg = if size == 1 {
            format!("{}: Wrote only {} of {} byte", name, status, size)
        } else {
            format!("{}: Wrote only {} of {} bytes", name, status, size)
        };
        eprintln!("{}", msg);
        unsafe { exit_status = 2 };
    }
}