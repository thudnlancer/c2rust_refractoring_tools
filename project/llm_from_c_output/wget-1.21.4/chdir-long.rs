use std::env;
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};

const PATH_MAX: usize = libc::PATH_MAX as usize;

struct CdBuf {
    fd: RawFd,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf {
            fd: libc::AT_FDCWD,
        }
    }

    fn fchdir(&self) -> io::Result<()> {
        unsafe {
            if libc::fchdir(self.fd) == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    fn close(&mut self) -> io::Result<()> {
        if self.fd >= 0 {
            unsafe {
                if libc::close(self.fd) == -1 {
                    return Err(io::Error::last_os_error());
                }
            }
            self.fd = -1;
        }
        Ok(())
    }
}

impl Drop for CdBuf {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

fn advance_fd(cdb: &mut CdBuf, dir: &Path) -> io::Result<()> {
    let c_str = CString::new(dir.as_os_str().as_bytes()).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains null byte")
    })?;

    unsafe {
        let new_fd = libc::openat(
            cdb.fd,
            c_str.as_ptr(),
            libc::O_SEARCH | libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK,
        );
        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        cdb.close()?;
        cdb.fd = new_fd;
    }

    Ok(())
}

fn find_non_slash(s: &str) -> &str {
    s.trim_start_matches('/')
}

pub fn chdir_long(dir: &Path) -> io::Result<()> {
    if let Err(e) = env::set_current_dir(dir) {
        if e.raw_os_error() != Some(libc::ENAMETOOLONG) {
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let dir_str = dir.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path contains invalid UTF-8")
    })?;

    let mut cdb = CdBuf::new();
    let mut remaining_path = dir_str;
    let mut path_buf = PathBuf::new();

    // Handle leading slashes
    let n_leading_slash = dir_str.chars().take_while(|&c| c == '/').count();
    if n_leading_slash == 2 {
        let next_slash = remaining_path[2..].find('/').ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "path too long")
        })? + 2;
        let component = &remaining_path[..next_slash];
        path_buf.push(component);
        advance_fd(&mut cdb, &path_buf)?;
        remaining_path = find_non_slash(&remaining_path[next_slash..]);
        path_buf.clear();
    } else if n_leading_slash > 0 {
        path_buf.push("/");
        advance_fd(&mut cdb, &path_buf)?;
        remaining_path = find_non_slash(remaining_path);
        path_buf.clear();
    }

    // Process remaining components
    while remaining_path.len() > PATH_MAX {
        let search_len = std::cmp::min(PATH_MAX, remaining_path.len());
        let sub_path = &remaining_path[..search_len];
        let last_slash = sub_path.rfind('/').ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "path component too long")
        })?;

        let component = &remaining_path[..last_slash];
        path_buf.push(component);
        advance_fd(&mut cdb, &path_buf)?;
        remaining_path = find_non_slash(&remaining_path[last_slash..]);
        path_buf.clear();
    }

    if !remaining_path.is_empty() {
        path_buf.push(remaining_path);
        advance_fd(&mut cdb, &path_buf)?;
    }

    cdb.fchdir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_chdir_long() {
        let temp_dir = tempdir().unwrap();
        let long_path = temp_dir.path().join("a").join("b").join("c");
        fs::create_dir_all(&long_path).unwrap();

        assert!(chdir_long(&long_path).is_ok());
        assert_eq!(env::current_dir().unwrap(), long_path);
    }
}