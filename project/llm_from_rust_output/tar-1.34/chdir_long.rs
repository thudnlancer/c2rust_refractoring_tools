use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::{io, ptr};

const O_PATH: i32 = 0o200000;
const O_RDONLY: i32 = 0o400;
const O_NOFOLLOW: i32 = 0o4000;

struct CdBuf {
    fd: RawFd,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: -100 }
    }

    fn fchdir(&self) -> io::Result<()> {
        if unsafe { libc::fchdir(self.fd) } == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let c_dir = CString::new(dir.as_os_str().as_bytes()).unwrap();
        let new_fd = unsafe {
            libc::openat(
                self.fd,
                c_dir.as_ptr(),
                O_PATH | O_RDONLY | O_NOFOLLOW,
            )
        };

        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        if self.fd >= 0 {
            unsafe { libc::close(self.fd) };
        }
        self.fd = new_fd;
        Ok(())
    }
}

impl Drop for CdBuf {
    fn drop(&mut self) {
        if self.fd >= 0 {
            unsafe { libc::close(self.fd) };
        }
    }
}

fn find_non_slash(s: &[u8]) -> &[u8] {
    s.iter()
        .position(|&c| c != b'/')
        .map(|pos| &s[pos..])
        .unwrap_or(&[])
}

pub fn chdir_long(dir: &Path) -> io::Result<()> {
    if let Err(e) = std::env::set_current_dir(dir) {
        if e.raw_os_error() != Some(36) { // ENAMETOOLONG
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let dir_str = dir.as_os_str().as_bytes();
    let len = dir_str.len();
    assert!(len > 0 && len >= 4096, "Path length must be >= 4096");

    let mut cdb = CdBuf::new();
    let n_leading_slash = dir_str.iter().take_while(|&&c| c == b'/').count();

    if n_leading_slash == 2 {
        let after_prefix = &dir_str[3..];
        if let Some(slash_pos) = after_prefix.iter().position(|&c| c == b'/') {
            let temp_path = Path::new(unsafe {
                std::ffi::OsStr::from_bytes(&dir_str[..3 + slash_pos])
            });
            cdb.advance_fd(temp_path)?;
            let remaining = find_non_slash(&after_prefix[slash_pos + 1..]);
            return chdir_long_impl(&mut cdb, remaining);
        } else {
            return Err(io::Error::from_raw_os_error(36));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        let remaining = &dir_str[n_leading_slash..];
        return chdir_long_impl(&mut cdb, remaining);
    }

    chdir_long_impl(&mut cdb, dir_str)
}

fn chdir_long_impl(cdb: &mut CdBuf, path: &[u8]) -> io::Result<()> {
    let mut remaining = path;
    while remaining.len() >= 4096 {
        let chunk = &remaining[..4096];
        if let Some(slash_pos) = chunk.iter().rposition(|&c| c == b'/') {
            let temp_path = Path::new(unsafe {
                std::ffi::OsStr::from_bytes(&remaining[..slash_pos])
            });
            cdb.advance_fd(temp_path)?;
            remaining = find_non_slash(&remaining[slash_pos + 1..]);
        } else {
            return Err(io::Error::from_raw_os_error(36));
        }
    }

    if !remaining.is_empty() {
        cdb.advance_fd(Path::new(unsafe {
            std::ffi::OsStr::from_bytes(remaining)
        }))?;
    }

    cdb.fchdir()
}