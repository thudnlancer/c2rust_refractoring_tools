use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use std::ffi::OsStr;
use std::os::unix::fs::OpenOptionsExt;
use libc::{O_RDONLY, O_WRONLY, O_RDWR, O_CREAT, O_TRUNC, O_APPEND, O_BINARY, O_EXCL, O_CLOEXEC};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::os::unix::fs::MetadataExt;
use nix::sys::stat::SFlag;

#[cfg(target_os = "windows")]
fn translate_dev_null(filename: &str) -> &str {
    if filename == "/dev/null" {
        "NUL"
    } else {
        filename
    }
}

#[cfg(not(target_os = "windows"))]
fn translate_dev_null(filename: &str) -> &str {
    filename
}

fn parse_mode(mode: &str) -> (i32, i32, Option<String>) {
    let mut open_direction = 0;
    let mut open_flags = 0;
    let mut fdopen_mode = String::new();
    let mut p = mode.chars();

    while let Some(c) = p.next() {
        match c {
            'r' => {
                open_direction = O_RDONLY;
                fdopen_mode.push(c);
            },
            'w' => {
                open_direction = O_WRONLY;
                open_flags |= O_CREAT | O_TRUNC;
                fdopen_mode.push(c);
            },
            'a' => {
                open_direction = O_WRONLY;
                open_flags |= O_CREAT | O_APPEND;
                fdopen_mode.push(c);
            },
            'b' => {
                open_flags |= O_BINARY;
                fdopen_mode.push(c);
            },
            '+' => {
                open_direction = O_RDWR;
                fdopen_mode.push(c);
            },
            'x' => {
                open_flags |= O_EXCL;
            },
            'e' => {
                open_flags |= O_CLOEXEC;
            },
            _ => {
                fdopen_mode.push(c);
                fdopen_mode.extend(p);
                break;
            }
        }
    }

    (open_direction, open_flags, Some(fdopen_mode))
}

fn rpl_fopen(filename: &str, mode: &str) -> io::Result<File> {
    let filename = translate_dev_null(filename);
    let (open_direction, open_flags, fdopen_mode) = parse_mode(mode);

    // Handle trailing slash bug
    if filename.ends_with('/') {
        if open_direction != O_RDONLY {
            return Err(io::Error::new(io::ErrorKind::IsADirectory, "is a directory"));
        }

        let file = OpenOptions::new()
            .read(true)
            .custom_flags(open_flags)
            .open(filename)?;
        
        let metadata = file.metadata()?;
        if !SFlag::from_bits_truncate(metadata.mode()).contains(SFlag::S_IFDIR) {
            return Err(io::Error::new(io::ErrorKind::NotADirectory, "not a directory"));
        }

        return Ok(file);
    }

    if open_flags & (O_EXCL | O_CLOEXEC) != 0 {
        let file = OpenOptions::new()
            .read(open_direction == O_RDONLY || open_direction == O_RDWR)
            .write(open_direction == O_WRONLY || open_direction == O_RDWR)
            .create(open_flags & O_CREAT != 0)
            .truncate(open_flags & O_TRUNC != 0)
            .append(open_flags & O_APPEND != 0)
            .custom_flags(open_flags)
            .open(filename)?;
        
        return Ok(file);
    }

    File::open(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_operations() {
        let mut tmpfile = NamedTempFile::new().unwrap();
        write!(tmpfile, "test").unwrap();
        
        let mut file = rpl_fopen(tmpfile.path().to_str().unwrap(), "r").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, "test");
    }
}