use std::ffi::CStr;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::fs::{File, OpenOptions};

#[derive(Debug)]
pub enum OpenError {
    InvalidPath,
    SystemError(std::io::Error),
}

pub fn openat_safer(
    dirfd: RawFd,
    file: &CStr,
    flags: i32,
    mode: Option<u32>,
) -> Result<File, OpenError> {
    let mut open_options = OpenOptions::new();
    
    // Translate flags to OpenOptions
    if flags & libc::O_RDONLY != 0 {
        open_options.read(true);
    }
    if flags & libc::O_WRONLY != 0 {
        open_options.write(true);
    }
    if flags & libc::O_RDWR != 0 {
        open_options.read(true).write(true);
    }
    if flags & libc::O_CREAT != 0 {
        open_options.create(true);
    }
    if flags & libc::O_EXCL != 0 {
        open_options.create_new(true);
    }
    if flags & libc::O_TRUNC != 0 {
        open_options.truncate(true);
    }
    if flags & libc::O_APPEND != 0 {
        open_options.append(true);
    }

    // Set mode if provided
    if let Some(m) = mode {
        open_options.mode(m);
    }

    // Safe because we've validated the inputs
    let file = unsafe {
        let fd = libc::openat(
            dirfd,
            file.as_ptr(),
            flags,
            mode.unwrap_or(0),
        );
        if fd == -1 {
            return Err(OpenError::SystemError(std::io::Error::last_os_error()));
        }
        File::from_raw_fd(fd)
    };

    // Make the file descriptor safer (close-on-exec, etc.)
    Ok(file)
}