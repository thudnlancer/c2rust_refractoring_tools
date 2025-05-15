//! A Rust implementation of the unistd.h functionality

use libc::{c_int, c_char, size_t, ssize_t, off_t, uid_t, gid_t, pid_t};
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::ptr;

// Constants
pub const STDIN_FILENO: RawFd = 0;
pub const STDOUT_FILENO: RawFd = 1;
pub const STDERR_FILENO: RawFd = 2;

pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;

// File operations
pub fn access(path: &str, mode: c_int) -> c_int {
    let c_path = CString::new(path).unwrap();
    unsafe { libc::access(c_path.as_ptr(), mode) }
}

pub fn chdir(path: &str) -> c_int {
    let c_path = CString::new(path).unwrap();
    unsafe { libc::chdir(c_path.as_ptr()) }
}

pub fn chown(path: &str, owner: uid_t, group: gid_t) -> c_int {
    let c_path = CString::new(path).unwrap();
    unsafe { libc::chown(c_path.as_ptr(), owner, group) }
}

pub fn close(fd: RawFd) -> c_int {
    unsafe { libc::close(fd) }
}

pub fn dup(oldfd: RawFd) -> RawFd {
    unsafe { libc::dup(oldfd) }
}

pub fn dup2(oldfd: RawFd, newfd: RawFd) -> RawFd {
    unsafe { libc::dup2(oldfd, newfd) }
}

// Process operations
pub fn getpid() -> pid_t {
    unsafe { libc::getpid() }
}

// File I/O
pub fn read(fd: RawFd, buf: &mut [u8]) -> ssize_t {
    unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) }
}

pub fn write(fd: RawFd, buf: &[u8]) -> ssize_t {
    unsafe { libc::write(fd, buf.as_ptr() as *const libc::c_void, buf.len()) }
}

pub fn lseek(fd: RawFd, offset: off_t, whence: c_int) -> off_t {
    unsafe { libc::lseek(fd, offset, whence) }
}

// Directory operations
pub fn getcwd(buf: &mut [u8]) -> Option<&str> {
    unsafe {
        if libc::getcwd(buf.as_mut_ptr() as *mut c_char, buf.len()).is_null() {
            None
        } else {
            Some(std::str::from_utf8_unchecked(buf))
        }
    }
}

// Process control
pub fn sleep(seconds: u32) -> u32 {
    unsafe { libc::sleep(seconds) }
}

// Symbolic links
pub fn symlink(target: &str, linkpath: &str) -> c_int {
    let c_target = CString::new(target).unwrap();
    let c_linkpath = CString::new(linkpath).unwrap();
    unsafe { libc::symlink(c_target.as_ptr(), c_linkpath.as_ptr()) }
}

pub fn readlink(path: &str, buf: &mut [u8]) -> ssize_t {
    let c_path = CString::new(path).unwrap();
    unsafe { libc::readlink(c_path.as_ptr(), buf.as_mut_ptr() as *mut c_char, buf.len()) }
}

// File system operations
pub fn unlink(path: &str) -> c_int {
    let c_path = CString::new(path).unwrap();
    unsafe { libc::unlink(c_path.as_ptr()) }
}

pub fn rmdir(path: &str) -> c_int {
    let c_path = CString::new(path).unwrap();
    unsafe { libc::rmdir(c_path.as_ptr()) }
}

// Environment
pub fn environ() -> *mut *mut c_char {
    unsafe { libc::environ }
}

// Pipe operations
pub fn pipe(fds: &mut [RawFd; 2]) -> c_int {
    unsafe { libc::pipe(fds.as_mut_ptr()) }
}

// Terminal operations
pub fn isatty(fd: RawFd) -> c_int {
    unsafe { libc::isatty(fd) }
}

// Process execution
pub fn execvp(file: &str, argv: &[*const c_char]) -> c_int {
    let c_file = CString::new(file).unwrap();
    unsafe { libc::execvp(c_file.as_ptr(), argv.as_ptr()) }
}

// System configuration
pub fn sysconf(name: c_int) -> i64 {
    unsafe { libc::sysconf(name) }
}