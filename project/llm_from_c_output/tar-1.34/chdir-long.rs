use std::env;
use std::ffi::{CString, OsStr, OsString};
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

fn find_non_slash(s: &[u8]) -> &[u8] {
    let mut pos = 0;
    while pos < s.len() && s[pos] == b'/' {
        pos += 1;
    }
    &s[pos..]
}

fn advance_fd(cdb: &mut CdBuf, dir: &[u8]) -> io::Result<()> {
    let dir_cstr = CString::new(dir).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid path"))?;
    
    let new_fd = unsafe {
        libc::openat(
            cdb.fd,
            dir_cstr.as_ptr(),
            libc::O_SEARCH | libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK,
        )
    };
    
    if new_fd == -1 {
        return Err(io::Error::last_os_error());
    }

    cdb.close()?;
    cdb.fd = new_fd;
    Ok(())
}

pub fn chdir_long<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    let dir = dir.as_ref();
    if let Err(e) = env::set_current_dir(dir) {
        if e.raw_os_error() != Some(libc::ENAMETOOLONG) {
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let dir_bytes = dir.as_os_str().as_bytes();
    let dir_len = dir_bytes.len();
    let mut cdb = CdBuf::new();

    if dir_len == 0 {
        return Err(io::Error::new(io::ErrorKind::NotFound, "empty path"));
    }

    let n_leading_slash = dir_bytes.iter().take_while(|&&b| b == b'/').count();

    if n_leading_slash == 2 {
        if dir_len < 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "path too short"));
        }

        if let Some(slash_pos) = dir_bytes[3..].iter().position(|&b| b == b'/') {
            let slash_pos = slash_pos + 3;
            let mut temp_path = dir_bytes.to_vec();
            temp_path[slash_pos] = 0;
            
            advance_fd(&mut cdb, &temp_path[..slash_pos])?;
            temp_path[slash_pos] = b'/';
            
            let remaining = find_non_slash(&dir_bytes[slash_pos + 1..]);
            return chdir_long_remaining(&mut cdb, remaining);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path component too long",
            ));
        }
    } else if n_leading_slash > 0 {
        advance_fd(&mut cdb, b"/")?;
        let remaining = find_non_slash(&dir_bytes[n_leading_slash..]);
        return chdir_long_remaining(&mut cdb, remaining);
    }

    chdir_long_remaining(&mut cdb, dir_bytes)
}

fn chdir_long_remaining(cdb: &mut CdBuf, remaining: &[u8]) -> io::Result<()> {
    if remaining.len() < libc::PATH_MAX as usize {
        if !remaining.is_empty() {
            advance_fd(cdb, remaining)?;
        }
        return cdb.fchdir();
    }

    let mut pos = 0;
    while pos + libc::PATH_MAX as usize <= remaining.len() {
        let search_slice = &remaining[pos..pos + libc::PATH_MAX as usize];
        if let Some(slash_pos) = search_slice.iter().rposition(|&b| b == b'/') {
            let slash_abs_pos = pos + slash_pos;
            let mut temp_path = remaining.to_vec();
            temp_path[slash_abs_pos] = 0;
            
            advance_fd(cdb, &temp_path[..slash_abs_pos])?;
            temp_path[slash_abs_pos] = b'/';
            
            pos = slash_abs_pos + 1;
            let next_part = find_non_slash(&remaining[pos..]);
            pos += next_part.as_ptr() as usize - remaining[pos..].as_ptr() as usize;
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path component too long",
            ));
        }
    }

    if pos < remaining.len() {
        advance_fd(cdb, &remaining[pos..])?;
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

        chdir_long(&long_path).unwrap();
        assert_eq!(env::current_dir().unwrap(), long_path);
    }
}