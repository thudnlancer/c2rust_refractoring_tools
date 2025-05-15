use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::{env, io};

const PATH_MAX: usize = 4096;

struct CdBuf {
    fd: Option<RawFd>,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(fd) = self.fd {
            let res = unsafe { libc::fchdir(fd) };
            if res == -1 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let dir_cstr = CString::new(dir.as_os_str().as_bytes())?;
        let new_fd = unsafe {
            libc::openat(
                self.fd.unwrap_or(libc::AT_FDCWD),
                dir_cstr.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW,
            )
        };

        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        self.close();
        self.fd = Some(new_fd);
        Ok(())
    }

    fn close(&mut self) {
        if let Some(fd) = self.fd.take() {
            unsafe {
                libc::close(fd);
            }
        }
    }
}

impl Drop for CdBuf {
    fn drop(&mut self) {
        self.close();
    }
}

fn find_non_slash(s: &[u8]) -> &[u8] {
    s.iter()
        .position(|&c| c != b'/')
        .map(|pos| &s[pos..])
        .unwrap_or(&[])
}

pub fn chdir_long(dir: &Path) -> io::Result<()> {
    if let Err(e) = env::set_current_dir(dir) {
        if e.raw_os_error() != Some(libc::ENAMETOOLONG) {
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let dir_bytes = dir.as_os_str().as_bytes();
    if dir_bytes.len() <= PATH_MAX {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path length doesn't exceed PATH_MAX",
        ));
    }

    let mut cdb = CdBuf::new();
    let n_leading_slash = dir_bytes.iter().take_while(|&&c| c == b'/').count();

    if n_leading_slash == 2 {
        let after_third_slash = &dir_bytes[3..];
        if let Some(slash_pos) = after_third_slash.iter().position(|&c| c == b'/') {
            let first_part = Path::new(&std::str::from_utf8(&dir_bytes[..3 + slash_pos]).unwrap());
            cdb.advance_fd(first_part)?;
            let remaining = find_non_slash(&dir_bytes[3 + slash_pos + 1..]);
            return chdir_long_remaining(&mut cdb, remaining);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid path format",
            ));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        let remaining = &dir_bytes[n_leading_slash..];
        return chdir_long_remaining(&mut cdb, remaining);
    }

    chdir_long_remaining(&mut cdb, dir_bytes)
}

fn chdir_long_remaining(cdb: &mut CdBuf, remaining: &[u8]) -> io::Result<()> {
    let mut current = remaining;
    while current.len() > PATH_MAX {
        let search_slice = &current[..PATH_MAX];
        if let Some(slash_pos) = search_slice.iter().rposition(|&c| c == b'/') {
            let part = Path::new(std::str::from_utf8(&current[..slash_pos]).unwrap());
            cdb.advance_fd(part)?;
            current = find_non_slash(&current[slash_pos + 1..]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path segment too long",
            ));
        }
    }

    if !current.is_empty() {
        cdb.advance_fd(Path::new(std::str::from_utf8(current).unwrap()))?;
    }

    cdb.fchdir()?;
    Ok(())
}