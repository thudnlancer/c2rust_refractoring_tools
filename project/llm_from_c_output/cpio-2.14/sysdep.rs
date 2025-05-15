/* -*- buffer-read-only: t -*- vi: set ro:
   THIS FILE IS GENERATED AUTOMATICALLY.  PLEASE DO NOT EDIT.
*/

/* System dependent functions for GNU cpio.

   Copyright (C) 2007-2023 Free Software Foundation, Inc.

   GNU cpio is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   GNU cpio is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with GNU cpiio.  If not, see <http://www.gnu.org/licenses/>. */

use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::ptr;
use std::io::{Error, ErrorKind};
use libc::{uid_t, gid_t};

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
#[derive(Debug)]
pub struct Passwd {
    pub pw_name: String,
    pub pw_passwd: String,
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_gecos: String,
    pub pw_dir: String,
    pub pw_shell: String,
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
#[derive(Debug)]
pub struct Group {
    pub gr_name: String,
    pub gr_passwd: String,
    pub gr_gid: gid_t,
    pub gr_mem: Vec<String>,
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn getpwuid(uid: uid_t) -> Result<Option<Passwd>, Error> {
    Err(Error::new(ErrorKind::Unsupported, "getpwuid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn getpwnam(name: &str) -> Result<Option<Passwd>, Error> {
    Err(Error::new(ErrorKind::Unsupported, "getpwnam not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn getgrgid(gid: gid_t) -> Result<Option<Group>, Error> {
    Err(Error::new(ErrorKind::Unsupported, "getgrgid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn getgrnam(name: &str) -> Result<Option<Group>, Error> {
    Err(Error::new(ErrorKind::Unsupported, "getgrnam not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn pipe() -> Result<(i32, i32), Error> {
    Err(Error::new(ErrorKind::Unsupported, "pipe not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn fork() -> Result<u32, Error> {
    Err(Error::new(ErrorKind::Unsupported, "fork not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn getuid() -> Result<uid_t, Error> {
    Err(Error::new(ErrorKind::Unsupported, "getuid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn geteuid() -> Result<uid_t, Error> {
    Err(Error::new(ErrorKind::Unsupported, "geteuid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn getgid() -> Result<gid_t, Error> {
    Err(Error::new(ErrorKind::Unsupported, "getgid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn setuid(uid: uid_t) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Unsupported, "setuid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn setgid(gid: gid_t) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Unsupported, "setgid not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn mknod(path: &str, mode: u32, dev: u64) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Unsupported, "mknod not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn symlink(oldname: &str, newname: &str) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Unsupported, "symlink not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn link(oldname: &str, newname: &str) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Unsupported, "link not implemented"))
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
pub fn chown(path: &str, owner: uid_t, group: gid_t) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Unsupported, "chown not implemented"))
}