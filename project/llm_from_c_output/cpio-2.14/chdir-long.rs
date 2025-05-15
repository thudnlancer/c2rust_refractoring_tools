use std::env;
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};

const AT_FDCWD: RawFd = -100;

struct CdBuf {
    fd: RawFd,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: AT_FDCWD }
    }

    fn fchdir(&self) -> io::Result<()> {
        if unsafe { libc::fchdir(self.fd) } == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn close(&mut self) -> io::Result<()> {
        if self.fd >= 0 {
            if unsafe { libc::close(self.fd) } == -1 {
                return Err(io::Error::last_os_error());
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

fn find_non_slash(s: &str) -> &str {
    s.trim_start_matches('/')
}

fn chdir_long(dir: &str) -> io::Result<()> {
    if let Err(e) = env::set_current_dir(dir) {
        if e.raw_os_error() != Some(libc::ENAMETOOLONG) {
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let len = dir.len();
    if len == 0 {
        return Err(io::Error::from_raw_os_error(libc::ENOENT));
    }

    let path_max = match PathBuf::from(dir).as_os_str().len() {
        0 => return Err(io::Error::from_raw_os_error(libc::ENOENT)),
        n => n,
    };

    let mut cdb = CdBuf::new();
    let mut dir = dir.to_string();
    let dir_end = dir.len();
    let n_leading_slash = dir.chars().take_while(|&c| c == '/').count();

    // Handle leading slashes and //hostname/ prefix
    if n_leading_slash == 2 {
        if let Some(slash_pos) = dir[3..].find('/') {
            let slash_pos = slash_pos + 3;
            let saved_char = dir.as_bytes()[slash_pos];
            unsafe {
                let bytes = dir.as_bytes_mut();
                bytes[slash_pos] = 0;
            }
            let res = advance_fd(&mut cdb, &dir[..slash_pos]);
            unsafe {
                let bytes = dir.as_bytes_mut();
                bytes[slash_pos] = saved_char;
            }
            res?;
            dir = find_non_slash(&dir[slash_pos + 1..]).to_string();
        } else {
            return Err(io::Error::from_raw_os_error(libc::ENAMETOOLONG));
        }
    } else if n_leading_slash > 0 {
        advance_fd(&mut cdb, "/")?;
        dir = dir[n_leading_slash..].to_string();
    }

    while path_max <= dir.len() {
        let search_len = std::cmp::min(path_max, dir.len());
        if let Some(slash_pos) = dir[..search_len].rfind('/') {
            let saved_char = dir.as_bytes()[slash_pos];
            unsafe {
                let bytes = dir.as_bytes_mut();
                bytes[slash_pos] = 0;
            }
            let res = advance_fd(&mut cdb, &dir[..slash_pos]);
            unsafe {
                let bytes = dir.as_bytes_mut();
                bytes[slash_pos] = saved_char;
            }
            res?;
            dir = find_non_slash(&dir[slash_pos + 1..]).to_string();
        } else {
            return Err(io::Error::from_raw_os_error(libc::ENAMETOOLONG));
        }
    }

    if !dir.is_empty() {
        advance_fd(&mut cdb, &dir)?;
    }

    cdb.fchdir()
}

fn advance_fd(cdb: &mut CdBuf, dir: &str) -> io::Result<()> {
    let path = Path::new(dir);
    let fd = unsafe {
        let c_str = CString::new(path.as_os_str().as_bytes()).unwrap();
        libc::openat(
            cdb.fd,
            c_str.as_ptr(),
            libc::O_SEARCH | libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK,
        )
    };

    if fd == -1 {
        return Err(io::Error::last_os_error());
    }

    cdb.close()?;
    cdb.fd = fd;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_chdir_long() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().to_str().unwrap();
        assert!(chdir_long(path).is_ok());
    }
}