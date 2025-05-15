// This is a Rust translation of the provided C header file unistd.h
// It provides equivalent functionality while following Rust's safety practices

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{c_int, c_char, c_void, size_t, ssize_t, off_t, uid_t, gid_t, pid_t};
use std::ffi::CString;
use std::os::raw;
use std::ptr;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;

#[cfg(target_os = "windows")]
pub fn access(path: *const c_char, mode: c_int) -> c_int {
    unsafe { libc::_access(path, mode) }
}

#[cfg(not(target_os = "windows"))]
pub fn access(path: *const c_char, mode: c_int) -> c_int {
    unsafe { libc::access(path, mode) }
}

#[cfg(target_os = "windows")]
pub fn chdir(dir: *const c_char) -> c_int {
    unsafe { libc::_chdir(dir) }
}

#[cfg(not(target_os = "windows"))]
pub fn chdir(dir: *const c_char) -> c_int {
    unsafe { libc::chdir(dir) }
}

pub fn close(fd: c_int) -> c_int {
    unsafe { libc::close(fd) }
}

pub fn dup(oldfd: c_int) -> c_int {
    unsafe { libc::dup(oldfd) }
}

pub fn dup2(oldfd: c_int, newfd: c_int) -> c_int {
    unsafe { libc::dup2(oldfd, newfd) }
}

pub fn fsync(fd: c_int) -> c_int {
    unsafe { libc::fsync(fd) }
}

#[cfg(target_os = "windows")]
pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    unsafe { libc::_getcwd(buf, size as c_int) }
}

#[cfg(not(target_os = "windows"))]
pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    unsafe { libc::getcwd(buf, size) }
}

pub fn getdtablesize() -> c_int {
    unsafe { libc::getdtablesize() }
}

pub fn getpid() -> pid_t {
    unsafe { libc::getpid() }
}

#[cfg(target_os = "windows")]
pub fn isatty(fd: c_int) -> c_int {
    unsafe { libc::_isatty(fd) }
}

#[cfg(not(target_os = "windows"))]
pub fn isatty(fd: c_int) -> c_int {
    unsafe { libc::isatty(fd) }
}

pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unsafe { libc::lseek(fd, offset, whence) }
}

pub fn pipe(fds: *mut c_int) -> c_int {
    unsafe { libc::pipe(fds) }
}

pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unsafe { libc::read(fd, buf, count) }
}

#[cfg(target_os = "windows")]
pub fn rmdir(path: *const c_char) -> c_int {
    unsafe { libc::_rmdir(path) }
}

#[cfg(not(target_os = "windows"))]
pub fn rmdir(path: *const c_char) -> c_int {
    unsafe { libc::rmdir(path) }
}

pub fn unlink(path: *const c_char) -> c_int {
    unsafe { libc::unlink(path) }
}

pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    unsafe { libc::write(fd, buf, count) }
}

// Environment variable functions
pub unsafe extern "C" fn getenv(name: *const c_char) -> *mut c_char {
    libc::getenv(name)
}

pub unsafe extern "C" fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int {
    libc::setenv(name, value, overwrite)
}

pub unsafe extern "C" fn unsetenv(name: *const c_char) -> c_int {
    libc::unsetenv(name)
}

// Process functions
pub fn fork() -> pid_t {
    unsafe { libc::fork() }
}

pub fn execvp(file: *const c_char, argv: *const *const c_char) -> c_int {
    unsafe { libc::execvp(file, argv) }
}

pub fn execv(file: *const c_char, argv: *const *const c_char) -> c_int {
    unsafe { libc::execv(file, argv) }
}

pub fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int {
    unsafe {
        let mut args = ptr::null();
        libc::execlp(file, arg, args)
    }
}

pub fn execl(file: *const c_char, arg: *const c_char, ...) -> c_int {
    unsafe {
        let mut args = ptr::null();
        libc::execl(file, arg, args)
    }
}

pub fn execle(file: *const c_char, arg: *const c_char, ...) -> c_int {
    unsafe {
        let mut args = ptr::null();
        libc::execle(file, arg, args)
    }
}

pub fn execve(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int {
    unsafe { libc::execve(file, argv, envp) }
}

pub fn execvpe(file: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int {
    unsafe { libc::execve(file, argv, envp) }
}

pub fn sleep(seconds: u32) -> u32 {
    unsafe { libc::sleep(seconds) }
}

pub fn usleep(usec: u32) -> c_int {
    unsafe { libc::usleep(usec) }
}

pub fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    unsafe { libc::chown(path, owner, group) }
}

pub fn fchown(fd: c_int, owner: uid_t, group: gid_t) -> c_int {
    unsafe { libc::fchown(fd, owner, group) }
}

pub fn lchown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    unsafe { libc::lchown(path, owner, group) }
}

pub fn ftruncate(fd: c_int, length: off_t) -> c_int {
    unsafe { libc::ftruncate(fd, length) }
}

pub fn truncate(path: *const c_char, length: off_t) -> c_int {
    unsafe { libc::truncate(path, length) }
}

pub fn getgroups(size: c_int, list: *mut gid_t) -> c_int {
    unsafe { libc::getgroups(size, list) }
}

pub fn setgroups(size: size_t, list: *const gid_t) -> c_int {
    unsafe { libc::setgroups(size, list) }
}

pub fn getlogin() -> *mut c_char {
    unsafe { libc::getlogin() }
}

pub fn getlogin_r(name: *mut c_char, size: size_t) -> c_int {
    unsafe { libc::getlogin_r(name, size) }
}

pub fn gethostname(name: *mut c_char, len: size_t) -> c_int {
    unsafe { libc::gethostname(name, len) }
}

pub fn sethostname(name: *const c_char, len: size_t) -> c_int {
    unsafe { libc::sethostname(name, len) }
}

pub fn getdomainname(name: *mut c_char, len: size_t) -> c_int {
    unsafe { libc::getdomainname(name, len) }
}

pub fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int {
    unsafe { libc::symlink(target, linkpath) }
}

pub fn readlink(path: *const c_char, buf: *mut c_char, bufsize: size_t) -> ssize_t {
    unsafe { libc::readlink(path, buf, bufsize) }
}

pub fn link(existing: *const c_char, new: *const c_char) -> c_int {
    unsafe { libc::link(existing, new) }
}

pub fn getpagesize() -> c_int {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as c_int }
}

pub fn getuid() -> uid_t {
    unsafe { libc::getuid() }
}

pub fn geteuid() -> uid_t {
    unsafe { libc::geteuid() }
}

pub fn getgid() -> gid_t {
    unsafe { libc::getgid() }
}

pub fn getegid() -> gid_t {
    unsafe { libc::getegid() }
}

pub fn setuid(uid: uid_t) -> c_int {
    unsafe { libc::setuid(uid) }
}

pub fn seteuid(uid: uid_t) -> c_int {
    unsafe { libc::seteuid(uid) }
}

pub fn setgid(gid: gid_t) -> c_int {
    unsafe { libc::setgid(gid) }
}

pub fn setegid(gid: gid_t) -> c_int {
    unsafe { libc::setegid(gid) }
}

pub fn getpgrp() -> pid_t {
    unsafe { libc::getpgrp() }
}

pub fn getpgid(pid: pid_t) -> pid_t {
    unsafe { libc::getpgid(pid) }
}

pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
    unsafe { libc::setpgid(pid, pgid) }
}

pub fn setsid() -> pid_t {
    unsafe { libc::setsid() }
}

pub fn tcgetpgrp(fd: c_int) -> pid_t {
    unsafe { libc::tcgetpgrp(fd) }
}

pub fn tcsetpgrp(fd: c_int, pgrp: pid_t) -> c_int {
    unsafe { libc::tcsetpgrp(fd, pgrp) }
}