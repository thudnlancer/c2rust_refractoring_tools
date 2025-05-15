use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::prelude::*;
use std::ptr;

#[derive(Debug)]
pub enum OpenMode {
    Read,
    Write,
    Append,
    ReadWrite,
}

impl OpenMode {
    fn from_cstr(mode: &str) -> io::Result<Self> {
        match mode {
            "r" => Ok(OpenMode::Read),
            "w" => Ok(OpenMode::Write),
            "a" => Ok(OpenMode::Append),
            "r+" | "w+" | "a+" => Ok(OpenMode::ReadWrite),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid file mode",
            )),
        }
    }

    fn to_open_options(&self) -> OpenOptions {
        let mut options = OpenOptions::new();
        match self {
            OpenMode::Read => options.read(true),
            OpenMode::Write => options.write(true).create(true).truncate(true),
            OpenMode::Append => options.write(true).create(true).append(true),
            OpenMode::ReadWrite => options.read(true).write(true).create(true),
        }
    }
}

pub fn rpl_fopen(filename: &str, mode: &str) -> io::Result<File> {
    let open_mode = OpenMode::from_cstr(mode)?;
    let mut options = open_mode.to_open_options();

    // Handle GNU-specific flags
    if mode.contains('x') {
        options.create_new(true);
    }

    let file = options.open(filename)?;

    // Handle 'e' flag (O_CLOEXEC)
    if mode.contains('e') {
        unsafe {
            let fd = file.as_raw_fd();
            let flags = libc::fcntl(fd, libc::F_GETFD);
            if flags != -1 {
                libc::fcntl(fd, libc::F_SETFD, flags | libc::FD_CLOEXEC);
            }
        }
    }

    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_fopen_read() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        let result = rpl_fopen(path, "r");
        assert!(result.is_ok());
    }

    #[test]
    fn test_fopen_write() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        let result = rpl_fopen(path, "w");
        assert!(result.is_ok());
    }

    #[test]
    fn test_fopen_invalid_mode() {
        let result = rpl_fopen("test.txt", "invalid");
        assert!(result.is_err());
    }
}