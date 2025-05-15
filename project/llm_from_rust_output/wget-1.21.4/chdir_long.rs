use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::{env, io};

const O_PATH: i32 = 0o200000;
const O_RDONLY: i32 = 0o0;
const O_NOFOLLOW: i32 = 0o4000;
const O_CLOEXEC: i32 = 0o400;

struct CdBuffer {
    fd: Option<RawFd>,
}

impl CdBuffer {
    fn new() -> Self {
        CdBuffer { fd: None }
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
                O_RDONLY | O_PATH | O_NOFOLLOW | O_CLOEXEC,
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
            unsafe { libc::close(fd) };
        }
    }
}

impl Drop for CdBuffer {
    fn drop(&mut self) {
        self.close();
    }
}

fn find_non_slash(s: &[u8]) -> &[u8] {
    s.iter()
        .position(|&c| c != b'/')
        .map(|pos| &s[pos..])
        .unwrap_or_default()
}

pub fn chdir_long(path: &Path) -> io::Result<()> {
    if env::set_current_dir(path).is_ok() {
        return Ok(());
    }

    let err = io::Error::last_os_error();
    if err.raw_os_error() != Some(libc::ENAMETOOLONG) {
        return Err(err);
    }

    let path_bytes = path.as_os_str().as_bytes();
    let mut cdb = CdBuffer::new();
    let mut remaining_path = path_bytes;

    // Handle leading slashes
    let leading_slashes = path_bytes.iter().take_while(|&&c| c == b'/').count();
    match leading_slashes {
        0 => {}
        1 => {
            cdb.advance_fd(Path::new("/"))?;
            remaining_path = find_non_slash(&path_bytes[1..]);
        }
        _ => {
            // Handle double slash case (network paths)
            if leading_slashes >= 2 {
                let next_slash = path_bytes[2..]
                    .iter()
                    .position(|&c| c == b'/')
                    .ok_or_else(|| io::Error::from_raw_os_error(libc::ENAMETOOLONG))?;

                let host_part = &path_bytes[..2 + next_slash];
                cdb.advance_fd(Path::new(std::str::from_utf8(host_part).unwrap()))?;
                remaining_path = find_non_slash(&path_bytes[2 + next_slash + 1..]);
            }
        }
    }

    // Process remaining path in chunks
    while remaining_path.len() > 4096 {
        let chunk = &remaining_path[..4096];
        let last_slash = chunk
            .iter()
            .rposition(|&c| c == b'/')
            .ok_or_else(|| io::Error::from_raw_os_error(libc::ENAMETOOLONG))?;

        let dir_part = &chunk[..last_slash];
        cdb.advance_fd(Path::new(std::str::from_utf8(dir_part).unwrap()))?;
        remaining_path = find_non_slash(&chunk[last_slash + 1..]);
    }

    if !remaining_path.is_empty() {
        cdb.advance_fd(Path::new(std::str::from_utf8(remaining_path).unwrap()))?;
    }

    cdb.fchdir()?;
    Ok(())
}