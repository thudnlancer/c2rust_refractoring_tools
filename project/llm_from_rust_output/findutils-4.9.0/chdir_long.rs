use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, FileExt};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};

const O_PATH: i32 = 0o200000;
const O_RDONLY: i32 = 0o400;
const O_DIRECTORY: i32 = 0o4000;

struct CdBuf {
    fd: Option<File>,
}

impl CdBuf {
    fn new() -> Self {
        CdBuf { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(fd) = &self.fd {
            let res = unsafe { libc::fchdir(fd.as_raw_fd()) };
            if res == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "No file descriptor"))
        }
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let c_dir = CString::new(dir.to_str().unwrap()).unwrap();
        let new_fd = unsafe {
            libc::openat(
                self.fd.as_ref().map_or(-1, |f| f.as_raw_fd()),
                c_dir.as_ptr(),
                O_PATH | O_RDONLY | O_DIRECTORY,
            )
        };

        if new_fd < 0 {
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

fn find_non_slash(s: &str) -> &str {
    s.trim_start_matches('/')
}

pub fn chdir_long(dir: &Path) -> io::Result<()> {
    if let Err(e) = std::env::set_current_dir(dir) {
        if e.raw_os_error() != Some(36) { // ENAMETOOLONG
            return Err(e);
        }
    } else {
        return Ok(());
    }

    let dir_str = dir.to_str().unwrap();
    let len = dir_str.len();
    if len == 0 || len < 4096 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path length"));
    }

    let mut cdb = CdBuf::new();
    let n_leading_slash = dir_str.chars().take_while(|&c| c == '/').count();

    if n_leading_slash == 2 {
        let path = &dir_str[3..];
        if let Some(slash_pos) = path.find('/') {
            let (first_part, rest) = path.split_at(slash_pos);
            let first_path = Path::new(&dir_str[..3 + slash_pos]);
            cdb.advance_fd(first_path)?;
            let remaining_path = find_non_slash(&rest[1..]);
            return chdir_long_impl(&mut cdb, remaining_path);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path too long"));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        let remaining_path = &dir_str[n_leading_slash..];
        return chdir_long_impl(&mut cdb, remaining_path);
    } else {
        return chdir_long_impl(&mut cdb, dir_str);
    }
}

fn chdir_long_impl(cdb: &mut CdBuf, dir: &str) -> io::Result<()> {
    let mut remaining = dir;
    while remaining.len() >= 4096 {
        let chunk = &remaining[..4096];
        if let Some(slash_pos) = chunk.rfind('/') {
            let (first_part, rest) = chunk.split_at(slash_pos);
            cdb.advance_fd(Path::new(first_part))?;
            remaining = find_non_slash(&rest[1..]);
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path too long"));
        }
    }

    if !remaining.is_empty() {
        cdb.advance_fd(Path::new(remaining))?;
    }

    cdb.fchdir()
}