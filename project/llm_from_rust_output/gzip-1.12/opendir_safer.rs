use std::ffi::CString;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::io;
use libc::{self, O_CLOEXEC};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::dir::Dir;
use nix::errno::errno;

pub fn opendir_safer(name: &CString) -> io::Result<Dir> {
    let dir = Dir::open(name, nix::fcntl::OFlag::empty(), nix::sys::stat::Mode::empty())?;
    
    let fd = dir.as_raw_fd();
    if fd >= 0 && fd <= 2 {
        let flags = fcntl(fd, FcntlArg::F_GETFD)?;
        let new_flags = FdFlag::from_bits_truncate(flags) | FdFlag::FD_CLOEXEC;
        fcntl(fd, FcntlArg::F_SETFD(new_flags))?;
        
        let file = unsafe { File::from_raw_fd(fd) };
        let new_dir = Dir::from_fd(file.into_raw_fd())?;
        drop(dir);
        Ok(new_dir)
    } else {
        Ok(dir)
    }
}