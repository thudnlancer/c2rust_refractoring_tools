use std::env;
use std::ffi::{CString, OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

const PATH_MAX: usize = libc::PATH_MAX as usize;

struct CdBuf {
    fd: Option<File>,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(fd) = &self.fd {
            let raw_fd = fd.as_raw_fd();
            if unsafe { libc::fchdir(raw_fd) } == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let flags = libc::O_SEARCH | libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK;
        let dir_cstr = CString::new(dir.as_os_str().as_bytes())?;
        let raw_fd = match &self.fd {
            Some(fd) => fd.as_raw_fd(),
            None => libc::AT_FDCWD,
        };

        let new_fd = unsafe { libc::openat(raw_fd, dir_cstr.as_ptr(), flags) };
        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        self.fd = Some(unsafe { File::from_raw_fd(new_fd) });
        Ok(())
    }
}

impl Drop for CdBuf {
    fn drop(&mut self) {
        if let Some(fd) = self.fd.take() {
            let _ = fd.sync_all();
        }
    }
}

fn find_non_slash(s: &[u8]) -> &[u8] {
    let mut pos = 0;
    while pos < s.len() && s[pos] == b'/' {
        pos += 1;
    }
    &s[pos..]
}

pub fn chdir_long(dir: &Path) -> io::Result<()> {
    if let Err(e) = env::set_current_dir(dir) {
        if e.raw_os_error() != Some(libc::ENAMETOOLONG) {
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let dir_os = dir.as_os_str().as_bytes();
    if dir_os.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "empty directory"));
    }

    if dir_os.len() < PATH_MAX {
        return env::set_current_dir(dir);
    }

    let mut cdb = CdBuf::new();
    let mut dir_slice = dir_os;
    let n_leading_slash = dir_slice.iter().take_while(|&&c| c == b'/').count();

    if n_leading_slash == 2 {
        if let Some(slash_pos) = dir_slice[3..].iter().position(|&c| c == b'/') {
            let slash_pos = 3 + slash_pos;
            let prefix = Path::new(OsStr::from_bytes(&dir_slice[..slash_pos]));
            cdb.advance_fd(prefix)?;
            dir_slice = find_non_slash(&dir_slice[slash_pos + 1..]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path too long",
            ));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        dir_slice = find_non_slash(dir_slice);
    }

    while dir_slice.len() >= PATH_MAX {
        let search_len = PATH_MAX.min(dir_slice.len());
        if let Some(slash_pos) = dir_slice[..search_len]
            .iter()
            .rposition(|&c| c == b'/')
        {
            let prefix = Path::new(OsStr::from_bytes(&dir_slice[..slash_pos]));
            cdb.advance_fd(prefix)?;
            dir_slice = find_non_slash(&dir_slice[slash_pos + 1..]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path component too long",
            ));
        }
    }

    if !dir_slice.is_empty() {
        cdb.advance_fd(Path::new(OsStr::from_bytes(dir_slice)))?;
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