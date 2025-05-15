use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::MetadataExt;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

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
                open_direction = libc::O_RDONLY;
                fdopen_mode.push(c);
            }
            'w' => {
                open_direction = libc::O_WRONLY;
                open_flags |= libc::O_CREAT | libc::O_TRUNC;
                fdopen_mode.push(c);
            }
            'a' => {
                open_direction = libc::O_WRONLY;
                open_flags |= libc::O_CREAT | libc::O_APPEND;
                fdopen_mode.push(c);
            }
            'b' => {
                open_flags |= libc::O_BINARY;
                fdopen_mode.push(c);
            }
            '+' => {
                open_direction = libc::O_RDWR;
                fdopen_mode.push(c);
            }
            'x' => {
                open_flags |= libc::O_EXCL;
            }
            'e' => {
                open_flags |= libc::O_CLOEXEC;
            }
            _ => {
                fdopen_mode.push(c);
                fdopen_mode.extend(p);
                break;
            }
        }
    }

    (open_direction, open_flags, Some(fdopen_mode))
}

fn check_trailing_slash(filename: &str, open_direction: i32) -> io::Result<()> {
    if filename.ends_with('/') {
        if open_direction != libc::O_RDONLY {
            return Err(io::Error::from_raw_os_error(libc::EISDIR));
        }

        let metadata = std::fs::metadata(filename)?;
        if !metadata.is_dir() {
            return Err(io::Error::from_raw_os_error(libc::ENOTDIR));
        }
    }
    Ok(())
}

pub fn rpl_fopen(filename: &str, mode: &str) -> io::Result<File> {
    let filename = translate_dev_null(filename);
    let (open_direction, open_flags, fdopen_mode) = parse_mode(mode);

    // Check for trailing slash bug
    if let Err(e) = check_trailing_slash(filename, open_direction) {
        return Err(e);
    }

    // Handle GNU extensions
    if open_flags & (libc::O_EXCL | libc::O_CLOEXEC) != 0 {
        let mut options = OpenOptions::new();
        options.read(open_direction == libc::O_RDONLY || open_direction == libc::O_RDWR);
        options.write(open_direction == libc::O_WRONLY || open_direction == libc::O_RDWR);
        options.create(open_flags & libc::O_CREAT != 0);
        options.truncate(open_flags & libc::O_TRUNC != 0);
        options.append(open_flags & libc::O_APPEND != 0);
        options.custom_flags(open_flags);

        let file = options.open(filename)?;
        return Ok(file);
    }

    // Fall back to standard fopen
    let mut options = OpenOptions::new();
    options.read(open_direction == libc::O_RDONLY || open_direction == libc::O_RDWR);
    options.write(open_direction == libc::O_WRONLY || open_direction == libc::O_RDWR);
    options.create(open_flags & libc::O_CREAT != 0);
    options.truncate(open_flags & libc::O_TRUNC != 0);
    options.append(open_flags & libc::O_APPEND != 0);
    
    options.open(filename)
}