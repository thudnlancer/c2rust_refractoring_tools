use std::env;
use std::ffi::CString;
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, OwnedFd};
use std::path::{Path, PathBuf};

const PATH_MAX: usize = libc::PATH_MAX as usize;

struct CdBuf {
    fd: Option<OwnedFd>,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(fd) = &self.fd {
            let res = unsafe { libc::fchdir(fd.as_raw_fd()) };
            if res == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let new_fd = unsafe {
            let dir_cstr = CString::new(dir.as_os_str().as_bytes()).unwrap();
            libc::openat(
                self.fd.as_ref().map_or(libc::AT_FDCWD, |f| f.as_raw_fd()),
                dir_cstr.as_ptr(),
                libc::O_SEARCH | libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK,
            )
        };

        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        self.fd = Some(unsafe { OwnedFd::from_raw_fd(new_fd) });
        Ok(())
    }
}

impl Drop for CdBuf {
    fn drop(&mut self) {
        if let Some(fd) = self.fd.take() {
            let _ = fd.into_raw_fd(); // Let the OS close it when the descriptor is dropped
        }
    }
}

fn find_non_slash(s: &str) -> &str {
    s.trim_start_matches('/')
}

pub fn chdir_long(dir: &str) -> io::Result<()> {
    if let Err(e) = env::set_current_dir(dir) {
        if e.raw_os_error() != Some(libc::ENAMETOOLONG) {
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let len = dir.len();
    if len == 0 || len < PATH_MAX {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid directory name"));
    }

    let mut cdb = CdBuf::new();
    let mut dir_path = PathBuf::from(dir);
    let mut dir_str = dir.to_string();

    let n_leading_slash = dir.chars().take_while(|&c| c == '/').count();

    if n_leading_slash == 2 {
        if let Some(slash_pos) = dir[3..].find('/') {
            let slash_pos = slash_pos + 3;
            let mut temp_path = dir_path.clone();
            temp_path = temp_path.join(&dir_str[..slash_pos]);
            cdb.advance_fd(&temp_path)?;
            dir_str = dir_str[slash_pos..].to_string();
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path too long"));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        dir_str = dir_str[n_leading_slash..].to_string();
    }

    while dir_str.len() >= PATH_MAX {
        let chunk = if let Some(slash_pos) = dir_str[..PATH_MAX].rfind('/') {
            slash_pos
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path component too long"));
        };

        let temp_path = PathBuf::from(&dir_str[..chunk]);
        cdb.advance_fd(&temp_path)?;
        dir_str = find_non_slash(&dir_str[chunk + 1..]).to_string();
    }

    if !dir_str.is_empty() {
        cdb.advance_fd(Path::new(&dir_str))?;
    }

    cdb.fchdir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_chdir_long() {
        let temp_dir = tempdir().unwrap();
        let path = temp_dir.path().to_str().unwrap();
        assert!(chdir_long(path).is_ok());
    }
}