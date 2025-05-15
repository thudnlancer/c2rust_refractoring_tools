use std::os::unix::io::{RawFd, FromRawFd, IntoRawFd};
use std::fs::File;
use std::io;

#[derive(Debug)]
pub enum PipeError {
    InvalidFlags,
    PipeCreationFailed(io::Error),
    FcntlFailed(io::Error),
}

pub fn rpl_pipe2(flags: i32) -> Result<(File, File), PipeError> {
    const O_CLOEXEC: i32 = 0o2000000;
    const O_NONBLOCK: i32 = 0o4000;
    
    if flags & !(O_CLOEXEC | O_NONBLOCK) != 0 {
        return Err(PipeError::InvalidFlags);
    }

    let (read, write) = create_pipe()?;
    
    if flags & O_NONBLOCK != 0 {
        set_non_blocking(&read)?;
        set_non_blocking(&write)?;
    }
    
    if flags & O_CLOEXEC != 0 {
        set_close_on_exec(&read)?;
        set_close_on_exec(&write)?;
    }
    
    Ok((read, write))
}

fn create_pipe() -> Result<(File, File), PipeError> {
    let mut fds = [0 as RawFd; 2];
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) == -1 {
            return Err(PipeError::PipeCreationFailed(io::Error::last_os_error()));
        }
    }
    
    let read = unsafe { File::from_raw_fd(fds[0]) };
    let write = unsafe { File::from_raw_fd(fds[1]) };
    
    Ok((read, write))
}

fn set_non_blocking(file: &File) -> Result<(), PipeError> {
    let fd = file.as_raw_fd();
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags == -1 {
        return Err(PipeError::FcntlFailed(io::Error::last_os_error()));
    }
    
    let new_flags = flags | libc::O_NONBLOCK;
    if unsafe { libc::fcntl(fd, libc::F_SETFL, new_flags) } == -1 {
        return Err(PipeError::FcntlFailed(io::Error::last_os_error()));
    }
    
    Ok(())
}

fn set_close_on_exec(file: &File) -> Result<(), PipeError> {
    let fd = file.as_raw_fd();
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags == -1 {
        return Err(PipeError::FcntlFailed(io::Error::last_os_error()));
    }
    
    let new_flags = flags | libc::FD_CLOEXEC;
    if unsafe { libc::fcntl(fd, libc::F_SETFD, new_flags) } == -1 {
        return Err(PipeError::FcntlFailed(io::Error::last_os_error()));
    }
    
    Ok(())
}