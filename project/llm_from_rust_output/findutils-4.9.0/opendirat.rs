use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, DirBuilderExt};
use std::io;
use libc::{O_DIRECTORY, O_RDONLY, O_CLOEXEC, O_NOCTTY, O_NOFOLLOW};

pub fn opendirat(
    dir_fd: RawFd,
    dir: &std::path::Path,
    extra_flags: i32,
) -> io::Result<(std::fs::File, RawFd)> {
    let mut open_flags = O_DIRECTORY | O_RDONLY | O_CLOEXEC | O_NOCTTY | O_NOFOLLOW | extra_flags;
    
    let file = OpenOptions::new()
        .custom_flags(open_flags)
        .open_at(&File::open(".")?, dir)?;
    
    let new_fd = file.as_raw_fd();
    Ok((file, new_fd))
}