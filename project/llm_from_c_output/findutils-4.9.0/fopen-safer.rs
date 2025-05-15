use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::io::{self, Error, ErrorKind};

/// Like File::open, but do not return stdin, stdout, or stderr.
pub fn fopen_safer(file: &str, mode: &str) -> io::Result<File> {
    let mut open_options = OpenOptions::new();
    
    match mode {
        "r" => open_options.read(true),
        "w" => open_options.write(true).truncate(true).create(true),
        "a" => open_options.append(true).create(true),
        "r+" => open_options.read(true).write(true),
        "w+" => open_options.read(true).write(true).truncate(true).create(true),
        "a+" => open_options.read(true).append(true).create(true),
        _ => return Err(Error::new(ErrorKind::InvalidInput, "invalid mode")),
    };
    
    let fp = open_options.open(file)?;
    let fd = fp.as_raw_fd();

    if fd >= 0 && fd <= libc::STDERR_FILENO as i32 {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd < 0 {
            let e = Error::last_os_error();
            drop(fp);
            return Err(e);
        }

        let new_fd = unsafe { libc::fcntl(new_fd, libc::F_DUPFD_CLOEXEC, libc::STDERR_FILENO + 1) };
        if new_fd < 0 {
            let e = Error::last_os_error();
            drop(fp);
            return Err(e);
        }

        drop(fp);
        unsafe { File::from_raw_fd(new_fd) }
    } else {
        Ok(fp)
    }
}