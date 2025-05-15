use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io;
use libc::{O_DIRECTORY, O_RDONLY, O_CLOEXEC, O_NOCTTY, O_NONBLOCK};

#[derive(Debug)]
pub struct Dir(File);

pub fn opendirat(
    dir_fd: RawFd,
    dir: &Path,
    extra_flags: i32,
) -> io::Result<(Dir, RawFd)> {
    let open_flags = O_DIRECTORY | O_RDONLY | O_CLOEXEC | O_NOCTTY | O_NONBLOCK | extra_flags;
    
    let file = OpenOptions::new()
        .custom_flags(open_flags)
        .open_at(unsafe { File::from_raw_fd(dir_fd) }, dir)?;
    
    let new_fd = file.as_raw_fd();
    let dir = Dir(file);
    
    Ok((dir, new_fd))
}

trait OpenOptionsExt {
    fn custom_flags(&mut self, flags: i32) -> &mut Self;
    fn open_at(&self, dir: File, path: &Path) -> io::Result<File>;
}

impl OpenOptionsExt for OpenOptions {
    fn custom_flags(&mut self, flags: i32) -> &mut Self {
        self.mode((flags & 0o777) as u32);
        self
    }

    fn open_at(&self, dir: File, path: &Path) -> io::Result<File> {
        let fd = unsafe {
            libc::openat(
                dir.as_raw_fd(),
                path.as_os_str().as_bytes().as_ptr() as *const libc::c_char,
                self.create_new(false).create(false).truncate(false).append(false).write(false).read(true).custom_flags(0),
                0,
            )
        };
        if fd == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}