use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_long, c_ulong};
use std::ptr;
use std::process;

type UidT = c_uint;
type GidT = c_uint;
type ModeT = c_uint;
type OffT = c_long;
type SizeT = c_ulong;
type UintMaxT = c_ulong;

static mut EXIT_STATUS: c_int = 0;
static mut ERROR_HOOK: Option<fn()> = None;

fn pax_decode_mode(mode: ModeT) -> String {
    let mut s = String::with_capacity(9);
    
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o4000 != 0, mode & 0o100 != 0) {
        (true, true) => 's',
        (true, false) => 'S',
        (false, true) => 'x',
        _ => '-',
    });
    
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o2000 != 0, mode & 0o010 != 0) {
        (true, true) => 's',
        (true, false) => 'S',
        (false, true) => 'x',
        _ => '-',
    });
    
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(match (mode & 0o1000 != 0, mode & 0o001 != 0) {
        (true, true) => 't',
        (true, false) => 'T',
        (false, true) => 'x',
        _ => '-',
    });
    
    s
}

fn call_arg_error(call: &str, name: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot {}: {}", name, call, e);
    unsafe { EXIT_STATUS = 2 };
}

fn call_arg_fatal(call: &str, name: &str) -> ! {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot {}: {}", name, call, e);
    process::exit(2);
}

fn call_arg_warn(call: &str, name: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Warning: Cannot {}: {}", name, call, e);
}

fn chmod_error_details(name: &str, mode: ModeT) {
    let e = std::io::Error::last_os_error();
    let mode_str = pax_decode_mode(mode);
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot change mode to {}: {}", name, mode_str, e);
    unsafe { EXIT_STATUS = 2 };
}

fn chown_error_details(name: &str, uid: UidT, gid: GidT) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot change ownership to uid {}, gid {}: {}", name, uid, gid, e);
    unsafe { EXIT_STATUS = 2 };
}

fn close_error(name: &str) {
    call_arg_error("close", name);
}

fn close_warn(name: &str) {
    call_arg_warn("close", name);
}

fn exec_fatal(name: &str) -> ! {
    call_arg_fatal("exec", name);
}

fn link_error(target: &str, source: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot hard link to {}: {}", source, target, e);
    unsafe { EXIT_STATUS = 2 };
}

fn mkdir_error(name: &str) {
    call_arg_error("mkdir", name);
}

fn mkfifo_error(name: &str) {
    call_arg_error("mkfifo", name);
}

fn mknod_error(name: &str) {
    call_arg_error("mknod", name);
}

fn open_error(name: &str) {
    call_arg_error("open", name);
}

fn open_fatal(name: &str) -> ! {
    call_arg_fatal("open", name);
}

fn open_warn(name: &str) {
    call_arg_warn("open", name);
}

fn read_error(name: &str) {
    call_arg_error("read", name);
}

fn read_error_details(name: &str, offset: OffT, size: SizeT) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    let msg = if size == 1 {
        format!("{}: Read error at byte {}, while reading 1 byte: {}", name, offset, e)
    } else {
        format!("{}: Read error at byte {}, while reading {} bytes: {}", name, offset, size, e)
    };
    eprintln!("{}", msg);
    unsafe { EXIT_STATUS = 2 };
}

fn read_warn_details(name: &str, offset: OffT, size: SizeT) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    let msg = if size == 1 {
        format!("{}: Warning: Read error at byte {}, while reading 1 byte: {}", name, offset, e)
    } else {
        format!("{}: Warning: Read error at byte {}, while reading {} bytes: {}", name, offset, size, e)
    };
    eprintln!("{}", msg);
}

fn read_fatal(name: &str) -> ! {
    call_arg_fatal("read", name);
}

fn read_fatal_details(name: &str, offset: OffT, size: SizeT) -> ! {
    read_error_details(name, offset, size);
    process::exit(2);
}

fn readlink_error(name: &str) {
    call_arg_error("readlink", name);
}

fn readlink_warn(name: &str) {
    call_arg_warn("readlink", name);
}

fn rmdir_error(name: &str) {
    call_arg_error("rmdir", name);
}

fn savedir_error(name: &str) {
    call_arg_error("savedir", name);
}

fn savedir_warn(name: &str) {
    call_arg_warn("savedir", name);
}

fn seek_error(name: &str) {
    call_arg_error("seek", name);
}

fn seek_error_details(name: &str, offset: OffT) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot seek to {}: {}", name, offset, e);
    unsafe { EXIT_STATUS = 2 };
}

fn seek_warn(name: &str) {
    call_arg_warn("seek", name);
}

fn seek_warn_details(name: &str, offset: OffT) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Warning: Cannot seek to {}: {}", name, offset, e);
}

fn symlink_error(contents: &str, name: &str) {
    let e = std::io::Error::last_os_error();
    if let Some(hook) = unsafe { ERROR_HOOK } {
        hook();
    }
    eprintln!("{}: Cannot create symlink to {}: {}", name, contents, e);
    unsafe { EXIT_STATUS = 2 };
}

fn stat_fatal(name: &str) -> ! {
    call_arg_fatal("stat", name);
}

fn stat_error(name: &str) {
    call_arg_error("stat", name);
}

fn stat_warn(name: &str) {
    call_arg_warn("stat", name);
}

fn truncate_error(name: &str) {
    call_arg_error("truncate", name);
}

fn truncate_warn(name: &str) {
    call_arg_warn("truncate", name);
}

fn unlink_error(name: &str) {
    call_arg_error("unlink", name);
}

fn utime_error(name: &str) {
    call_arg_error("utime", name);
}

fn waitpid_error(name: &str) {
    call_arg_error("waitpid", name);
}

fn write_error(name: &str) {
    call_arg_error("write", name);
}

fn write_error_details(name: &str, status: SizeT, size: SizeT) {
    if status == 0 {
        write_error(name);
    } else {
        if let Some(hook) = unsafe { ERROR_HOOK } {
            hook();
        }
        let msg = if size == 1 {
            format!("{}: Wrote only {} of 1 byte", name, status)
        } else {
            format!("{}: Wrote only {} of {} bytes", name, status, size)
        };
        eprintln!("{}", msg);
        unsafe { EXIT_STATUS = 2 };
    }
}

fn chdir_fatal(name: &str) -> ! {
    call_arg_fatal("chdir", name);
}