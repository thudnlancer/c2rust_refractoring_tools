use std::env;
use std::ffi::{CString, OsStr, OsString};
use std::fs;
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};

const PATH_MAX: usize = libc::PATH_MAX as usize;

struct CdBuf {
    fd: Option<fs::File>,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(fd) = &self.fd {
            let dir = fd.try_clone()?;
            env::set_current_dir(dir.path()?)?;
        }
        Ok(())
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let new_fd = fs::File::open(dir)?;
        self.fd = Some(new_fd);
        Ok(())
    }
}

impl Drop for CdBuf {
    fn drop(&mut self) {
        if let Some(fd) = &self.fd {
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

    let dir_str = dir.as_os_str().as_bytes();
    if dir_str.is_empty() {
        return Err(io::Error::from_raw_os_error(libc::ENOENT));
    }

    let mut cdb = CdBuf::new();
    let mut pos = 0;
    let len = dir_str.len();

    // Count leading slashes
    let n_leading_slash = dir_str.iter().take_while(|&&c| c == b'/').count();

    // Handle special cases for leading slashes
    if n_leading_slash == 2 {
        if let Some(slash_pos) = dir_str[3..].iter().position(|&c| c == b'/') {
            let slash_pos = slash_pos + 3;
            let temp_path = Path::new(OsStr::from_bytes(&dir_str[..slash_pos]));
            cdb.advance_fd(temp_path)?;
            pos = slash_pos + 1;
        } else {
            return Err(io::Error::from_raw_os_error(libc::ENAMETOOLONG));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        pos = n_leading_slash;
    }

    let remaining = &dir_str[pos..];
    pos = 0;

    while remaining.len() - pos > PATH_MAX {
        let search_slice = &remaining[pos..pos + PATH_MAX];
        if let Some(slash_pos) = search_slice.iter().rposition(|&c| c == b'/') {
            let temp_path = Path::new(OsStr::from_bytes(&remaining[..pos + slash_pos]));
            cdb.advance_fd(temp_path)?;
            pos = pos + slash_pos + 1;
        } else {
            return Err(io::Error::from_raw_os_error(libc::ENAMETOOLONG));
        }
    }

    if pos < remaining.len() {
        let temp_path = Path::new(OsStr::from_bytes(&remaining[pos..]));
        cdb.advance_fd(temp_path)?;
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