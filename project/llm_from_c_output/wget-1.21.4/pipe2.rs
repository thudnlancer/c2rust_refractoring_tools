use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::prelude::FromRawFd;
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use libc::{self, c_int, F_GETFL, F_SETFL, F_GETFD, F_SETFD, O_NONBLOCK, FD_CLOEXEC};

#[derive(Debug)]
pub struct Pipe {
    pub read_end: File,
    pub write_end: File,
}

pub fn pipe2(flags: i32) -> io::Result<Pipe> {
    // Check the supported flags
    const SUPPORTED_FLAGS: i32 = libc::O_CLOEXEC | libc::O_NONBLOCK | libc::O_BINARY | libc::O_TEXT;
    if (flags & !SUPPORTED_FLAGS) != 0 {
        return Err(Error::new(ErrorKind::InvalidInput, "unsupported flags"));
    }

    let mut fds: [c_int; 2] = [0; 2];

    // Try to use pipe2 if available
    #[cfg(target_os = "linux")]
    {
        match unsafe { libc::pipe2(fds.as_mut_ptr(), flags) } {
            0 => {
                let read_end = unsafe { File::from_raw_fd(fds[0]) };
                let write_end = unsafe { File::from_raw_fd(fds[1]) };
                return Ok(Pipe { read_end, write_end });
            }
            -1 if Error::last_os_error().raw_os_error() != Some(libc::ENOSYS) => {
                return Err(Error::last_os_error());
            }
            _ => (),
        }
    }

    // Fall back to pipe + fcntl
    if unsafe { libc::pipe(fds.as_mut_ptr()) } == -1 {
        return Err(Error::last_os_error());
    }

    let read_end = unsafe { File::from_raw_fd(fds[0]) };
    let write_end = unsafe { File::from_raw_fd(fds[1]) };

    // Set O_NONBLOCK if requested
    if (flags & libc::O_NONBLOCK) != 0 {
        set_nonblocking(&read_end)?;
        set_nonblocking(&write_end)?;
    }

    // Set O_CLOEXEC if requested
    if (flags & libc::O_CLOEXEC) != 0 {
        set_cloexec(&read_end)?;
        set_cloexec(&write_end)?;
    }

    Ok(Pipe { read_end, write_end })
}

fn set_nonblocking(file: &File) -> io::Result<()> {
    let fd = file.as_raw_fd();
    let flags = unsafe { libc::fcntl(fd, F_GETFL) };
    if flags == -1 {
        return Err(Error::last_os_error());
    }
    if unsafe { libc::fcntl(fd, F_SETFL, flags | O_NONBLOCK) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

fn set_cloexec(file: &File) -> io::Result<()> {
    let fd = file.as_raw_fd();
    let flags = unsafe { libc::fcntl(fd, F_GETFD) };
    if flags == -1 {
        return Err(Error::last_os_error());
    }
    if unsafe { libc::fcntl(fd, F_SETFD, flags | FD_CLOEXEC) } == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}